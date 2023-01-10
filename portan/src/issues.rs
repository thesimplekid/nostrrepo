use crate::{
    repository::RepoInfo,
    types::{IssueComment, IssueInfo, IssueResponse, IssueStatus, StatusUpdate},
    Error, Portan,
};

use nostr_rust::{
    bech32::{to_bech32, ToBech32Kind},
    events::{Event, EventPrepare},
    req::ReqFilter,
    utils::get_timestamp,
};

impl Portan {
    pub fn publish_issue(
        &mut self,
        repo_info: &RepoInfo,
        issue_info: IssueInfo,
    ) -> Result<IssueInfo, Error> {
        let event = EventPrepare {
            pub_key: self.identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 125,
            tags: vec![vec!["e".to_string(), repo_info.id.to_string()]],
            content: serde_json::to_string(&issue_info)?,
        }
        .to_event(&self.identity, 0);

        self.nostr_client.publish_event(&event)?;
        let issue_info = self.event_to_issue_info(&event, repo_info)?;
        Ok(issue_info)
    }

    /// Gets the current status of the issue
    /// requests status events from relays and finds more recent
    /// ignores events not published by repo owner or event author
    pub fn get_issue_status(
        &mut self,
        issue_info: &IssueInfo,
        repo_info: &RepoInfo,
    ) -> Result<IssueStatus, Error> {
        let filter = ReqFilter {
            ids: None,
            authors: Some(vec![
                issue_info.author.clone(),
                repo_info.owner_pub_key.clone(),
            ]),
            kinds: Some(vec![127]),
            e: Some(vec![issue_info.id.to_string()]),
            p: None,
            since: None,
            until: None,
            limit: None,
        };

        if let Ok(mut events) = self.nostr_client.get_events_of(vec![filter]) {
            // Only keeps elemants where status is published by issue author or repo owner
            events.retain(|e| {
                e.pub_key.eq(&issue_info.author) || e.pub_key.eq(&repo_info.owner_pub_key)
            });
            events.sort_by_key(|e| e.created_at);
            if let Some(last_event) = events.last() {
                return Ok(serde_json::from_str(&last_event.content).unwrap());
            }
        }
        return Ok(IssueStatus::Open);
    }

    /// Converts a nostr event IssueInfo
    /// ```
    /// use portan::Portan;
    /// use portan::repository::RepoInfo;
    /// use portan::types::{IssueInfo, IssueStatus};
    /// use nostr_rust::events::Event;
    ///
    /// let mut portan = Portan::default();
    /// let event = Event {
    /// id: "32f0bfccca50c05062af6e37f462727a2a33d0c3237d5db56ca927af6a174a89".to_string(),
    /// pub_key: "508786081ce5b80d31aba322c36b10c6cc2d7fc71a01bf91f4c4bd84814e66ce".to_string(),
    /// created_at: 1673033095,
    /// kind: 125,
    /// tags: [
    ///    [
    ///        "e".to_string(),
    ///        "8d1e0b862ed783b09d8fe51f5a38b592df9fdf4faf1a5a3766f361fc1761d437".to_string(),
    ///    ].to_vec(),
    /// ].to_vec(),
    /// content: "{\"title\":\"This is an issue\",\"content\":\"This is the content\"}".to_string(),
    /// sig: "d69e44308acad774348231c30ac65bf0d7221eff1db83c241d55e0efa4f393e0d9843d7a00f6539b7cc5d5be991951dac656e43b0f8dd4caa4676712bf9624f6".to_string(),
    /// };
    ///
    /// let repo_info = RepoInfo {
    ///                     id: "8d1e0b862ed783b09d8fe51f5a38b592df9fdf4faf1a5a3766f361fc1761d437".to_string(),
    ///                     owner_pub_key: "508786081ce5b80d31aba322c36b10c6cc2d7fc71a01bf91f4c4bd84814e66ce".to_string(),
    ///                     name: "repo".to_string(),
    ///                     description: "".to_string(),
    ///                     git_url: "".to_string()
    ///                 };
    ///
    /// let issue_info = portan.event_to_issue_info(&event, &repo_info).unwrap();
    ///
    /// let i = IssueInfo {
    ///     id: "32f0bfccca50c05062af6e37f462727a2a33d0c3237d5db56ca927af6a174a89".to_string(),
    ///     author: "508786081ce5b80d31aba322c36b10c6cc2d7fc71a01bf91f4c4bd84814e66ce".to_string(),
    ///     timestamp: 1673033095,
    ///     title: "This is an issue".to_string(),
    ///     content: "This is the content".to_string(),
    ///     current_status: IssueStatus::Open
    /// };
    ///
    /// assert_eq!(i, issue_info);
    /// ```
    pub fn event_to_issue_info(
        &mut self,
        event: &Event,
        repo_info: &RepoInfo,
    ) -> Result<IssueInfo, Error> {
        if event.verify().is_err() {
            return Err(Error::EventInvalid);
        }

        let mut content: IssueInfo = serde_json::from_str(&event.content)?;
        content.id = event.id.clone();
        content.author = event.pub_key.clone();
        content.timestamp = event.created_at.clone();
        content.current_status = self.get_issue_status(&content, repo_info)?;
        Ok(content)
    }

    /// Gets issues from nostr relays
    pub fn get_issues(&mut self, repo_info: &RepoInfo) -> Result<Vec<IssueInfo>, Error> {
        let filter = ReqFilter {
            ids: None,
            authors: None,
            kinds: Some(vec![125]),
            e: Some(vec![repo_info.id.to_string()]),
            p: None,
            since: None,
            until: None,
            limit: None,
        };

        if let Ok(events) = self.nostr_client.get_events_of(vec![filter]) {
            if !events.is_empty() {
                let issues: Result<Vec<IssueInfo>, _> = events
                    .into_iter()
                    .map(|e| self.event_to_issue_info(&e, repo_info))
                    .collect();
                return issues;
            }
        }
        Ok(vec![])
    }

    /// Get issue comments from nostr
    pub fn get_issue_comments(
        &mut self,
        issue_id: &str,
    ) -> Result<Vec<IssueComment>, serde_json::Error> {
        let filter = ReqFilter {
            ids: None,
            authors: None,
            kinds: Some(vec![126]),
            e: Some(vec![issue_id.to_string()]),
            p: None,
            since: None,
            until: None,
            limit: None,
        };

        if let Ok(events) = self.nostr_client.get_events_of(vec![filter]) {
            if !events.is_empty() {
                let mut issues: Vec<IssueComment> = events
                    .into_iter()
                    .filter(|e| e.verify().is_ok())
                    .map(|e| IssueComment {
                        author: to_bech32(ToBech32Kind::PublicKey, &e.pub_key).unwrap(),
                        timestamp: e.created_at,
                        description: e.content,
                    })
                    .collect();
                issues.sort_by_key(|i| i.timestamp);
                return Ok(issues);
            }
        }
        Ok(vec![])
    }

    /// Get issue response from nostr relays
    /// Issue responses is a enum so that both issue comments and status updates
    /// can be in one vec
    pub fn get_issue_responses(&mut self, issue_id: &str) -> Result<Vec<IssueResponse>, Error> {
        let filter = ReqFilter {
            ids: None,
            authors: None,
            kinds: Some(vec![126, 127]),
            e: Some(vec![issue_id.to_string()]),
            p: None,
            since: None,
            until: None,
            limit: None,
        };

        if let Ok(events) = self.nostr_client.get_events_of(vec![filter]) {
            if !events.is_empty() {
                let mut events = events;
                events.sort_by_key(|e| e.created_at);

                let mut issues = vec![];
                for event in events {
                    if event.verify().is_ok() {
                        match event.kind {
                            126 => issues.push(IssueResponse::Comment(IssueComment {
                                author: to_bech32(ToBech32Kind::PublicKey, &event.pub_key).unwrap(),
                                timestamp: event.created_at,
                                description: event.content,
                            })),
                            127 => issues.push(IssueResponse::Status(StatusUpdate {
                                author: event.pub_key,
                                timestamp: event.created_at,
                                status: serde_json::from_str(&event.content).unwrap(),
                            })),
                            _ => (),
                        }
                    }
                }
                return Ok(issues);
            }
        }
        Ok(vec![])
    }

    /// Publish issue comment to nostr
    pub fn publish_issue_comment(
        &mut self,
        issue_id: &str,
        content: &str,
    ) -> Result<IssueComment, Error> {
        let event = EventPrepare {
            pub_key: self.identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 126,
            tags: vec![vec!["e".to_string(), issue_id.to_string()]],
            content: content.to_string(),
        }
        .to_event(&self.identity, 0);

        self.nostr_client.publish_event(&event)?;

        Ok(IssueComment {
            author: event.pub_key,
            timestamp: event.created_at,
            description: event.content,
        })
    }

    /// Publish close issue to nostr
    pub fn publish_close_issue(
        &mut self,
        issue_id: &str,
        comment: &str,
        completed: bool,
    ) -> Result<IssueResponse, Error> {
        let comment = comment.trim();

        if !comment.is_empty() {
            self.publish_issue_comment(issue_id, comment)?;
        }

        let content = match completed {
            true => IssueStatus::CloseCompleted,
            false => IssueStatus::Close,
        };

        let event = EventPrepare {
            pub_key: self.identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 127,
            tags: vec![vec!["e".to_string(), issue_id.to_string()]],
            content: serde_json::to_string(&content)?,
        }
        .to_event(&self.identity, 0);

        self.nostr_client.publish_event(&event)?;

        Ok(IssueResponse::Status(StatusUpdate {
            author: event.pub_key,
            timestamp: event.created_at,
            status: content,
        }))
    }

    pub fn publish_reopen_issue(
        &mut self,
        issue_id: &str,
        comment: &str,
    ) -> Result<IssueResponse, Error> {
        let comment = comment.trim();

        if !comment.is_empty() {
            self.publish_issue_comment(issue_id, comment)?;
        }

        let event = EventPrepare {
            pub_key: self.identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 127,
            tags: vec![vec!["e".to_string(), issue_id.to_string()]],
            content: serde_json::to_string(&IssueStatus::Open)?,
        }
        .to_event(&self.identity, 0);

        self.nostr_client.publish_event(&event)?;

        Ok(IssueResponse::Status(StatusUpdate {
            author: event.pub_key,
            timestamp: event.created_at,
            status: IssueStatus::Open,
        }))
    }
}
