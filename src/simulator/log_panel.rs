use eframe::egui;
use crate::simulator::log::Log;

pub struct LogPanel {
    logs: Vec<Log>
}

impl Default for LogPanel {
    fn default() -> Self {
        Self {
            logs: Vec::new()
        }
    }
}

impl LogPanel {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.heading("Blockchain Logs");
                ui.separator();

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |scroll_ui| {
                        for log in self.logs.iter() {
                            scroll_ui.heading(log.heading.as_str());
                            scroll_ui.add_space(2.0);

                            scroll_ui.label(log.info.as_str());
                            scroll_ui.add_space(2.0);

                            let mut owner_label = "Log raised by ".to_owned();
                            owner_label.push_str(log.owner.as_str());
                            scroll_ui.label(owner_label.as_str());

                            scroll_ui.separator();
                        }
                    })
            });
    }

    pub fn add_log(&mut self, heading: String, info: String, owner: String) {
        self.logs.push(Log {heading, info, owner});
    }
}