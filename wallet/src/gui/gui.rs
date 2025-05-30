use eframe::egui;
use crate::gui::keys_modal::KeysModal;
use crate::gui::transaction_form::TransactionForm;
use crate::gui::transaction_panel::TransactionPanel;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

#[derive(Default)]
pub struct Gui {
    wallet: Option<Wallet>,
    current_transaction: Option<Transaction>,
    transaction_form: TransactionForm,
    transaction_panel: TransactionPanel,
    keys_modal: KeysModal,
    keys_modal_visible: bool,
    private_key_visible: bool
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::default()
                .show(ui, |ui| {
                    ui.set_min_width(800.0);
                    ui.heading("Your personal blockchain wallet");
                    ui.add_space(5.0);

                    if self.wallet.is_none() {
                        ui.horizontal(|ui| {
                            if ui.button("Load keys").clicked() {
                                self.keys_modal_visible = true;
                            }

                            ui.add_space(3.0);

                            if ui.button("Create new keys").clicked() {
                                self.wallet = Some(Wallet::new());
                            }
                        });
                    } else {
                        self.separator(ui);
                        self.key_details(ui);

                        self.separator(ui);
                        self.transaction_form.show(ui, &mut self.current_transaction, self.wallet.clone().unwrap());
                    }

                    self.transaction_panel.show(ctx, &self.current_transaction);

                    if self.keys_modal_visible {
                        self.keys_modal.show(ctx, &mut self.keys_modal_visible, &mut self.wallet);
                    }
                });
        });
    }
}

impl Gui {
    fn key_details(&mut self, ui: &mut egui::Ui) {
        let wallet_clone = self.wallet.clone().unwrap();
        ui.label(format!("Public key: {}", wallet_clone.get_public_key()));
        ui.add_space(3.0);
        ui.label(format!("Address: {}", wallet_clone.address));
        ui.add_space(3.0);
        self.private_key_button(ui);
    }

    fn private_key_button(&mut self, ui: &mut egui::Ui) {
        let btn_text = if self.private_key_visible { "Hide Private Key" } else { "Reveal Private Key" };
        ui.horizontal(|ui| {
            if ui.button(btn_text).clicked() {
                self.private_key_visible = !self.private_key_visible;
            }

            if self.private_key_visible {
                ui.label(format!("Private key: {}", self.wallet.clone().unwrap().get_private_key()));
            }
        });
    }

    fn separator(&mut self, ui: &mut egui::Ui) {
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);
    }
}