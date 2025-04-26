use eframe::egui;
use crate::node::Node;
use crate::simulator::log_panel::LogPanel;

pub struct CreateTransactionModal {
    pub recipient_node: Option<Node>,
    pub amount: u64
}

impl CreateTransactionModal {
    pub fn new() -> Self {
        Self {
            recipient_node: None,
            amount: 0
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, sender_node: &mut Node, nodes: Vec<Node>, visible: &mut bool, log_panel: &mut LogPanel) {
        egui::Window::new("Transaction")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("Select recipient node:");
                ui.add_space(5.0);
                for node in nodes.iter() {
                    let button = ui.button(node.name.clone());
                    if button.clicked() {
                        self.recipient_node = Some(node.clone());
                    }
                    if self.recipient_node.as_ref().map_or(false, |selected| selected.name == node.name) {
                        button.highlight();
                    }
                    ui.add_space(3.0);
                }
                ui.add_space(10.0);

                ui.label("Enter amount:");
                ui.add_space(5.0);
                ui.add(egui::DragValue::new(&mut self.amount).speed(1.0));

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Confirm").clicked() {
                        sender_node.create_transaction(self.recipient_node.as_ref().unwrap().wallet.address.clone(), self.amount);
                        self.add_transaction_log(sender_node.name.clone(), log_panel);
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
        self.recipient_node = None;
        self.amount = 0;
    }

    fn add_transaction_log(&mut self, sender_name: String, log_panel: &mut LogPanel) {
        log_panel.add_log(
            "Transaction Created".to_string(),
            format!("{} sent ${} to {}.\nTransaction saved to mempool", sender_name, self.amount, self.recipient_node.as_ref().unwrap().name.clone()),
            sender_name
        );
    }
}