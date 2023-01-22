use thiserror::Error;
use tokio::task::JoinError;

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

    #[error("Repo undefined")]
    MissingValue,

    #[error("Repo is not defined")]
    RepoUndefined,

    #[error("Nip 1 error")]
    NostrRustError(nostr_rust::nips::nip1::NIP1Error),

    #[error("Nostr Types error")]
    NostrTypesError(nostr_types::Error),

    #[error("DB Error")]
    DBError(redb::Error),

    #[error("Join error")]
    JoinError,

    #[error("Missing Nostr Client")]
    MissingNostrClient,

    #[error("Missing Database")]
    MissingDb,
}

impl From<nostr_types::Error> for Error {
    fn from(err: nostr_types::Error) -> Self {
        Self::NostrTypesError(err)
    }
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
impl From<nostr_rust::nips::nip1::NIP1Error> for Error {
    fn from(err: nostr_rust::nips::nip1::NIP1Error) -> Self {
        Self::NostrRustError(err)
    }
}

impl From<redb::Error> for Error {
    fn from(err: redb::Error) -> Self {
        Self::DBError(err)
    }
}

impl From<JoinError> for Error {
    fn from(_err: JoinError) -> Self {
        Self::JoinError
    }
}
