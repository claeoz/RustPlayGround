use eframe::{self, egui};

//holds the base data like fields
struct BaseData {
    counter: u32
}

//inititializes the BaseData
impl Default for BaseData {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

//definies the gui
impl eframe::App for BaseData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Bucket");

            ui.group(|ui| {
                ui.label(format!("Count: {}", self.counter));
    
                if ui.button("Increment").clicked() {
                    self.counter += 1;
                };
            });
            ui.vertical(|ui| {
                ui.label("one");
                ui.label("two");
            });
        });
    }
}
//this is the main function
fn main() -> Result<(), eframe::Error> {

    let mut options = eframe::NativeOptions::default();

    options.viewport.maximized = Some(true);

    //launches app
    eframe::run_native(
        "Rust Bucket", 
        options, 
        Box::new(|_cc| Box::new(BaseData::default())),
    )
}