use eframe::egui;
use crate::simulator::sidebar::Sidebar;

#[derive(Default)]
pub struct Gui {
    sidebar: Sidebar
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sidebar.show(ctx);

        egui::CentralPanel::default().show(ctx, |_ui| {

        });
    }
}