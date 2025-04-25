use eframe::egui;
use crate::node::Node;
use crate::simulator::logs::LogPanel;

pub struct NodeMenu;

impl Default for NodeMenu {
    fn default() -> Self {
        Self {}
    }
}

impl NodeMenu {
    pub fn show(&mut self, ctx: &egui::Context, node: &Node, log_panel: &mut LogPanel) {
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading(node.name.clone());
                ui.separator();

                if ui.button("Send Transaction").clicked() {
                    log_panel.add_log("Heading".to_string(), "Info lorem ipsum dolor init nfdjskfdsbnfskdnf dsf idsonfdsjnf kj".to_string(), node.name.clone());
                }
                ui.add_space(5.0);
                if ui.button("Mine block").clicked() {
                    println!("Mine block");
                }

                let margin_top = ui.available_height() - 30.0;
                ui.add_space(margin_top);

                ui.label("Balance: $100");
            });
    }
}