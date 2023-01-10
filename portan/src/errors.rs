use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Publish repository error")]
    PublishRepoError,

    #[error("Serde error")]
    SerdeError(serde_json::Error),

    #[error("Nostr rust client error")]
    NostrRustClientError(nostr_rust::nostr_client::ClientError),

    #[error("Event not found")]
    EventNotFound,

    #[error("Event verification failed")]
    EventInvalid,

    #[error("Invalid key")]
    InvalidKey,
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeError(err)
    }
}

impl From<nostr_rust::nostr_client::ClientError> for Error {
    fn from(err: nostr_rust::nostr_client::ClientError) -> Self {
        Self::NostrRustClientError(err)
    }
}

impl From<nostr_rust::bech32::Bech32Error> for Error {
    fn from(_err: nostr_rust::bech32::Bech32Error) -> Self {
        Self::InvalidKey
    }
}
