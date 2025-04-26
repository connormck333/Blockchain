use eframe::egui;
use crate::network::Network;
use crate::node::Node;
use crate::simulator::blockchain_menu::BlockchainMenu;
use crate::simulator::log_panel::LogPanel;
use crate::simulator::node_menu::NodeMenu;
use crate::simulator::sidebar::Sidebar;

#[derive(Default)]
pub struct Gui {
    sidebar: Sidebar,
    node_menu: NodeMenu,
    log_panel: LogPanel,
    network: Network,
    blockchain_menu: BlockchainMenu,
    blockchain_menu_visible: bool,
    selected_node: Option<Node>
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sidebar.show(ctx, &mut self.network.nodes, &mut self.selected_node);

        if self.selected_node.is_some() {
            self.node_menu.show(ctx, &mut self.network, &mut self.selected_node.as_mut().unwrap(), &mut self.log_panel);

            if self.node_menu.is_blockchain_menu_open() {
                self.blockchain_menu.show(ctx, &self.selected_node.as_ref().unwrap().blockchain);
            }
        }

        self.log_panel.show(ctx);
    }
}