use anyhow::Result;
use portan::Portan;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Settings {
    priv_key: String,
    show_keys: bool,
    new_relay: String,
}

impl Settings {
    pub fn render_settings(
        &mut self,
        portan: &mut Portan,
        ui: &mut eframe::egui::Ui,
    ) -> Result<()> {
        self.render_login(portan, ui)?;
        ui.add(egui::Separator::default());
        self.render_relay_settings(portan, ui)?;

        Ok(())
    }

    pub fn render_login(&mut self, portan: &mut Portan, ui: &mut eframe::egui::Ui) -> Result<()> {
        ui.label("Login");

        ui.label("Keys are not persisted and genrated new at start up so if you want to use this key agian make sure to save it");
        ui.label("If you want to login with an existing nostr private key it can be pasted below");
        ui.label("Keys are not saved, but should still be used with cation");
        ui.horizontal(|ui| {
            ui.label("Private Key: ");
            ui.text_edit_singleline(&mut self.priv_key);
        });

        if ui.button("login").clicked() {
            portan.login(&self.priv_key)?;
        }

        if ui.button("Show Keys").clicked() {
            self.show_keys = !self.show_keys;
        }

        if self.show_keys {
            let (sec_key, pub_key) = portan.get_bech32_keys()?;
            ui.horizontal(|ui| {
                ui.label(format!("Public Key: {}", pub_key));
                if ui.button("Copy").clicked() {
                    ui.output().copied_text = pub_key;
                }
            });
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label(format!("Private Key: {}", sec_key));
                if ui.button("Copy").clicked() {
                    ui.output().copied_text = sec_key;
                }
            });
        }
        Ok(())
    }

    pub fn render_relay_settings(
        &mut self,
        portan: &mut Portan,
        ui: &mut eframe::egui::Ui,
    ) -> Result<()> {
        ui.label("Relay Settings");

        let relays: Vec<String> = portan.nostr_client.relays.keys().cloned().collect();

        for (i, relay) in relays.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("{}: {}", i, relay));
                if ui.button("Remove").clicked() {
                    portan.remove_relay(relay).unwrap();
                }
            });
        }

        ui.horizontal(|ui| {
            ui.label("New relay: ");
            ui.text_edit_singleline(&mut self.new_relay);
            if ui.button("Add relay").clicked() {
                portan.add_relay(&self.new_relay).unwrap();
                self.new_relay = "".to_string();
            }
        });

        Ok(())
    }
}
