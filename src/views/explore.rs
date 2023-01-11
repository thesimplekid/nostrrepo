use egui::{Label, RichText, ScrollArea, Sense, Separator};
use portan::{repository::RepoInfo, utils::truncated_npub, Portan};

use anyhow::Result;

use crate::app::View;

pub const PADDING: f32 = 5.0;

#[derive(Debug, Clone)]
pub struct Explore {
    published_repositories: Vec<Repo>,
}

#[derive(Debug, Clone)]
struct Repo {
    event_id: String,
    owner_pub_key: String,
    name: String,
    description: String,
    // likes: u32,
    // updated: date
}

impl Repo {
    pub fn new(repo_info: RepoInfo) -> Self {
        Repo {
            event_id: repo_info.id,
            owner_pub_key: repo_info.owner_pub_key.to_string(),
            name: repo_info.name,
            description: repo_info.description,
        }
    }
}

impl Explore {
    pub fn new(portan: &mut Portan) -> Result<Explore> {
        let repos = portan.get_published_repositories(None)?;
        let repos = repos.into_iter().map(Repo::new).collect();

        Ok(Explore {
            published_repositories: repos,
        })
    }

    //FIX: This doesnt really work
    pub fn add_repo(&mut self, repo_info: RepoInfo) -> Result<()> {
        let repo = Repo::new(repo_info);
        self.published_repositories.push(repo);
        Ok(())
    }

    pub fn render_explore(
        &mut self,
        view: &mut View,
        portan: &mut Portan,
        ui: &mut eframe::egui::Ui,
    ) -> Result<()> {
        if ui.button("Refresh").clicked() {
            self.published_repositories = portan
                .get_published_repositories(None)?
                .into_iter()
                .map(Repo::new)
                .collect();
        }
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for r in &self.published_repositories {
                    ui.add_space(PADDING);

                    let owner = match portan.db.read_name(&r.owner_pub_key).unwrap() {
                        Some(value) => value.clone(),
                        None => truncated_npub(&r.owner_pub_key).unwrap(),
                    };
                    let repo_slug = format!("{}/{}", owner, &r.name);

                    if ui
                        .add(Label::new(RichText::new(repo_slug).heading()).sense(Sense::click()))
                        .clicked()
                    {
                        *view = View::Repo(r.event_id.clone());
                    };
                    ui.add_space(PADDING);

                    ui.label(&r.description);

                    //if ui.button("View Repo").clicked() {
                    //    *view = View::Repo(r.event_id.clone());
                    //}
                    ui.add(Separator::default());
                }
            });
        Ok(())
    }
}
