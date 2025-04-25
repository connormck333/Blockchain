use eframe::egui;
use crate::node::Node;

pub struct AddNodeModal {
    pub name_value: String
}

impl AddNodeModal {
    pub fn new() -> Self {
        Self {
            name_value: String::new()
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, nodes: &mut Vec<Node>, visible: &mut bool) {
        egui::Window::new("Add node")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("Node name:");
                ui.add_space(5.0);
                ui.text_edit_singleline(&mut self.name_value);
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    if ui.button("Confirm").clicked() {
                        nodes.push(Node::new(self.name_value.as_str(), 3));
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
        self.name_value.clear();
    }
}