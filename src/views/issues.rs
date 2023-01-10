use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use eframe::epaint::Shadow;
use egui::{
    style::Margin, Button, Color32, Label, RichText, Rounding, ScrollArea, Sense, Separator,
    Stroke, TextEdit,
};
use portan::{
    repository::RepoInfo,
    types::{IssueInfo, IssueResponse, IssueStatus},
    utils::{encode_id_to_number, truncated_npub},
    Portan,
};
use serde::{Deserialize, Serialize};

pub const PADDING: f32 = 5.0;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Issue {
    pub repo_info: RepoInfo,
    pub issue_info: IssueInfo,
    pub comments: Vec<IssueResponse>,

    pub new_issue_comment: String,
}

#[derive(Debug)]
pub enum IssueState {
    Issue(IssueInfo),
    Issues(bool),
    NewIssue,
}

impl Default for IssueState {
    fn default() -> Self {
        IssueState::Issues(true)
    }
}
impl Issue {
    pub fn new(issue_info: IssueInfo, repo_info: RepoInfo, portan: &mut Portan) -> Self {
        let comments = portan.get_issue_responses(&issue_info.id).unwrap();
        Self {
            issue_info,
            repo_info,
            comments,
            new_issue_comment: "".to_string(),
        }
    }
    pub fn render_issue(&mut self, portan: &mut Portan, ui: &mut eframe::egui::Ui) -> Result<()> {
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(Label::new(RichText::new(&self.issue_info.title).heading().strong()));
                    ui.add(Label::new(RichText::new(format!("#{}", encode_id_to_number(&self.issue_info.id)))));
                });
                ui.add_space(PADDING);
                ui.add(Separator::default());
                egui::Frame::none()
                    .outer_margin(Margin::symmetric(1.0, 1.0))
                    .inner_margin(Margin::symmetric(5.0, 5.0))
                    .rounding(Rounding::same(10.0))
                    .shadow(Shadow::small_light())
                    .stroke(Stroke::new(1.0, Color32::GRAY))
                    .show(ui, |ui| {
                        ui.add_space(PADDING);
                        let author = match portan.petnames.get(&self.repo_info.owner_pub_key) {
                            Some(value) => value.clone().unwrap(),
                            None => truncated_npub(&self.repo_info.owner_pub_key).unwrap(),
                        };
                        let datetime: DateTime<Utc> = DateTime::from_utc(
                            NaiveDateTime::from_timestamp_opt(
                                self.issue_info.timestamp.try_into().unwrap(),
                                0,
                            )
                            .unwrap(),
                            Utc,
                        );
                        let head = format!("{author} commented on {datetime}");
                        ui.label(head);
                        ui.add(Separator::default());
                        ui.label(&self.issue_info.content);
                        ui.add_space(PADDING);
                    });

                for response in &self.comments {
                    egui::Frame::none()
                        .outer_margin(Margin::symmetric(1.0, 1.0))
                        .inner_margin(Margin::symmetric(5.0, 5.0))
                        .rounding(Rounding::same(10.0))
                        .shadow(Shadow::small_light())
                        .stroke(Stroke::new(1.0, Color32::GRAY))
                        .show(ui, |ui| {
                            ui.add_space(PADDING);
                            match response {
                                IssueResponse::Comment(comment) => {
                                    let author = match portan.petnames.get(&comment.author) {
                                        Some(value) => value.clone().unwrap(),
                                        None => truncated_npub(&comment.author).unwrap(),
                                    };
                                    let datetime: DateTime<Utc> = DateTime::from_utc(
                                        NaiveDateTime::from_timestamp_opt(
                                            comment.timestamp.try_into().unwrap(),
                                            0,
                                        )
                                        .unwrap(),
                                        Utc,
                                    );
                                    let head = format!("{author} commented on {datetime}");
                                    ui.label(head);
                                    ui.add(Separator::default());
                                    ui.add_space(PADDING);

                                    ui.label(&comment.description);
                                }
                                IssueResponse::Status(status) => {
                                    // Ignore status that isn't from issue author or repo owner
                                    if status.author.eq(&self.issue_info.author)
                                        || status.author.eq(&self.repo_info.owner_pub_key)
                                    {
                                        let author = match portan.petnames.get(&status.author) {
                                            Some(value) => value.clone().unwrap(),
                                            None => truncated_npub(&status.author).unwrap(),
                                        };
                                        let datetime: DateTime<Utc> = DateTime::from_utc(
                                            NaiveDateTime::from_timestamp_opt(
                                                status.timestamp.try_into().unwrap(),
                                                0,
                                            )
                                            .unwrap(),
                                            Utc,
                                        );
                                        let icon = match &status.status {
                                            IssueStatus::Close => egui_extras::RetainedImage::from_svg_bytes_with_size(
                                                "closed.svg",
                                                include_bytes!("../../assets/iconoir/closed.svg"),
                                                egui_extras::image::FitTo::Original)
                                                .unwrap(),
                                            IssueStatus::CloseCompleted =>egui_extras::RetainedImage::from_svg_bytes_with_size(
                                                "completed.svg",
                                                include_bytes!("../../assets/iconoir/completed.svg"),
                                                     egui_extras::image::FitTo::Original)
                                                     .unwrap() ,
                                            IssueStatus::Open => egui_extras::RetainedImage::from_svg_bytes_with_size(
                                                "reopen.svg",
                                                include_bytes!("../../assets/iconoir/reopen.svg"),
                                                     egui_extras::image::FitTo::Original,
                                                        ).unwrap(),
                                        };
                                        ui.horizontal(|ui| {
                                        icon.show(ui);
                                        let status = match status.status {
                                            IssueStatus::Close => "Closed",
                                            IssueStatus::CloseCompleted => "Closed as completed",
                                            IssueStatus::Open => "Reopened",

                                        };
                                        ui.add(Label::new(RichText::new(format!(
                                            "{} by {} on {}",
                                            status, author, datetime,
                                        ))));

                                        });
                                    }
                                }
                            }
                        });
                    ui.add_space(10.0);
                }

                ui.add(Separator::default());
                ui.add_sized(
                    [ui.available_width(), 10.0],
                    TextEdit::multiline(&mut self.new_issue_comment).hint_text("New Comment"),
                );

                ui.horizontal(|ui| {
                    if ui.add_enabled(!self.new_issue_comment.is_empty(), Button::new("Comment")).clicked() {
                        let comment = portan
                            .publish_issue_comment(&self.issue_info.id, &self.new_issue_comment)
                            .unwrap();
                        self.comments.push(IssueResponse::Comment(comment));
                        self.new_issue_comment = "".to_string();
                    }
                    // REVIEW: Not sure whats going on with code format
                    match self.issue_info.current_status {
                        IssueStatus::Open => {
                            // Shows close button to repo owner or issue author
                            if self.issue_info.author.eq(&portan.identity.public_key_str) || self.repo_info.owner_pub_key.eq(&portan.identity.public_key_str) {
                                let comment_text = match &self.new_issue_comment.is_empty() {
                                    true => "",
                                    false => "with comment",
                                };
                            if ui
                                .button(format!("Close as completed {}", comment_text)).clicked()
                            {
                                portan
                                .publish_close_issue(
                                    &self.issue_info.id,
                                    &self.new_issue_comment,
                                    true,
                                )
                                .unwrap();
                            self.issue_info.current_status = IssueStatus::CloseCompleted;                            }

                            if ui.button(format!("Close {}", comment_text)).clicked() {
                            portan
                                .publish_close_issue(
                                    &self.issue_info.id,
                                    &self.new_issue_comment,
                                    false,
                                )
                                .unwrap();
                            self.issue_info.current_status = IssueStatus::Close;
                        }
                                }
                            },
                            IssueStatus::Close | IssueStatus::CloseCompleted => {
                                if self.issue_info.author.eq(&portan.identity.public_key_str) || self.repo_info.owner_pub_key.eq(&portan.identity.public_key_str) {
                                    let comment_text = match &self.new_issue_comment.is_empty() {
                                        true => "",
                                        false => "with comment",
                                };

                                if ui.button(format!("Reopen {}", comment_text)).clicked() {
                                    let reopen_response = portan
                                        .publish_reopen_issue(
                                    &self.issue_info.id,
                                    &self.new_issue_comment,
                                ).unwrap();

                                self.comments.push(reopen_response);
                            }
                        }
                    }
                }
            }
        );
    });
        Ok(())
    }
}

pub fn render_issues(
    issues: &[IssueInfo],
    state: &mut IssueState,
    ui: &mut eframe::egui::Ui,
) -> Result<()> {
    let open = match state {
        IssueState::Issues(s) => *s,
        _ => false,
    };

    ui.horizontal(|ui| {
        if ui.add_enabled(!open, Button::new("Open Issues")).clicked() {
            *state = IssueState::Issues(true)
        }
        if ui.add_enabled(open, Button::new("Closed Issues")).clicked() {
            *state = IssueState::Issues(false)
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.add_space(ui.available_width() / 2.);
            if ui.button("New Issue").clicked() {
                *state = IssueState::NewIssue;
            }
        });
    });

    let issues: Vec<&IssueInfo> = match state {
        IssueState::Issues(true) => issues
            .iter()
            .filter(|issue| issue.current_status.eq(&IssueStatus::Open))
            .collect(),
        IssueState::Issues(false) => issues
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
                            *state = IssueState::Issue(issue.clone());
                        }
                        let number = encode_id_to_number(&issue.id);

                        ui.add(Label::new(RichText::new(format!("#{}", number))))
                    });
                    ui.add_space(PADDING);

                    ui.label(&issue.content);
                    ui.add_space(PADDING);

                    ui.add(Separator::default());
                }
            });
    }
    Ok(())
}

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
