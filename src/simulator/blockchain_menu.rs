use eframe::egui;
use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::simulator::block_modal::BlockModal;

pub struct BlockchainMenu {
    block_modal: BlockModal,
    selected_block: Option<Block>,
    modal_visible: bool
}

impl Default for BlockchainMenu {
    fn default() -> Self {
        Self {
            block_modal: BlockModal::default(),
            selected_block: None,
            modal_visible: false
        }
    }
}

impl BlockchainMenu {
    pub fn show(&mut self, ctx: &egui::Context, blockchain: Blockchain) {
        egui::SidePanel::left("blockchain_menu")
            .resizable(false)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Blockchain");
                    ui.separator();
                    ui.add_space(5.0);

                    for block in blockchain.get_chain() {
                        if ui.button(format!("Block {}", block.index)).clicked() {
                            self.selected_block = Some(block.clone());
                            self.modal_visible = true;
                        }
                    }
                });

                if self.modal_visible {
                    self.block_modal.show(ctx, self.selected_block.as_ref().unwrap().clone(), &mut self.modal_visible)
                }
            });
    }
}