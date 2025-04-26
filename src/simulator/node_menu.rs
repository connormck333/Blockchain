use eframe::egui;
use crate::block::Block;
use crate::network::Network;
use crate::node::Node;
use crate::simulator::create_transaction_modal::CreateTransactionModal;
use crate::simulator::log_panel::LogPanel;
use crate::utils::format_timestamp;

pub struct NodeMenu {
    modal_visible: bool,
    create_transaction_modal: CreateTransactionModal
}

impl Default for NodeMenu {
    fn default() -> Self {
        Self {
            modal_visible: false,
            create_transaction_modal: CreateTransactionModal::new()
        }
    }
}

impl NodeMenu {
    pub fn show(&mut self, ctx: &egui::Context, network: &mut Network, node: &mut Node, log_panel: &mut LogPanel) {
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading(node.name.clone());
                ui.separator();

                if ui.button("Send Transaction").clicked() {
                    self.modal_visible = true;
                }

                ui.add_space(5.0);

                if ui.button("Mine block").clicked() {
                    let mined_block = node.mine_block();
                    Self::add_mined_block_log(log_panel, mined_block.clone(), node.name.clone());
                    network.broadcast_block(mined_block.clone(), log_panel);
                }

                let margin_top = ui.available_height() - 30.0;
                ui.add_space(margin_top);

                ui.label("Balance: $100");

                if self.modal_visible {
                    self.create_transaction_modal.show(ctx, node, network.nodes.clone(), &mut self.modal_visible, log_panel);
                }
            });
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