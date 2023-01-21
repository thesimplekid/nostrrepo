use dashmap::DashMap;
use lazy_static::lazy_static;
use nostr_rust::{nostr_client::Client, Identity};
use nostr_types::{Event, Id, PublicKeyHex, Url};
use portan::{repository::RepoInfo, types::IssueResponse, Portan};
use redb::{Database, ReadableTable, TableDefinition};
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};

use super::comms::{ToMinionMessage, ToOverlordMessage};
use crate::{
    issues::{IssueResponses, Issues},
    repositories::Repositories,
    //ui::issues::Issue,
};

pub struct Globals {
    pub db: Mutex<Option<Database>>,
    /// This is a broadcast channel. All Minions should listen on it.
    /// To create a receiver, just run .subscribe() on it.
    //pub to_minions: broadcast::Sender<ToMinionMessage>,

    /// This is a mpsc channel. The Overlord listens on it.
    /// To create a sender, just clone() it.
    pub to_overlord: mpsc::UnboundedSender<ToOverlordMessage>,

    /// This is ephemeral. It is filled during lazy_static initialization,
    /// and stolen away when the Overlord is created.
    pub tmp_overlord_receiver: Mutex<Option<mpsc::UnboundedReceiver<ToOverlordMessage>>>,

    pub incoming_events: RwLock<Vec<(Event, Url, Option<String>)>>,
    pub events: DashMap<Id, Event>,

    pub people: DashMap<PublicKeyHex, String>,

    // All publishes repositories
    pub repositories: Repositories,

    pub repository: RwLock<Option<RepoInfo>>,

    // Issues currently in memory
    pub issues: RwLock<Issues>,
    // pub issue: RwLock<Option<RepoInfo>>,
    // Issue comments currently in memory
    pub issue_responses: RwLock<IssueResponses>,

    // Nostr
    pub identity: Mutex<Option<Identity>>,
    pub nostr_client: Mutex<Option<Client>>,
    // pub portan: Mutex<Option<Portan>>,
}

lazy_static! {
    pub static ref GLOBALS: Globals = {
        let (to_overlord, tmp_overlord_receiver) = mpsc::unbounded_channel();
        // Setup a communications channel from the Overlord to the Minions.
        // let (to_minions, _) = broadcast::channel(256);

        Globals {
            //to_minions,
            to_overlord,
            tmp_overlord_receiver: Mutex::new(Some(tmp_overlord_receiver)),
            db: Mutex::new(None),
            incoming_events: RwLock::new(Vec::new()),
            events: DashMap::new(),

            repositories: Repositories::default(),
            repository: RwLock::new(None),

            issues: RwLock::new(Issues::default()),
            //issue: RwLock::new(None),
            issue_responses: RwLock::new(IssueResponses::default()),

            people: DashMap::new(),

            identity: Mutex::new(None),
            nostr_client: Mutex::new(None),

            //portan: Mutex::new(None),
        }
    };
}
