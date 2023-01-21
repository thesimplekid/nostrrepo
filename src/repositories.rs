use dashmap::DashMap;
use nostr_rust::{events::Event, req::ReqFilter};
use nostr_types::{Id, PublicKeyHex, Url};
use portan::repository::{RepoEventContent, RepoInfo};
use tracing::event;

use crate::{db, errors::Error, globals::GLOBALS, people};

#[derive(Debug, Default)]
pub struct Repositories {
    pub repositories: DashMap<Id, RepoInfo>, //pub repo_info: RepoInfo,
                                             // issues: Vec<IssueInfo>,
                                             //state: State,
                                             // issue_state: IssueState,
                                             // new_issue_data: IssueInfo,
                                             //patch_state: PatchState,
                                             // local_repo_data: LocalRepoData,

                                             // issue_view: Issue,
                                             // patch_view: Patch,
}

pub async fn publish_repository(repo_info: RepoEventContent) -> Result<(), Error> {
    println!("Publish repository");
    let tags = vec![
        vec!["r".to_string(), repo_info.git_url],
        vec!["n".to_string(), repo_info.name],
    ];

    let event = GLOBALS.identity.lock().await.as_mut().unwrap().make_event(
        124,
        &repo_info.description,
        &tags,
        0,
    );

    GLOBALS
        .nostr_client
        .lock()
        .await
        .as_mut()
        .ok_or(Error::EventInvalid)?
        .broadcast_event(&event)
        .await?;

    let repo_info = event_to_repo_info(&event)?;
    println!("{:?}", repo_info);

    db::write_repo_info(repo_info).await?;

    Ok(())
}

pub async fn get_published_repositories(
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

    if let Ok(events) = GLOBALS
        .nostr_client
        .lock()
        .await
        .as_mut()
        .ok_or(Error::EventInvalid)?
        .get_events_of(vec![filter])
        .await
    {
        if !events.is_empty() {
            //self.get_petnames(new_keys).await?;
            let repos = events
                .iter()
                .filter_map(|event| event_to_repo_info(event).ok())
                .collect::<Vec<RepoInfo>>();

            // Iterates over repos to check if author is already in DB
            let mut new_keys = vec![];
            for r in &repos {
                if db::read_name(r.owner_pub_key.clone()).await.is_err() {
                    new_keys.push(r.owner_pub_key.clone());
                }
            }
            people::populate_names(new_keys).await?;

            return Ok(repos);
        }
    } else {
        println!("Some error is happening");
    }

    Ok(vec![])
}

pub fn event_to_repo_info(event: &Event) -> Result<RepoInfo, Error> {
    if event.verify().is_err() {
        return Err(Error::EventInvalid);
    }
    // let content: RepoEventContent = serde_json::from_str(&event.content).unwrap();
    let mut git_url: Option<String> = None;
    let mut name: Option<String> = None;

    for v in &event.tags {
        match v[0].as_str() {
            "r" => git_url = Some(v[1].clone()),
            "n" => name = Some(v[1].clone()),
            _ => (),
        }
    }

    if name.is_none() || git_url.is_none() {
        return Err(Error::RepoUndefined);
    }
    Ok(RepoInfo {
        owner_pub_key: PublicKeyHex(event.pub_key.clone()),
        id: event.id.clone(),
        name: name.unwrap(),
        description: event.content.clone(),
        git_url: Url::new(&git_url.unwrap()),
    })
}

pub async fn populate_published_repositories() -> Result<(), Error> {
    let repos = get_published_repositories(None).await.unwrap();
    for r in repos {
        GLOBALS
            .repositories
            .repositories
            .insert(Id::try_from_hex_string(&r.id)?, r);
    }

    Ok(())
}
