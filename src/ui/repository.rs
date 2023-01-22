use std::{fs, path::PathBuf, str::FromStr};
use url;

use eframe::epaint::Shadow;
use egui::{
    style::Margin, Button, Color32, Frame, Label, RichText, Rounding, ScrollArea, Sense, Stroke,
};

use anyhow::Result;

use crate::{
    ui::issues::{render_issues, render_new_issue, Issue, IssueState},
    ui::patch::{render_repository_patches, Patch, PatchState},
};
use portan::{
    repository::RepoInfo,
    types::{IssueInfo, PatchInfo},
    utils::truncated_npub,
    Portan,
};

#[derive(Debug, Default)]
pub struct Repository {
    repo_info: RepoInfo,
    issues: Vec<IssueInfo>,
    state: State,
    issue_state: IssueState,
    new_issue_data: IssueInfo,

    // patch_state: PatchState,
    local_repo_data: LocalRepoData,

    issue_view: Issue,
    patch_view: Patch,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LocalRepoData {
    pub git_log: Vec<String>,
    pub paste_patch: bool,
    pub commit_num: usize,
    pub patch: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum State {
    Code,
    #[default]
    Issues,
    LocalRepository,
    Patches,
}

pub const PADDING: f32 = 5.0;

impl Repository {
    pub fn new(publish_event_id: &str, portan: &mut Portan) -> Result<Self> {
        let repo_info = RepoInfo::get_info_from_id(publish_event_id, portan);
        let issues = portan.get_issues(&repo_info)?;
        Ok(Self {
            repo_info,
            issues,
            state: State::default(),
            issue_state: IssueState::default(),
            new_issue_data: IssueInfo::default(),

            patch_state: PatchState::default(),
            local_repo_data: LocalRepoData::default(),

            issue_view: Issue::default(),
            patch_view: Patch::default(),
        })
    }

    pub fn render_repository(
        &mut self,
        portan: &mut Portan,
        nostrrepo_folder: &PathBuf,
        ui: &mut eframe::egui::Ui,
    ) -> Result<()> {
        ui.label("Repo");
        let owner = match portan.db.read_name(&self.repo_info.owner_pub_key) {
            Ok(value) => value,
            Err(_) => truncated_npub(&self.repo_info.owner_pub_key)?,
        };
        let repo_slug = format!("{}/{}", owner, self.repo_info.name);
        ui.add_space(PADDING);
        if ui
            .add(Label::new(RichText::new(repo_slug).heading()).sense(Sense::click()))
            .clicked()
        {
            self.issue_state = IssueState::Issues(true);
        }

        ui.vertical_centered_justified(|ui| {
            Frame::none()
                .outer_margin(Margin::symmetric(1.0, 1.0))
                .inner_margin(Margin::symmetric(5.0, 5.0))
                .rounding(Rounding::same(10.0))
                .shadow(Shadow::small_light())
                .stroke(Stroke::new(1.0, Color32::GRAY))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(PADDING);
                        if ui
                            .add_enabled(self.state.ne(&State::Code), Button::new("Code"))
                            .clicked()
                        {
                            self.state = State::Code;
                        }
                        ui.add_space(PADDING);

                        if ui.add_enabled(true, Button::new("Issues")).clicked() {
                            self.issue_state = IssueState::Issues(true);
                            self.state = State::Issues;
                        }

                        if ui
                            .add_enabled(
                                self.state.ne(&State::LocalRepository),
                                Button::new("Local Repository"),
                            )
                            .clicked()
                        {
                            self.state = State::LocalRepository;
                        }

                        if ui.add_enabled(true, Button::new("Patches")).clicked() {
                            self.patch_state = PatchState::Patches(true);
                            self.state = State::Patches;
                        }
                    });
                });
        });
        match &self.state {
            State::Code => self.render_code(nostrrepo_folder, ui)?,
            State::Issues => match &self.issue_state {
                IssueState::Issues(_) => render_issues(&self.issues, &mut self.issue_state, ui)?,
                IssueState::NewIssue => render_new_issue(
                    &self.repo_info,
                    &mut self.issue_state,
                    &mut self.issues,
                    &mut self.new_issue_data,
                    portan,
                    ui,
                )?,
                IssueState::Issue(issue_info) => {
                    if issue_info.ne(&self.issue_view.issue_info) {
                        self.issue_view =
                            Issue::new(issue_info.clone(), self.repo_info.clone(), portan);
                    }
                    self.issue_view.render_issue(portan, ui)?;
                }
            },
            State::LocalRepository => render_local_repository(
                &mut self.local_repo_data,
                portan,
                &mut self.repo_info,
                nostrrepo_folder,
                ui,
            )?,
            State::Patches => match &self.patch_state {
                PatchState::Patches(_) => {
                    render_repository_patches(&mut self.patch_state, &self.repo_info, portan, ui)?
                }
                PatchState::Patch(patch_info) => {
                    if patch_info.ne(&self.patch_view.patch_info) {
                        self.patch_view = Patch::new(patch_info.clone(), self.repo_info.clone());
                    }
                    self.patch_view
                        .render_patch(&mut self.repo_info, nostrrepo_folder, ui)?;
                }
            },
        }
        Ok(())
    }

    fn render_code(&mut self, nostrrepo_folder: &PathBuf, ui: &mut eframe::egui::Ui) -> Result<()> {
        ui.add(Label::new(RichText::new("Code").heading()));
        ui.add(Label::new(RichText::new(
            "For now displaying code is not supported (ik lame)",
        )));
        ui.add(Label::new(RichText::new("The code can be found below")));

        if ui.button("Clone").clicked() {
            portan_git::clone_repository(
                &url::Url::from_str(&self.repo_info.git_url)?,
                nostrrepo_folder,
            )?;
        }
        ui.hyperlink(&self.repo_info.git_url);

        Ok(())
    }
}

fn render_local_repository(
    local_data: &mut LocalRepoData,
    portan: &mut Portan,
    repo_info: &mut RepoInfo,
    nostrrepo_folder: &PathBuf,
    ui: &mut eframe::egui::Ui,
) -> Result<()> {
    ScrollArea::vertical().auto_shrink([false; 2])
    .show(ui, |ui|{

    ui.label("Local repository");

    ui.horizontal(|ui| {
        if ui.add_enabled(local_data.paste_patch, Button::new("Generate a patch")).clicked() {
            local_data.paste_patch = false;
        }
        if ui.add_enabled(!local_data.paste_patch, Button::new("Paste a patch")).clicked() {
            local_data.paste_patch = true;
        }
    });

    if local_data.paste_patch {
        ui.text_edit_multiline(&mut local_data.patch);
    } else {

        // This should be a config option or something not declared here
        portan_git::create_directory(nostrrepo_folder)?;


        let path = nostrrepo_folder.join(repo_info.name.clone());
        match fs::metadata(&path) {
            Ok(_) => {
                ui.add(Label::new(RichText::new(format!("Local repo at: {}", path.display()))));
                local_data.git_log = portan_git::get_log(&path)?;
            },
            Err(_) => {
                ui.label("Looks like there is no matching local repo.\nYou may need to clone the repo");
            },
        }

        if !local_data.git_log.is_empty() {
            for (i, c) in local_data.git_log.iter().enumerate() {
                if ui
                    .selectable_label(
                        i <= local_data.commit_num,
                        RichText::new(format!("{}: {c}", i + 1)),
                    )
                    .clicked()
                {
                    local_data.commit_num = i;
                }
            }
        }

        ui.label("Enter the commit number to generate a patch");
        ui.add(Label::new(RichText::new(
            "This will generate a patch between the HEAD back to entered commit number",
        )));

        if ui.button("Generate patch").clicked() {
            local_data.patch = portan_git::generate_patch(
                &path,
                local_data.commit_num + 1,
            )?;
        }
    }
    if !local_data.patch.is_empty() {
        ui.label("Title");
        ui.text_edit_singleline(&mut local_data.title);
        ui.label("Description");
        ui.text_edit_multiline(&mut local_data.description);
        if ui.button("Publish Patch").clicked() {
            let patch_info = PatchInfo {
                id: "".to_string(),
                author: "".to_string(),
                name: local_data.title.to_string(),
                description: local_data.description.to_string(),
                patch: local_data.patch.to_string(),
            };
            portan.publish_patch(repo_info, patch_info)?;
        }

        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.label(local_data.patch.clone());
            });
    }
    });

    Ok(())
}
