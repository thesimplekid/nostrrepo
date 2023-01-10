use portan::{repository::RepoInfo, types::PatchInfo, utils::encode_id_to_number, Portan};
use portan_git::save_patch;
use serde::{Deserialize, Serialize};

use egui::{Label, RichText, ScrollArea, Sense};
use std::{fs, path::Path};

use anyhow::Result;

pub const PADDING: f32 = 5.0;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Patch {
    pub repo_info: RepoInfo,
    pub patch_info: PatchInfo,
}

#[derive(Debug)]
pub enum PatchState {
    Patch(PatchInfo),
    Patches(bool),
}

impl Default for PatchState {
    fn default() -> Self {
        PatchState::Patches(true)
    }
}

impl Patch {
    pub fn new(patch_info: PatchInfo, repo_info: RepoInfo) -> Self {
        Self {
            patch_info,
            repo_info,
        }
    }

    pub fn render_patch(
        &mut self,
        repo_info: &mut RepoInfo,
        nostrrepo_path: &Path,
        ui: &mut eframe::egui::Ui,
    ) -> Result<()> {
        ui.add(Label::new(
            RichText::new(format!(
                "{} #{}",
                self.patch_info.title.clone(),
                encode_id_to_number(&self.patch_info.id)
            ))
            .heading(),
        ));
        ui.label(self.patch_info.description.clone());

        ui.add_space(PADDING);

        ui.label(self.patch_info.patch.clone());
        if ui.button("Copy Patch").clicked() {
            ui.output().copied_text = self.patch_info.patch.clone();
        }

        let path = nostrrepo_path.join(repo_info.name.clone());
        match fs::metadata(&path) {
            Ok(_) => {
                ui.label("Download the patch to the local folder");
                ui.label("for now you'll have to apply the patch manually");
                if ui.button("Save patch").clicked() {
                    save_patch(&path, &self.patch_info)?;
                    ui.add(Label::new(RichText::new(format!(
                        "Local repo at: {}",
                        path.display()
                    ))));
                }
            }
            Err(_) => {
                ui.label(
                    "Looks like there is no matching local repo.\nYou may need to clone the repo",
                );
            }
        }
        Ok(())
    }
}

pub fn render_repository_patches(
    state: &mut PatchState,
    repo_info: &RepoInfo,
    portan: &mut Portan,
    ui: &mut eframe::egui::Ui,
) -> Result<()> {
    let patches = portan.get_published_patches(&repo_info.id)?;

    if patches.is_empty() {
        ui.label("There have been no published patches");
    } else {
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for patch in patches {
                    if ui
                        .add(
                            Label::new(
                                RichText::new(format!(
                                    "{} #{}",
                                    patch.title,
                                    encode_id_to_number(&patch.id)
                                ))
                                .heading(),
                            )
                            .sense(Sense::click()),
                        )
                        .clicked()
                    {
                        *state = PatchState::Patch(patch);
                    }
                }
            });
    }

    Ok(())
}
