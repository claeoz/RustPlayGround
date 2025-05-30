use eframe::{self, egui};
mod models;
use models::task::Task;

//holds the base data like fields
struct BaseData {
    counter: u32,
    tasks: Vec<Task>,
}

//inititializes the BaseData
impl Default for BaseData {
    fn default() -> Self {
        Self 
        { 
            counter: 0,
            tasks: vec![
                Task {
                    Name: "Delete Me".to_string(),
                    pid: 0,
                    CPUPercentage: 1.1,
                    RamPercentage: 1.1,
                    RamBytes: 12,
                    StorageUsagePercentage: 1.1,
                    StorageUsageBytes: 12,
                    NetworkPercentage: 1.1,
                    NetworkBytes: 12
                }
            ]
        }
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
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.label("PID");
                    ui.label("CPU Percentage");
                    ui.label("Ram Percentage");
                    ui.label("Ram Bytes");
                    ui.label("Storage Percentage");
                    ui.label("Storage Bytes");
                    ui.label("Network Percentage");
                    ui.label("Network Bytes");
                });
                for task in &self.tasks {
                    ui.horizontal(|ui| {
                        ui.label(&task.Name);
                        ui.label(task.pid.to_string());
                        ui.label(format!("{:.1}", task.CPUPercentage));
                        ui.label(format!("{:.1}", task.RamPercentage));
                        ui.label(format!("{}", task.RamBytes));
                        ui.label(format!("{:.1}", task.StorageUsagePercentage));
                        ui.label(format!("{}", task.StorageUsageBytes));
                        ui.label(format!("{:.1}", task.NetworkPercentage));
                        ui.label(format!("{}", task.NetworkBytes));
                    });
                }
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