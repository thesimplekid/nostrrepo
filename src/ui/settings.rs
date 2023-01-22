use crate::comms::ToOverlordMessage;
use crate::globals::GLOBALS;

use super::NostrRepoUi;
use anyhow::Result;
use egui::{Context, Label, RichText, ScrollArea, Sense, Separator, Ui};
use portan::Portan;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Settings {
    priv_key: String,
    show_keys: bool,
    new_relay: String,
}

pub(super) fn update(
    app: &mut NostrRepoUi,
    ctx: &Context,
    _frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    render_login(app, ui).unwrap();
    ui.add(egui::Separator::default());
    render_relay_settings(app, ui).unwrap();
}
pub fn render_relay_settings(app: &mut NostrRepoUi, ui: &mut eframe::egui::Ui) -> Result<()> {
    ui.label("Relay Settings");

    for (i, relay) in app.relays.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.label(format!("{}: {}", i, relay));
            if ui.button("Remove").clicked() {
                let _ = GLOBALS
                    .to_overlord
                    .send(ToOverlordMessage::RemoveRelay(relay.clone()));
            }
        });
    }

    ui.horizontal(|ui| {
        ui.label("New relay: ");
        ui.text_edit_singleline(&mut app.new_relay);
        if ui.button("Add relay").clicked() {
            let _ = GLOBALS
                .to_overlord
                .send(ToOverlordMessage::AddRelay(app.new_relay.clone()));
            app.new_relay = "".to_string();
        }
    });

    Ok(())
}

pub fn render_login(app: &mut NostrRepoUi, ui: &mut eframe::egui::Ui) -> Result<()> {
    ui.label("Login");

    ui.label("Keys are not persisted and genrated new at start up so if you want to use this key agian make sure to save it");
    ui.label("If you want to login with an existing nostr private key it can be pasted below");
    ui.label("Keys are not saved, but should still be used with cation");
    ui.horizontal(|ui| {
        ui.label("Private Key: ");
        ui.text_edit_singleline(&mut app.new_priv_key);
    });

    if ui.button("login").clicked() {
        let _ = GLOBALS
            .to_overlord
            .send(ToOverlordMessage::Login(app.new_priv_key.clone()));
    }

    if ui.button("Show Keys").clicked() {
        app.show_keys = !app.show_keys;
    }

    if app.show_keys {
        // let (sec_key, pub_key) = portan.get_bech32_keys()?;
        let pub_key = app.public_key.as_ref().unwrap();
        ui.horizontal(|ui| {
            ui.label(format!("Public Key: {}", pub_key));
            if ui.button("Copy").clicked() {
                ui.output().copied_text = pub_key.to_string();
            }
        });
        ui.add_space(5.0);

        /*

        ui.horizontal(|ui| {
            ui.label(format!("Private Key: {}", sec_key));
            if ui.button("Copy").clicked() {
                ui.output().copied_text = sec_key;
            }
        });
        */
    }
    Ok(())
}
