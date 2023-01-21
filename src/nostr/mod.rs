use crate::{errors::Error, globals::GLOBALS};
use dotenvy::{self, dotenv};
use nostr_rust::{
    bech32::{from_hb_to_hex, to_bech32, ToBech32Kind},
    keys::{get_random_secret_key, get_str_keys_from_secret},
    nostr_client::Client,
    Identity,
};
use std::str::FromStr;
use std::{env, fmt, fs};

pub async fn setup_nostr() -> Result<(), Error> {
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

    let nostr_client = Client::new(relays).await.unwrap();

    // Set global identity
    {
        let mut g_id = GLOBALS.identity.lock().await;
        *g_id = Some(identity);
    }

    // Set global nostr_client
    {
        let mut g_nc = GLOBALS.nostr_client.lock().await;
        *g_nc = Some(nostr_client);
    }

    Ok(())
}
