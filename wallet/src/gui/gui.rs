use eframe::egui;
use crate::gui::keys_modal::KeysModal;
use crate::wallet::Wallet;

#[derive(Default)]
pub struct Gui {
    wallet: Option<Wallet>,
    keys_modal: KeysModal,
    keys_modal_visible: bool
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Your personal blockchain wallet");
            ui.add_space(5.0);

            if self.wallet.is_none() {
                if ui.button("Submit keys").clicked() {
                    self.keys_modal_visible = true;
                }
            }

            if self.keys_modal_visible {
                self.keys_modal.show(ctx, &mut self.keys_modal_visible, &mut self.wallet);
            }
        });
    }
}