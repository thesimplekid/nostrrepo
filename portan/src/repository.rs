use crate::{errors::Error, types::PatchInfo, utils, Portan};

use nostr_rust::req::ReqFilter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RepoEventContent {
    pub name: String,
    pub description: String,
    pub git_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct RepoInfo {
    pub id: String,
    pub owner_pub_key: String,
    pub name: String,
    pub description: String,
    pub git_url: String,
}

impl RepoInfo {
    pub fn get_info_from_id(event_id: &str, portan: &mut Portan) -> Self {
        let filter = ReqFilter {
            ids: Some(vec![event_id.to_string()]),
            authors: None,
            kinds: Some(vec![124]),
            e: None,
            p: None,
            since: None,
            until: None,
            limit: Some(1),
        };
        // TODO: check there isnt more then one
        if let Ok(event) = portan.nostr_client.get_events_of(vec![filter]) {
            let event = &event[0];

            if let Ok(rep) = utils::event_to_repo_info(event) {
                return rep;
            } else {
                return Self::default();
            }
        }
        Self::default()
    }
}

impl Portan {
    pub fn publish_repository(&mut self, repo_info: RepoEventContent) -> Result<RepoInfo, Error> {
        let tags = vec![
            vec!["r".to_string(), repo_info.git_url],
            vec!["n".to_string(), repo_info.name],
        ];

        let event = self
            .identity
            .make_event(124, &repo_info.description, &tags, 0);

        self.nostr_client.broadcast_event(&event)?;

        utils::event_to_repo_info(&event)
    }

    pub fn get_repo_info(&mut self, repo_event_id: &str) -> Result<RepoInfo, Error> {
        let filter = ReqFilter {
            ids: Some(vec![repo_event_id.to_string()]),
            authors: None,
            kinds: Some(vec![124]),
            e: None,
            p: None,
            since: None,
            until: None,
            limit: Some(1),
        };

        if let Ok(events) = self.nostr_client.get_events_of(vec![filter]) {
            if !events.is_empty() {
                let repo_info = utils::event_to_repo_info(&events.into_iter().next().unwrap());
                return repo_info;
            }
        }

        Err(Error::EventNotFound)
    }

    pub fn get_published_repositories(
        &mut self,
        authors: Option<Vec<String>>,
    ) -> Result<Vec<RepoInfo>, Error> {
        let filter = ReqFilter {
            ids: None,
            authors,
            kinds: Some(vec![124]),
            e: None,
            p: None,
            since: None,
            until: None,
            limit: None,
        };

        if let Ok(events) = self.nostr_client.get_events_of(vec![filter]) {
            if !events.is_empty() {
                let repos = events
                    .iter()
                    .filter_map(|event| utils::event_to_repo_info(event).ok())
                    .collect::<Vec<RepoInfo>>();

                // Iterates over the events to find nostr pub keys that haven't been seen
                let new_keys = events.iter().fold(vec![], |mut v, e| {
                    if let Ok(r) = &self.db.read_name(&e.pub_key) {
                        if r.is_none() {
                            v.push(e.pub_key.clone());
                        }
                    }
                    v
                });

                self.get_petnames(new_keys)?;

                return Ok(repos);
            }
        }

        Ok(vec![])
    }

    pub fn publish_patch(
        &mut self,
        repo_info: &RepoInfo,
        patch_info: PatchInfo,
    ) -> Result<(), Error> {
        let tags = vec![
            vec!["e".to_string(), repo_info.id.to_string()],
            vec!["n".to_string(), patch_info.name.clone()],
        ];

        let event = self
            .identity
            .make_event(128, &serde_json::to_string(&patch_info)?, &tags, 0);

        self.nostr_client.broadcast_event(&event)?;

        Ok(())
    }

    pub fn get_published_patches(&mut self, repo_id: &str) -> Result<Vec<PatchInfo>, Error> {
        let filter = ReqFilter {
            ids: None,
            authors: None,
            kinds: Some(vec![128]),
            e: Some(vec![repo_id.to_string()]),
            p: None,
            since: None,
            until: None,
            limit: None,
        };

        if let Ok(events) = self.nostr_client.get_events_of(vec![filter]) {
            if !events.is_empty() {
                let patches = events
                    .iter()
                    .filter_map(|e| utils::event_to_patch_info(e).ok())
                    .collect::<Vec<PatchInfo>>();
                let new_keys = events.iter().fold(vec![], |mut v, e| {
                    if let Ok(r) = &self.db.read_name(&e.pub_key) {
                        if r.is_none() {
                            v.push(e.pub_key.clone());
                        }
                    }
                    v
                });

                self.get_petnames(new_keys)?;
                return Ok(patches);
            }
        }
        Ok(vec![])
    }
}
