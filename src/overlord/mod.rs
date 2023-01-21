use crate::comms::{ToMinionMessage, ToOverlordMessage};
use crate::errors::Error;
use crate::globals::GLOBALS;
use crate::repositories;

use nostr_types::{
    Event, EventKind, Id, IdHex, PreEvent, PrivateKey, PublicKey, PublicKeyHex, Tag, Unixtime, Url,
};
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::thread;
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::{select, task};

pub struct Overlord {
    //to_minions: Sender<ToMinionMessage>,
    inbox: UnboundedReceiver<ToOverlordMessage>,
    // All the minion tasks running.
    //minions: task::JoinSet<()>,
    // Map from minion task::Id to Url
    //minions_task_url: HashMap<task::Id, Url>,
}

impl Overlord {
    pub fn new(inbox: UnboundedReceiver<ToOverlordMessage>) -> Overlord {
        let to_minions = GLOBALS.to_minions.clone();
        Overlord {
            //to_minions,
            inbox,
            //minions: task::JoinSet::new(),
            //minions_task_url: HashMap::new(),
        }
    }

    pub async fn run(&mut self) {
        if let Err(e) = self.run_inner().await {
            tracing::error!("{:?}", e);
        }

        tracing::info!("Overlord signalling UI to shutdown");

        // GLOBALS.shutting_down.store(true, Ordering::Relaxed);

        tracing::info!("Overlord signalling minions to shutdown");

        // Send shutdown message to all minions (and ui)
        // If this fails, it's probably because there are no more listeners
        // so just ignore it and keep shutting down.
        /*
        let _ = self.to_minions.send(ToMinionMessage {
            target: "all".to_string(),
            payload: ToMinionPayload::Shutdown,
        });
        */

        tracing::info!("Overlord waiting for minions to all shutdown");

        // Listen on self.minions until it is empty
        //while !self.minions.is_empty() {
        //    let task_nextjoined = self.minions.join_next_with_id().await;

        //    self.handle_task_nextjoined(task_nextjoined).await;
        // }

        tracing::info!("Overlord confirms all minions have shutdown");
    }

    pub async fn run_inner(&mut self) -> Result<(), Error> {
        // Load signer from settings
        //GLOBALS.signer.write().await.load_from_settings().await;

        // FIXME - if this needs doing, it should be done dynamically as
        //         new people are encountered, not batch-style on startup.
        // Create a person record for every person seen

        //People::populate_new_people().await?;

        // FIXME - if this needs doing, it should be done dynamically as
        //         new people are encountered, not batch-style on startup.
        // Create a relay record for every relay in person_relay map (these get
        // updated from events without necessarily updating our relays list)
        //DbRelay::populate_new_relays().await?;

        // Load relays from the database
        //let all_relays = DbRelay::fetch(None).await?;

        // Store copy of all relays in globals (we use it again down below)
        /*
        for relay in all_relays.iter() {
            GLOBALS
                .relays
                .write()
                .await
                .insert(Url::new(&relay.url), relay.clone());
        }
        */

        // Load people from the database
        //GLOBALS.people.load_all_followed().await?;

        // Load latest metadata per person and update their metadata
        // This can happen in the background
        task::spawn(async move {
            /*
            if let Ok(db_events) = DbEvent::fetch_latest_metadata().await {
                for dbevent in db_events.iter() {
                    let e: Event = match serde_json::from_str(&dbevent.raw) {
                        Ok(e) => e,
                        Err(_) => {
                            tracing::error!(
                                "Bad raw event: id={}, raw={}",
                                dbevent.id,
                                dbevent.raw
                            );
                            continue;
                        }
                    };

                    // Process this metadata event to update people
                    if let Err(e) = crate::process::process_new_event(&e, false, None, None).await {
                        tracing::error!("{}", e);
                    }
                }
            }
                */
        });

        // Load feed-related events from database and process (TextNote, EventDeletion, Reaction)
        {
            /*
            let now = Unixtime::now().unwrap();
            //let feed_chunk = GLOBALS.settings.read().await.feed_chunk;
            //let then = now.0 - feed_chunk as i64;

            let cond = if GLOBALS.settings.read().await.reactions {
                format!(" (kind=1 OR kind=5 OR kind=6 OR kind=7) AND created_at > {} ORDER BY created_at ASC", then)
            } else {
                format!(
                    " (kind=1 OR kind=5 OR kind=6) AND created_at > {} ORDER BY created_at ASC",
                    then
                )
            };
            let db_events = DbEvent::fetch(Some(&cond)).await?;

            // Map db events into Events
            let mut events: Vec<Event> = Vec::with_capacity(db_events.len());
            for dbevent in db_events.iter() {
                let e = serde_json::from_str(&dbevent.raw)?;
                events.push(e);
            }

            // Process these events
            let mut count = 0;
            for event in events.iter() {
                count += 1;
                crate::process::process_new_event(event, false, None, None).await?;
            }
            tracing::info!("Loaded {} events from the database", count);
            */
        }

        'mainloop: loop {
            match self.loop_handler().await {
                Ok(keepgoing) => {
                    if !keepgoing {
                        break 'mainloop;
                    }
                }
                Err(e) => {
                    // Log them and keep looping
                    tracing::error!("{}", e);
                }
            }
        }

        Ok(())
    }

    #[allow(unused_assignments)]
    async fn loop_handler(&mut self) -> Result<bool, Error> {
        let mut keepgoing: bool = true;

        tracing::trace!("overlord looping");

        // Listen on inbox, and dying minions
        select! {
                message = self.inbox.recv() => {
                    let message = match message {
                        Some(bm) => bm,
                        None => {
                            // All senders dropped, or one of them closed.
                            return Ok(false);
                        }
                    };
                    keepgoing = self.handle_message(message).await?;
                },
        }
        Ok(keepgoing)
    }

    async fn handle_message(&mut self, message: ToOverlordMessage) -> Result<bool, Error> {
        match message {
            ToOverlordMessage::PublishIssueComment(comment, id) => {
                println!("{}", comment);
                println!("{:?}", id);
            }
            ToOverlordMessage::Shutdown => (),
            ToOverlordMessage::PublishRepository(repo_content) => {
                repositories::publish_repository(repo_content)
                    .await
                    .unwrap()
            }
            ToOverlordMessage::GetPublishedRepositories => {
                let repos = repositories::get_published_repositories(None)
                    .await
                    .unwrap();
                for r in repos {
                    GLOBALS
                        .repositories
                        .repositories
                        .insert(Id::try_from_hex_string(&r.id)?, r);
                }
            }
        }

        Ok(true)
    }
}
