use crate::gui::gui::Gui;

mod wallet;
mod transaction;
mod gui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Blockchain Wallet",
        options,
        Box::new(|_cc| Ok(Box::new(Gui::default())))
    )
}
