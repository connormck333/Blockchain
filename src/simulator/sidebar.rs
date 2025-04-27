use eframe::egui;
use uuid::Uuid;
use crate::node::Node;
use crate::simulator::add_node_modal::AddNodeModal;

pub struct Sidebar {
    modal_visible: bool,
    add_node_modal: AddNodeModal
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            modal_visible: false,
            add_node_modal: AddNodeModal::new()
        }
    }
}

impl Sidebar {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        nodes: &mut Vec<Node>,
        selected_node: &mut Option<Uuid>
    ) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.vertical(|ui|{
                    ui.heading("Nodes");
                    ui.separator();

                    for node in nodes.iter() {
                        if ui.button(&node.name).clicked() {
                            *selected_node = Some(node.id.clone());
                        }
                        ui.add_space(5.0);
                    }

                    let margin_top = ui.available_height() - 30.0;
                    ui.add_space(margin_top);

                    if ui.button("Add Node").clicked() {
                        self.modal_visible = true;
                    }

                    if self.modal_visible {
                        self.add_node_modal.show(ctx, nodes, &mut self.modal_visible);
                    }
                });
            });
    }
}