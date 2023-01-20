use crate::comms::ToOverlordMessage;
use crate::globals::GLOBALS;

use super::{NostrRepoUi, Page};

use chrono::{DateTime, NaiveDateTime, Utc};
use eframe::epaint::Shadow;
use egui::{
    style::Margin, Button, Color32, Context, Label, RichText, Rounding, ScrollArea, Sense,
    Separator, Stroke, TextEdit, Ui,
};

use nostr_types::PublicKeyHex;
use portan::{
    repository::RepoInfo,
    types::{IssueInfo, IssueResponse, IssueStatus},
    utils::{encode_id_to_number, truncated_npub},
    Portan,
};

pub const PADDING: f32 = 5.0;

pub(super) fn update(
    app: &mut NostrRepoUi,
    ctx: &Context,
    _frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    if let (Some(repo_info), Some(issue_info)) = (&app.repository_info, &app.issue_info) {
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(Label::new(
                        RichText::new(&issue_info.title).heading().strong(),
                    ));
                    ui.add(Label::new(RichText::new(format!(
                        "#{}",
                        encode_id_to_number(&issue_info.id.unwrap())
                    ))));
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

                        let author = match GLOBALS.people.get(&repo_info.owner_pub_key) {
                            Some(value) => value.to_string(),
                            None => truncated_npub(&repo_info.owner_pub_key).unwrap(),
                        };

                        let datetime: DateTime<Utc> = DateTime::from_utc(
                            NaiveDateTime::from_timestamp_opt(
                                issue_info.timestamp.try_into().unwrap(),
                                0,
                            )
                            .unwrap(),
                            Utc,
                        );
                        let head = format!("{author} commented on {datetime}");
                        ui.label(head);
                        ui.add(Separator::default());
                        ui.label(&issue_info.content);
                        ui.add_space(PADDING);
                    });

                let comments = GLOBALS
                    .issue_responses
                    .blocking_read()
                    .issue_responses
                    .clone();

                for response in &comments {
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
                                    let author = match GLOBALS
                                        .people
                                        .get(&comment.author.as_ref().unwrap())
                                    {
                                        Some(value) => value.to_string(),
                                        None => truncated_npub(&comment.author.as_ref().unwrap())
                                            .unwrap(),
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
                                    if status.author.eq(&issue_info.author)
                                        || status
                                            .author
                                            .as_ref()
                                            .unwrap()
                                            .eq(&repo_info.owner_pub_key)
                                    {
                                        let author = match GLOBALS
                                            .people
                                            .get(&status.author.as_ref().unwrap())
                                        {
                                            Some(value) => value.to_string(),
                                            None => {
                                                truncated_npub(&status.author.as_ref().unwrap())
                                                    .unwrap()
                                            }
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
                                        IssueStatus::Close => {
                                            egui_extras::RetainedImage::from_svg_bytes_with_size(
                                                "closed.svg",
                                                include_bytes!("../../assets/iconoir/closed.svg"),
                                                egui_extras::image::FitTo::Original,
                                            )
                                            .unwrap()
                                        }
                                        IssueStatus::CloseCompleted => {
                                            egui_extras::RetainedImage::from_svg_bytes_with_size(
                                                "completed.svg",
                                                include_bytes!(
                                                    "../../assets/iconoir/completed.svg"
                                                ),
                                                egui_extras::image::FitTo::Original,
                                            )
                                            .unwrap()
                                        }
                                        IssueStatus::Open => {
                                            egui_extras::RetainedImage::from_svg_bytes_with_size(
                                                "reopen.svg",
                                                include_bytes!("../../assets/iconoir/reopen.svg"),
                                                egui_extras::image::FitTo::Original,
                                            )
                                            .unwrap()
                                        }
                                    };
                                        ui.horizontal(|ui| {
                                            icon.show(ui);
                                            let status = match status.status {
                                                IssueStatus::Close => "Closed",
                                                IssueStatus::CloseCompleted => {
                                                    "Closed as completed"
                                                }
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
                    TextEdit::multiline(&mut app.new_issue_comment).hint_text("New Comment"),
                );

                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(!app.new_issue_comment.is_empty(), Button::new("Comment"))
                        .clicked()
                    {
                        /*
                        let comment = portan
                            .publish_issue_comment(&issue_info.id, &self.new_issue_comment)
                            .await
                            .unwrap();
                        self.comments.push(IssueResponse::Comment(comment));
                        self.new_issue_comment = "".to_string();
                        */
                        let _ = GLOBALS
                            .to_overlord
                            .send(ToOverlordMessage::PublishIssueComment(
                                app.issue_comment_text.clone(),
                                app.issue_info.as_ref().unwrap().id.unwrap(),
                            ));
                    }
                    // REVIEW: Not sure whats going on with code format
                    match issue_info.current_status {
                        IssueStatus::Open => {
                            // Shows close button to repo owner or issue author
                            if issue_info
                                .author
                                .as_ref()
                                .unwrap()
                                .eq(&app.public_key.as_ref().unwrap())
                                || repo_info
                                    .owner_pub_key
                                    .eq(&app.public_key.as_ref().unwrap())
                            {
                                let comment_text = match &app.new_issue_comment.is_empty() {
                                    true => "",
                                    false => "with comment",
                                };
                                if ui
                                    .button(format!("Close as completed {}", comment_text))
                                    .clicked()
                                {
                                    /*
                                    portan
                                        .publish_close_issue(
                                            &issue_info.id,
                                            &new_issue_comment,
                                            true,
                                        )
                                        .unwrap();
                                    self.issue_info.current_status = IssueStatus::CloseCompleted;
                                    */
                                }

                                if ui.button(format!("Close {}", comment_text)).clicked() {
                                    /*
                                    portan
                                        .publish_close_issue(
                                            &self.issue_info.id,
                                            &self.new_issue_comment,
                                            false,
                                        )
                                        .await
                                        .unwrap();
                                    self.issue_info.current_status = IssueStatus::Close;
                                    */
                                }
                            }
                        }
                        IssueStatus::Close | IssueStatus::CloseCompleted => {
                            if issue_info
                                .author
                                .as_ref()
                                .unwrap()
                                .eq(&app.public_key.as_ref().unwrap())
                                || repo_info
                                    .owner_pub_key
                                    .eq(&app.public_key.as_ref().unwrap())
                            {
                                let comment_text = match &app.new_issue_comment.is_empty() {
                                    true => "",
                                    false => "with comment",
                                };

                                if ui.button(format!("Reopen {}", comment_text)).clicked() {
                                    /*
                                        let reopen_response = portan
                                            .publish_reopen_issue(
                                                &self.issue_info.id,
                                                &self.new_issue_comment,
                                            )
                                            .unwrap();

                                        self.comments.push(reopen_response);
                                    */
                                }
                            }
                        }
                    }
                });
            });
    }
}
