use eframe::egui;
use serde_json::json;
use crate::transaction::Transaction;

pub struct TransactionPanel;

impl Default for TransactionPanel {
    fn default() -> Self {
        Self
    }
}

impl TransactionPanel {
    pub fn show(&mut self, ctx: &egui::Context, current_transaction: &Option<Transaction>) {
        let json = match current_transaction {
            Some(tx) => {
                let signature = tx.signature.unwrap().to_string();
                let json_value = json!({
                    "sender_public_key": tx.sender,
                    "recipient_address": tx.recipient,
                    "id": tx.id,
                    "amount": tx.amount,
                    "signature": signature
                });

                serde_json::to_string_pretty(&json_value).unwrap()
            }
            None => serde_json::to_string_pretty(&json!({})).unwrap(),
        };

        egui::SidePanel::right("Transaction")
            .frame(egui::Frame::default().fill(egui::Color32::WHITE))
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut json.clone())
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(10)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                        .lock_focus(true)
                );
            });
    }
}