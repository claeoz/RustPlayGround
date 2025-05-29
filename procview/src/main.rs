use eframe::{self, egui};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Rust Bucket", options, |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("this is a label chuckle duck")
        });
    })
}
