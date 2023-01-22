use serde::{Deserialize, Serialize};

use nostr_types::{Id, PublicKeyHex};

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq)]
pub struct IssueInfo {
    #[serde(default, skip_serializing)]
    pub id: Option<Id>,
    #[serde(default, skip_serializing)]
    pub author: Option<PublicKeyHex>,
    #[serde(default, skip_serializing)]
    pub timestamp: u64,
    pub title: String,
    pub content: String,
    #[serde(default, skip_serializing)]
    pub current_status: IssueStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueComment {
    #[serde(default, skip_serializing)]
    pub author: Option<PublicKeyHex>,
    #[serde(default, skip_serializing)]
    pub timestamp: u64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum IssueStatus {
    #[default]
    Open,
    Close,
    CloseCompleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusUpdate {
    #[serde(default, skip_serializing)]
    pub author: Option<PublicKeyHex>,
    #[serde(default, skip_serializing)]
    pub timestamp: u64,
    pub status: IssueStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueResponse {
    Comment(IssueComment),
    Status(StatusUpdate),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct PatchInfo {
    #[serde(default, skip_serializing)]
    pub id: String,
    #[serde(default, skip_serializing)]
    pub author: String,
    #[serde(default, skip_serializing)]
    pub name: String,
    pub description: String,
    pub patch: String,
    // pub status: PatchStatus
}
