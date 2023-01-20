use egui::{Context, Label, RichText, ScrollArea, Sense, Separator, Ui};
use portan::{repository::RepoInfo, utils::truncated_npub, Portan};

use crate::globals::{self, GLOBALS};

use super::NostrRepoUi;

pub const PADDING: f32 = 5.0;

#[derive(Debug, Clone)]
pub struct Repo {
    id: String,
    owner_pub_key: String,
    owner_name: Option<String>,
    name: String,
    description: String,
    // likes: u32,
    // updated: date
}

impl Repo {
    pub fn new(repo_info: RepoInfo, portan: &mut Portan) -> Self {
        let owner_name = match portan.db.read_name(&repo_info.owner_pub_key) {
            Ok(name) => Some(name),
            Err(_) => None,
        };
        Repo {
            id: repo_info.id,
            owner_pub_key: repo_info.owner_pub_key.to_string(),
            owner_name,
            name: repo_info.name,
            description: repo_info.description,
        }
    }
}

pub(super) fn update(
    app: &mut NostrRepoUi,
    ctx: &Context,
    _frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    /*
    if ui.button("Refresh").clicked() {
        self.published_repositories = portan
            .get_published_repositories(None)
            .await?
            .into_iter()
            .map(|r| Repo::new(r, portan))
            .collect();
    }
    */
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            for r in &GLOBALS.repositories.repositories {
                ui.add_space(PADDING);

                let owner = match &GLOBALS.people.get(&r.owner_pub_key) {
                    Some(name) => name.to_string(),
                    None => truncated_npub(&r.owner_pub_key).unwrap().clone(),
                };

                let repo_slug = format!("{}/{}", owner, &r.name);

                /*
                if ui
                    .add(Label::new(RichText::new(repo_slug).heading()).sense(Sense::click()))
                    .clicked()
                {
                    *v = View::Repo(r.id.clone());
                };
                */
                ui.add_space(PADDING);

                ui.label(&r.description);

                ui.add(Separator::default());
            }
        });
}
