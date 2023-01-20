use crate::globals::GLOBALS;

use super::{NostrRepoUi, Page};
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use eframe::epaint::Shadow;
use egui::{
    style::Margin, Button, Color32, Context, Label, RichText, Rounding, ScrollArea, Sense,
    Separator, Stroke, TextEdit, Ui,
};
use nostr_types::Id;
use portan::{
    repository::RepoInfo,
    types::{IssueInfo, IssueResponse, IssueStatus},
    utils::{encode_id_to_number, truncated_npub},
    Portan,
};
use serde::{Deserialize, Serialize};

pub const PADDING: f32 = 5.0;

pub(super) fn update(
    app: &mut NostrRepoUi,
    ctx: &Context,
    _frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    let open = match app.page {
        Page::Issues(s) => s,
        _ => false,
    };

    ui.horizontal(|ui| {
        if ui.add_enabled(!open, Button::new("Open Issues")).clicked() {
            app.set_page(Page::Issues(true))
        }
        if ui.add_enabled(open, Button::new("Closed Issues")).clicked() {
            app.set_page(Page::Issues(false))
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.add_space(ui.available_width() / 2.);
            if ui.button("New Issue").clicked() {
                app.set_page(Page::NewIssue)
            }
        });
    });

    let issues = GLOBALS.issues.blocking_read().issues.clone();

    let issues: Vec<&IssueInfo> = match open {
        true => issues
            .iter()
            .filter(|issue| issue.current_status.eq(&IssueStatus::Open))
            .collect(),
        false => issues
            .iter()
            .filter(|issue| issue.current_status.ne(&IssueStatus::Open))
            .collect(),
        _ => vec![],
    };

    if issues.is_empty() {
        let empty_text = match open {
            true => "There are no open issues",
            false => "There are no closed issues",
        };
        ui.add(Label::new(RichText::new(empty_text)));
    } else {
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for issue in issues {
                    let icon = match issue.current_status {
                        IssueStatus::Open => egui_extras::RetainedImage::from_svg_bytes_with_size(
                            "open.svg",
                            include_bytes!("../../assets/iconoir/open.svg"),
                            egui_extras::image::FitTo::Original,
                        )
                        .unwrap(),
                        IssueStatus::Close => egui_extras::RetainedImage::from_svg_bytes_with_size(
                            "open.svg",
                            include_bytes!("../../assets/iconoir/closed.svg"),
                            egui_extras::image::FitTo::Original,
                        )
                        .unwrap(),
                        IssueStatus::CloseCompleted => {
                            egui_extras::RetainedImage::from_svg_bytes_with_size(
                                "open.svg",
                                include_bytes!("../../assets/iconoir/completed.svg"),
                                egui_extras::image::FitTo::Original,
                            )
                            .unwrap()
                        }
                    };
                    ui.add_space(PADDING);

                    ui.horizontal(|ui| {
                        icon.show(ui);
                        if ui
                            .add(
                                Label::new(RichText::new(&issue.title).heading())
                                    .sense(Sense::click()),
                            )
                            .clicked()
                        {
                            app.set_page(Page::Issue(issue.id.unwrap()));
                        }
                        let number = encode_id_to_number(&issue.id.unwrap());

                        ui.add(Label::new(RichText::new(format!("#{}", number))))
                    });
                    ui.add_space(PADDING);

                    ui.label(&issue.content);
                    ui.add_space(PADDING);

                    ui.add(Separator::default());
                }
            });
    }
}
/*
pub fn render_new_issue(
    repo_info: &RepoInfo,
    state: &mut IssueState,
    issues: &mut Vec<IssueInfo>,
    new_issue_data: &mut IssueInfo,
    portan: &mut Portan,
    ui: &mut eframe::egui::Ui,
) -> Result<()> {
    ui.label("Title");
    ui.text_edit_singleline(&mut new_issue_data.title);

    ui.add(Separator::default());

    ui.label("Description");
    ui.text_edit_multiline(&mut new_issue_data.content);

    if ui.button("Submit Issue").clicked() {
        if let Ok(issue_info) = portan.publish_issue(repo_info, new_issue_data.clone()) {
            issues.push(issue_info);
            *state = IssueState::Issues(true);
        } else {
            // TODO: Modal with error couldn't publish
        }
    }
    Ok(())
}
*/
