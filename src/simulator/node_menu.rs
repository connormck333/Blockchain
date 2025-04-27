use eframe::egui;
use uuid::Uuid;
use crate::block::Block;
use crate::network::Network;
use crate::simulator::create_transaction_modal::CreateTransactionModal;
use crate::simulator::log_panel::LogPanel;
use crate::utils::format_timestamp;

pub struct NodeMenu {
    modal_visible: bool,
    create_transaction_modal: CreateTransactionModal,
    blockchain_menu_open: bool
}

impl Default for NodeMenu {
    fn default() -> Self {
        Self {
            modal_visible: false,
            create_transaction_modal: CreateTransactionModal::new(),
            blockchain_menu_open: false
        }
    }
}

impl NodeMenu {
    pub fn show(&mut self, ctx: &egui::Context, network: &mut Network, selected_node_id: Uuid, log_panel: &mut LogPanel) {
        let node_copy = network.get_node_by_id(selected_node_id).clone();

        egui::SidePanel::left("node_menu")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading(node_copy.name.clone());
                ui.separator();

                if ui.button("Send Transaction").clicked() {
                    self.modal_visible = true;
                }

                ui.add_space(5.0);

                if ui.button("Mine block").clicked() {
                    let mined_block = network.get_node_by_id(selected_node_id).mine_block();
                    Self::add_mined_block_log(log_panel, mined_block.clone(), node_copy.name.clone());
                    network.broadcast_block(mined_block.clone(), node_copy.id.clone(), log_panel);
                }

                let margin_top = ui.available_height() - 60.0;
                ui.add_space(margin_top);
                ui.separator();

                if self.blockchain_menu_open {
                    if ui.button("Close Blockchain Menu").clicked() {
                        self.blockchain_menu_open = false;
                    }
                } else {
                    if ui.button("View Blockchain").clicked() {
                        self.blockchain_menu_open = true;
                    }
                }

                ui.label("Balance: $100");

                if self.modal_visible {
                    self.create_transaction_modal.show(ctx, network, selected_node_id, &mut self.modal_visible, log_panel);
                }
            });
    }

    pub fn is_blockchain_menu_open(&self) -> bool {
        self.blockchain_menu_open
    }

    fn add_mined_block_log(log_panel: &mut LogPanel, mined_block: Block, node_name: String) {
        log_panel.add_log(
            "Block Mined".to_string(),
            format!("Block id {} mined containing {} transactions.\nBlock hash: {}\nTime mined at: {}",
                    mined_block.index, mined_block.transactions.len(), mined_block.hash, format_timestamp(mined_block.timestamp)),
            node_name
        );
    }
}