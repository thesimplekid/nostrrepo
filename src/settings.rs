use crate::errors::Error;
use crate::globals::GLOBALS;

use nostr_rust::{
    bech32::{from_hb_to_hex, to_bech32, ToBech32Kind},
    Identity,
};

use std::str::FromStr;

/// Add a relay
pub async fn add_relay(new_relay: &str) -> Result<(), Error> {
    Ok(GLOBALS
        .nostr_client
        .lock()
        .await
        .as_mut()
        .ok_or(Error::MissingNostrClient)?
        .add_relay(new_relay)
        .await?)
}

/// Remove a relay
pub async fn remove_relay(relay: &str) -> Result<(), Error> {
    Ok(GLOBALS
        .nostr_client
        .lock()
        .await
        .as_mut()
        .ok_or(Error::MissingNostrClient)?
        .remove_relay(relay)
        .await?)
}

pub async fn login(priv_key: &str) -> Result<(), Error> {
    let hex_key = match priv_key.starts_with("nsec") {
        true => from_hb_to_hex(ToBech32Kind::SecretKey, priv_key).unwrap(),
        false => priv_key.to_string(),
    };

    let identity = Identity::from_str(&hex_key).unwrap();

    *GLOBALS.identity.lock().await = Some(identity);
    Ok(())
}
