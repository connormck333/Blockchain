use eframe::egui;
use crate::node::Node;
use crate::simulator::logs::LogPanel;
use crate::simulator::node_menu::NodeMenu;
use crate::simulator::sidebar::Sidebar;

#[derive(Default)]
pub struct Gui {
    sidebar: Sidebar,
    node_menu: NodeMenu,
    log_panel: LogPanel,
    nodes: Vec<Node>,
    selected_node: Option<Node>
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sidebar.show(ctx, &mut self.nodes, &mut self.selected_node);

        if self.selected_node.is_some() {
            self.node_menu.show(ctx, self.selected_node.as_ref().unwrap(), &mut self.log_panel);
        }

        self.log_panel.show(ctx);
    }
}