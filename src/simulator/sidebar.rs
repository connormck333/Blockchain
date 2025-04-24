use eframe::egui;

pub struct Sidebar;

impl Default for Sidebar {
    fn default() -> Self {
        Self {}
    }
}

impl Sidebar {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.vertical(|ui|{
                    ui.heading("Menu");
                    ui.separator();

                    if ui.button("Mine block").clicked() {
                        println!("Mine block clicked");
                    }

                    ui.add_space(5.0);

                    if ui.button("Transfer money").clicked() {
                        println!("Transfer money clicked");
                    }

                    let margin_top = ui.available_height() - 25.0;
                    ui.add_space(margin_top);

                    ui.label("Balance: $100.00")
                });
            });
    }
}