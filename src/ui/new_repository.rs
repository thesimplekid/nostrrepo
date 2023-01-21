use super::{NostrRepoUi, Page};
use crate::comms::ToOverlordMessage;
use crate::globals::GLOBALS;
use anyhow::Result;
use egui::{Context, Label, RichText, Ui};
use portan::{repository::RepoEventContent, Portan};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NewRepository {
    repo_info: RepoEventContent,
}
/*
impl NewRepository {
    pub fn new() -> NewRepository {
        NewRepository {
            repo_info: RepoEventContent::default(),
        }
    }
}

pub fn render_new_repo(
    &mut self,
    _explore: &mut Explore,
    view: &mut View,
    portan: &mut Portan,
    ui: &mut eframe::egui::Ui,
) -> Result<()>
*/
pub(super) fn update(
    app: &mut NostrRepoUi,
    ctx: &Context,
    _frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    ui.add(Label::new(
        RichText::new("publish a new repository").heading(),
    ));

    ui.add(Label::new(RichText::new("Name")));
    app.new_repo_name = app.new_repo_name.trim().to_string();
    ui.text_edit_singleline(&mut app.new_repo_name);

    ui.add(Label::new(RichText::new("Description").strong()));
    ui.text_edit_multiline(&mut app.new_repo_description);

    ui.add(Label::new(RichText::new("Git url").strong()));
    ui.add(Label::new(RichText::new(
            "Currently, the code needs to be hosted in a repository separate from this tool\nThis isn't the best user experience and hopefully can be integrated soon.\nFor now this can be thought of as announcing the repo and its location to nostr relays.\nThis allows connected nostr clients to find the repository and respond to it with issues, etc.\n",
        )));
    app.new_repo_git_url = app.new_repo_git_url.trim().to_string();
    ui.text_edit_singleline(&mut app.new_repo_git_url);

    if ui.button("Publish").clicked() {
        println!("Publish repository clicked");
        let repo_content = RepoEventContent {
            name: app.new_repo_name.clone(),
            description: app.new_repo_description.clone(),
            git_url: app.new_repo_description.clone(),
        };
        let _ = GLOBALS
            .to_overlord
            .send(ToOverlordMessage::PublishRepository(repo_content));
        app.new_repo_name = "".to_string();
        app.new_repo_description = "".to_string();
        app.new_repo_description = "".to_string();
        // TODO: Publish
        //if let Ok(_repo_info) = portan.publish_repository(repo_info.clone()) {
        // *explore = Explore::new(portan).unwrap();
        // explore.add_repo(repo_info)?;
        app.set_page(Page::Explore);
        //}
    }
}
