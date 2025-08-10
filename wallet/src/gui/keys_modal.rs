use eframe::egui;
use crate::wallet::Wallet;

pub struct KeysModal {
    pub public_key: String,
    pub private_key: String,
    pub address: String
}

impl Default for KeysModal {
    fn default() -> Self {
        Self {
            private_key: "".to_string(),
            public_key: "".to_string(),
            address: "".to_string()
        }
    }
}

impl KeysModal {
    pub fn show(&mut self, ctx: &egui::Context, visible: &mut bool, wallet: &mut Option<Wallet>) {
        egui::Window::new("Keys")
            .resizable(false)
            .collapsible(false)
            .default_width(300.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("Public Key");
                ui.add_space(1.0);
                ui.text_edit_singleline(&mut self.public_key);

                ui.add_space(5.0);

                ui.label("Private Key");
                ui.add_space(1.0);
                ui.text_edit_singleline(&mut self.private_key);

                ui.add_space(5.0);

                ui.label("Address");
                ui.add_space(1.0);
                ui.text_edit_singleline(&mut self.address);

                ui.horizontal(|ui| {
                    if ui.button("Confirm").clicked() {
                        *wallet = Some(Wallet::load(self.private_key.clone(), self.public_key.clone(), self.address.clone()));
                        self.close_modal(visible);
                    }
                    ui.add_space(5.0);
                    if ui.button("Cancel").clicked() {
                        self.close_modal(visible);
                    }
                });
            });
    }

    fn close_modal(&mut self, visible: &mut bool) {
        *visible = false;
        self.public_key.clear();
        self.private_key.clear();
        self.address.clear();
    }
}