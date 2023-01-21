use nostr_rust::req::ReqFilter;

use std::{collections::HashMap, hash::Hash};

use nostr_types::PublicKeyHex;

use serde_json::Value;

use dashmap::DashMap;

use crate::{db, errors::Error, globals::GLOBALS};

/// Gets petnames of pubkeys in list
pub async fn get_petnames(pubkeys: Vec<String>) -> Result<HashMap<PublicKeyHex, String>, Error> {
    let filter = ReqFilter {
        ids: None,
        authors: Some(pubkeys),
        kinds: Some(vec![0]),
        e: None,
        p: None,
        since: None,
        until: None,
        limit: None,
    };

    let mut names = HashMap::new();
    if let Ok(events) = GLOBALS
        .nostr_client
        .lock()
        .await
        .as_mut()
        .ok_or(Error::MissingNosreClient)?
        .get_events_of(vec![filter])
        .await
    {
        if !events.is_empty() {
            for event in events {
                let content: Value = serde_json::from_str(&event.content)?;

                if let Some(name) = content.get("name") {
                    names.insert(
                        PublicKeyHex(event.pub_key),
                        serde_json::from_value::<String>(name.clone())?,
                    );
                }
            }
        }
    }
    Ok(names)
}

pub async fn populate_names(pubkeys: Vec<PublicKeyHex>) -> Result<(), Error> {
    let pubkeys = pubkeys.iter().map(|k| k.to_string()).collect();
    let petnames = get_petnames(pubkeys).await?;

    for (k, p) in &petnames {
        GLOBALS.people.insert(k.clone(), p.clone());
    }

    db::add_names(petnames).await?;

    Ok(())
}
