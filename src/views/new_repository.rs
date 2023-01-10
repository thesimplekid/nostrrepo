use crate::{app::View, views::explore::Explore};
use anyhow::Result;
use egui::{Label, RichText};
use portan::{repository::RepoEventContent, Portan};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NewRepository {
    repo_info: RepoEventContent,
}

impl NewRepository {
    pub fn new() -> NewRepository {
        NewRepository {
            repo_info: RepoEventContent::default(),
        }
    }

    pub fn render_new_repo(
        &mut self,
        explore: &mut Explore,
        view: &mut View,
        portan: &mut Portan,
        ui: &mut eframe::egui::Ui,
    ) -> Result<()> {
        let Self { repo_info } = self;

        ui.add(Label::new(
            RichText::new("publish a new repository").heading(),
        ));

        ui.add(Label::new(RichText::new("Name")));
        repo_info.name = repo_info.name.trim().to_string();
        ui.text_edit_singleline(&mut repo_info.name);

        ui.add(Label::new(RichText::new("Description").strong()));
        ui.text_edit_multiline(&mut repo_info.description);

        ui.add(Label::new(RichText::new("Git url").strong()));
        ui.add(Label::new(RichText::new(
            "Currently, the code needs to be hosted in a repository separate from this tool\nThis isn't the best user experience and hopefully can be integrated soon.\nFor now this can be thought of as announcing the repo and its location to nostr relays.\nThis allows connected nostr clients to find the repository and respond to it with issues, etc.\n",
        )));
        repo_info.git_url = repo_info.git_url.trim().to_string();
        ui.text_edit_singleline(&mut repo_info.git_url);

        if ui.button("Publish").clicked() {
            if let Ok(repo_info) = portan.publish_repository(repo_info.clone()) {
                // *explore = Explore::new(portan).unwrap();
                // explore.add_repo(repo_info)?;
                *view = View::Explore;
            }
        }

        Ok(())
    }
}
