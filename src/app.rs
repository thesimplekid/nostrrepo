use crate::views::{
    explore::Explore, new_repository::NewRepository, repository::Repository, settings::Settings,
};
use dirs::home_dir;
use portan::Portan;
use std::path::PathBuf;

use anyhow::Result;

pub struct NostrRepoApp {
    pub view: View,
    pub state: State,
}

pub struct State {
    pub publish_repository_view: NewRepository,
    pub explore_view: Explore,

    pub repository_id: String,
    pub repository_view: Repository,

    pub settings_view: Settings,

    pub nostrrepo_folder: PathBuf,

    pub portan: Portan,
}

impl State {
    fn new() -> Result<State> {
        // TODO:
        let mut portan = Portan::default();
        Ok(State {
            publish_repository_view: NewRepository::default(),
            explore_view: Explore::new(&mut portan)?,
            repository_view: Repository::default(),
            repository_id: "".to_string(),

            settings_view: Settings::default(),
            nostrrepo_folder: home_dir().unwrap().join("nostrrepo"),

            portan,
        })
    }
}

#[derive(Clone)]
pub enum View {
    NewRepo,
    Repo(String),
    Explore,
    Settings,
    About,
}

impl Default for NostrRepoApp {
    fn default() -> Self {
        Self {
            view: View::Explore,
            state: State::new().unwrap(),
        }
    }
}

impl NostrRepoApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }
        let nostr_repo_folder = home_dir().unwrap().join("nostrrepo");
        portan_git::create_directory(&nostr_repo_folder).unwrap();

        Default::default()
    }
}

impl eframe::App for NostrRepoApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //    eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
            });

            if ui.button("New").clicked() {
                self.view = View::NewRepo;
            }
            if ui.button("Explore").clicked() {
                self.view = View::Explore
            }
            if ui.button("Settings").clicked() {
                self.view = View::Settings
            }
            if ui.button("About").clicked() {
                self.view = View::About
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.label("v0.1.0");
                });
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Powered by ");
                    ui.hyperlink_to("nostr", "https://github.com/nostr-protocol/nostr");
                });
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Made by ");
                    ui.hyperlink_to("thesimplekid", "https://thesimplekid.com");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.view {
                View::Settings => self
                    .state
                    .settings_view
                    .render_settings(&mut self.state.portan, ui)
                    .unwrap(),
                View::Explore => self
                    .state
                    .explore_view
                    .render_explore(&mut self.view, &mut self.state.portan, ui)
                    .unwrap(),
                View::About => (),
                View::NewRepo => self
                    .state
                    .publish_repository_view
                    .render_new_repo(
                        &mut self.state.explore_view,
                        &mut self.view,
                        &mut self.state.portan,
                        ui,
                    )
                    .unwrap(),
                View::Repo(publish_repo_event_id) => {
                    if publish_repo_event_id != &self.state.repository_id {
                        self.state.repository_view =
                            Repository::new(publish_repo_event_id, &mut self.state.portan).unwrap();
                        self.state.repository_id = publish_repo_event_id.clone();
                    }

                    self.state
                        .repository_view
                        .render_repository(&mut self.state.portan, &self.state.nostrrepo_folder, ui)
                        .unwrap();
                }
            };
        });
    }
}
