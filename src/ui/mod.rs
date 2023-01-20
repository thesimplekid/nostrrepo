use crate::errors::Error;
use dirs::home_dir;
use nostr_types::{Id, PublicKeyHex};
use portan::{repository::RepoInfo, types::IssueInfo};
use serde::{Deserialize, Serialize};

pub mod explore;
pub mod issue;
pub mod issues;
pub mod new_repository;
// pub mod patch;
//pub mod repository;
//pub mod settings;

pub fn run() -> Result<(), Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(NostrRepoUi::new(cc))),
    );

    Ok(())
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum Page {
    Explore,
    NewRepo,
    Repository(Id),
    Issues(bool),
    Issue(Id),
    NewIssue,
    Settings,
    About,
}

pub struct NostrRepoUi {
    page: Page,
    issue_title_text: String,
    issue_comment_text: String,

    // New repo
    new_repo_name: String,
    new_repo_description: String,
    new_repo_git_url: String,

    new_issue_comment: String,

    repository_info: Option<RepoInfo>,
    issue_info: Option<IssueInfo>,

    public_key: Option<PublicKeyHex>,
}

impl NostrRepoUi {
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

        // Default::default()

        NostrRepoUi {
            page: Page::Explore,
            issue_title_text: "".to_string(),
            issue_comment_text: "".to_string(),
            new_repo_name: "".to_string(),
            new_repo_description: "".to_string(),
            new_repo_git_url: "".to_string(),
            repository_info: None,
            issue_info: None,
            new_issue_comment: "".to_string(),
            public_key: None,
        }
    }

    fn set_page(&mut self, page: Page) {
        if self.page != page {
            self.set_page_inner(page);
        }
    }

    fn set_page_inner(&mut self, page: Page) {
        // Setting the page often requires some associated actions:
        match &page {
            // If change to a repo will have to get repo issues and such
            _ => {}
        }
        self.page = page;
    }
}

impl eframe::App for NostrRepoUi {
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
                self.set_page(Page::NewRepo);
            }
            if ui.button("Explore").clicked() {
                self.set_page(Page::Explore);
            }
            if ui.button("Settings").clicked() {
                self.set_page(Page::Settings);
            }
            if ui.button("About").clicked() {
                self.set_page(Page::About);
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

        egui::CentralPanel::default().show(ctx, |ui| match &self.page {
            Page::Explore => explore::update(self, ctx, _frame, ui),
            Page::Issues(_) => issues::update(self, ctx, _frame, ui),
            Page::Issue(_) => issue::update(self, ctx, _frame, ui),
            Page::NewRepo => new_repository::update(self, ctx, _frame, ui),
            _ => (),
        });
    }
}
