use eframe::egui;
use reqwest::blocking::Response;
use serde_json::json;
use crate::transaction::Transaction;

pub struct TransactionPanel {
    transaction_response: Option<String>
}

impl Default for TransactionPanel {
    fn default() -> Self {
        Self {
            transaction_response: None
        }
    }
}

impl TransactionPanel {
    pub fn show(&mut self, ctx: &egui::Context, current_transaction: &Option<Transaction>, request_endpoint: &str) {
        let json = match current_transaction {
            Some(tx) => {
                let signature = tx.signature.unwrap().to_string();
                let json_value = json!({
                    "sender_public_key": tx.sender,
                    "recipient_address": tx.recipient,
                    "id": tx.id,
                    "timestamp": tx.timestamp,
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
                ui.add_space(10.0);
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    if ui.button("Send transaction").clicked() {
                        self.send_transaction(json.clone(), request_endpoint);
                    }
                    if self.transaction_response.is_some() {
                        ui.add_space(5.0);
                        ui.label(self.transaction_response.clone().unwrap());
                    }
                });
            });
    }

    fn send_transaction(&mut self, mut transaction_json: String, request_endpoint: &str) {
        let mut endpoint: String = request_endpoint.to_string();
        endpoint.push_str("/transaction");
        transaction_json += "\n";

        let response = reqwest::blocking::Client::new()
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(transaction_json)
            .send();

        let body = Self::get_body_from_response(response);
        self.transaction_response = Some(body);
    }

    fn get_body_from_response(response: reqwest::Result<Response>) -> String {
        if let Ok(resp) = response {
            return resp.text().unwrap_or_else(|err| "Error reading response body: ".to_string() + &err.to_string())
        }

        "Error sending request".to_string()
    }
}