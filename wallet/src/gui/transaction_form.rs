use eframe::egui;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub struct TransactionForm {
    receiver_address: String,
    amount: String,
    pub endpoint: String
}

impl Default for TransactionForm {
    fn default() -> Self {
        Self {
            receiver_address: "".to_string(),
            amount: "".to_string(),
            endpoint: "http://localhost:3000".to_string()
        }
    }
}

impl TransactionForm {
    pub fn show(&mut self, ui: &mut egui::Ui, current_transaction: &mut Option<Transaction>, wallet: Wallet) {
        ui.vertical(|ui| {
            ui.heading("Create Transaction");

            ui.add_space(3.0);

            ui.label("Recipient address");
            ui.add_space(1.0);
            ui.text_edit_singleline(&mut self.receiver_address);

            ui.add_space(3.0);
            ui.label("Amount");
            ui.add_space(1.0);
            ui.text_edit_singleline(&mut self.amount);

            ui.add_space(3.0);
            if ui.button("Create").clicked() {
                let amount: u64 = self.amount.clone().parse().unwrap();
                let mut new_tx = Transaction::new(wallet.get_public_key(), self.receiver_address.clone(), amount);
                wallet.create_signature(&mut new_tx);

                *current_transaction = Some(new_tx);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(10.0);
                ui.text_edit_singleline(&mut self.endpoint);
                ui.add_space(1.0);
                ui.label("Request URL");
            });
        });
    }
}