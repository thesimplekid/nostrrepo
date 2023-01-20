use dashmap::DashMap;
use nostr_types::Id;
use portan::repository::RepoInfo;

#[derive(Debug, Default)]
pub struct Repositories {
    pub repositories: DashMap<Id, RepoInfo>, //pub repo_info: RepoInfo,
                                             // issues: Vec<IssueInfo>,
                                             //state: State,
                                             // issue_state: IssueState,
                                             // new_issue_data: IssueInfo,
                                             //patch_state: PatchState,
                                             // local_repo_data: LocalRepoData,

                                             // issue_view: Issue,
                                             // patch_view: Patch,
}
