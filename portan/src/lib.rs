pub mod database;
pub mod errors;
pub mod issues;
pub mod repository;
pub mod types;
pub mod utils;

use database::PortanDb;
use errors::Error;
use nostr_rust::req::ReqFilter;
use serde_json::Value;

use dotenvy::{self, dotenv};
use std::{env, fmt, fs};

use nostr_rust::{
    bech32::{from_hb_to_hex, to_bech32, ToBech32Kind},
    keys::{get_random_secret_key, get_str_keys_from_secret},
    nostr_client::Client as NostrClient,
    Identity,
};

use async_trait::async_trait;

use std::str::FromStr;

pub struct Portan {
    pub identity: Identity,
    pub nostr_client: NostrClient,
}

impl Portan {
    pub async fn new() -> Self {
        let path = ".env-dev";
        if fs::metadata(path).is_ok() {
            dotenvy::from_path(path).expect("Messed up dev env");
        } else {
            dotenv().expect(".env file not found");
        }

        let sec_key = match env::var("SECRET_KEY") {
            Ok(sec) => sec,
            Err(_) => {
                let sec_key = get_random_secret_key().0;
                get_str_keys_from_secret(&sec_key).0
            }
        };

        let identity = Identity::from_str(&sec_key).unwrap();

        let relays = env::var("RELAYS").unwrap();
        let relays = serde_json::from_str::<Vec<&str>>(relays.trim()).unwrap();
        let nostr_client = NostrClient::new(relays).await.unwrap();
        //nostr_client.publish_text_note(&identity, "hi", &[vec!["".to_string()]], 0);

        Portan {
            identity,
            nostr_client,
        }
    }
}

impl std::fmt::Debug for Portan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "identity pub key: {}", self.identity.public_key_str)
    }
}

impl Portan {
    /*
    pub async fn new(priv_key: &str, relay_urls: Vec<&str>) -> Result<Self, Error> {
        let identity = Identity::from_str(priv_key).unwrap();

        let nostr_client = NostrClient::new(relay_urls).await?;

        Ok(Self {
            identity,
            nostr_client,
            db: PortanDb::new(),
        })
    }
    */

    /// Login
    /// ```rust
    /// use portan::Portan;
    ///
    /// let priv_key = "a4c75131064cecdceac1275bc42310d02c5ddae643d83e075ee7941137c7e1c9";
    /// let mut portan = Portan::default();
    /// portan.login(priv_key);
    ///
    /// assert_eq!("a4c6a127b3f78f0f8531fdc1faa855fb5c21b1a6f82add4b3df4c45adff8d443", portan.identity.public_key_str);
    /// ```
    pub fn login(&mut self, priv_key: &str) -> Result<(), Error> {
        let hex_key = match priv_key.starts_with("nsec") {
            true => from_hb_to_hex(ToBech32Kind::SecretKey, priv_key).unwrap(),
            false => priv_key.to_string(),
        };

        let identity = Identity::from_str(&hex_key).unwrap();

        self.identity = identity;
        Ok(())
    }

    /// Get bech32 keys
    /// ```no_run
    /// use portan::Portan;
    ///
    /// let priv_key = "a4c75131064cecdceac1275bc42310d02c5ddae643d83e075ee7941137c7e1c9";
    ///
    /// let mut portan = Portan::new(priv_key, vec!["wss://nostr.thesimplekid.com"]).unwrap();
    /// let (privkey, pubkey) = portan.get_bech32_keys().unwrap();
    ///
    /// assert_eq!(pubkey, "npub15nr2zfan778slpf3lhql42z4ldwzrvdxlq4d6jea7nz94hlc63ps2vza9s".to_string());
    /// assert_eq!(privkey, "nsec15nr4zvgxfnkde6kpyaduggcs6qk9mkhxg0vrup67u72pzd78u8ysf9j7jd".to_string());
    /// ```
    pub fn get_bech32_keys(&mut self) -> Result<(String, String), Error> {
        let (priv_key, pub_key) = get_str_keys_from_secret(&self.identity.secret_key);
        let priv_key = to_bech32(ToBech32Kind::SecretKey, &priv_key).unwrap();
        let pub_key = to_bech32(ToBech32Kind::PublicKey, &pub_key).unwrap();

        Ok((priv_key, pub_key))
    }

    /// Add a relay
    pub async fn add_relay(&mut self, new_relay: &str) -> Result<(), Error> {
        Ok(self.nostr_client.add_relay(new_relay).await?)
    }

    /// Remove a relay
    pub async fn remove_relay(&mut self, relay: &str) -> Result<(), Error> {
        Ok(self.nostr_client.remove_relay(relay).await?)
    }

    /// Gets petnames of pubkeys in list
    pub async fn get_petnames(&mut self, pubkeys: Vec<String>) -> Result<(), Error> {
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

        if let Ok(events) = self.nostr_client.get_events_of(vec![filter]).await {
            if !events.is_empty() {
                for event in events {
                    let content: Value = serde_json::from_str(&event.content)?;
                    /*
                    if let Some(name) = content.get("name") {
                        self.db.write_name(
                            &event.pub_key,
                            &serde_json::from_value::<String>(name.clone())?,
                        )?;
                    } else {
                        // TODO: Maybe should just not add
                        //self.petnames.insert(event.pub_key, None);
                    };
                    */
                }
            }
        }
        Ok(())
    }
}
