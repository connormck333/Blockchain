use eframe::egui;
use crate::block::Block;
use crate::utils::format_timestamp;

pub struct BlockModal;

impl Default for BlockModal {
    fn default() -> Self {
        Self {}
    }
}

impl BlockModal {
    pub fn show(&mut self, ctx: &egui::Context, block: Block, visible: &mut bool) {
        egui::Window::new(format!("Block {}", block.index))
            .resizable(false)
            .collapsible(false)
            .default_height(350.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |scroll_ui| {
                        scroll_ui.horizontal(|horiz_ui| {
                            horiz_ui.label(format!("Index: {}", block.index));

                            horiz_ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Close").clicked() {
                                    *visible = false;
                                }
                            });
                        });
                        scroll_ui.add_space(5.0);

                        scroll_ui.label(format!("Time mined: {}", format_timestamp(block.timestamp)));
                        scroll_ui.add_space(5.0);

                        scroll_ui.label(format!("Nonce: {}", block.nonce));
                        scroll_ui.add_space(5.0);

                        scroll_ui.label(format!("Difficulty: {}", block.difficulty));
                        scroll_ui.add_space(5.0);

                        scroll_ui.label(format!("Hash: {}", block.hash));
                        scroll_ui.add_space(5.0);

                        scroll_ui.label(format!("Previous block hash: {}", block.previous_block_hash));
                        scroll_ui.add_space(5.0);

                        scroll_ui.separator();

                        scroll_ui.label(format!("Transactions: {}", block.transactions.len()));
                        scroll_ui.add_space(5.0);

                        for tx in block.transactions.iter() {
                            scroll_ui.label(format!("Transaction ID: {}", tx.id));
                            scroll_ui.add_space(5.0);

                            scroll_ui.label(format!("Sender: {}", tx.sender));
                            scroll_ui.add_space(5.0);

                            scroll_ui.label(format!("Recipient address: {}", tx.recipient));
                            scroll_ui.add_space(5.0);

                            scroll_ui.label(format!("Amount: {}", tx.amount));
                            scroll_ui.add_space(5.0);

                            scroll_ui.label(format!("Time of transaction: {}", format_timestamp(tx.timestamp)));
                            scroll_ui.add_space(5.0);
                        }
                    });
            });
    }
}