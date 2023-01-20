use portan::types::{IssueComment, IssueInfo, IssueResponse};

#[derive(Debug, Default)]
pub struct Issues {
    pub issues: Vec<IssueInfo>,
}

#[derive(Debug, Default)]
pub struct IssueResponses {
    pub issue_responses: Vec<IssueResponse>,
}
