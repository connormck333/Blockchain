use eframe::egui;
use uuid::Uuid;
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
    selected_node_id: Option<Uuid>
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sidebar.show(ctx, &mut self.network.nodes, &mut self.selected_node_id);

        if self.selected_node_id.is_some() {
            self.node_menu.show(ctx, &mut self.network, *self.selected_node_id.as_ref().unwrap(), &mut self.log_panel);

            if self.node_menu.is_blockchain_menu_open() {
                let mut selected_node = self.network.get_node_by_id(*self.selected_node_id.as_ref().unwrap()).clone();
                self.blockchain_menu.show(ctx, selected_node.blockchain);
            }
        }

        self.log_panel.show(ctx);
    }
}