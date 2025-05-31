use eframe::{self, egui};
mod models;
use models::task::Task;
use egui_extras::TableBuilder;
use egui_extras::Column;


//holds the base data like fields
struct BaseData {
    counter: u32,
    tasks: Vec<Task>,
    selectedTask: Option<Task>,
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
                ],
            selectedTask: None,
        }
    }
}

//definies the gui
impl eframe::App for BaseData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Bucket");
            if let Some(task) = &self.selectedTask {
                ui.label(format!("Selected Process: {} : {}", task.Name, task.pid));
            }

            let table = TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .resizable(true);

            table
                .header(20.0, |mut header| {
                    header.col(|ui| { ui.label("Name"); });
                    header.col(|ui| { ui.label("PID"); });
                    header.col(|ui| { ui.label("CPU %"); });
                    header.col(|ui| { ui.label("RAM %"); });
                    header.col(|ui| { ui.label("RAM Bytes"); });
                    header.col(|ui| { ui.label("Storage %"); });
                    header.col(|ui| { ui.label("Storage Bytes"); });
                    header.col(|ui| { ui.label("Network %"); });
                    header.col(|ui| { ui.label("Network Bytes"); });
                })
                .body(|mut body| {
                    for task in &self.tasks {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                if ui.button(&task.Name).clicked() {
                                self.selectedTask = Some(task.clone());
                            }
                        });
                            row.col(|ui| { ui.label(task.pid.to_string()); });
                            row.col(|ui| { ui.label(format!("{:.1}", task.CPUPercentage)); });
                            row.col(|ui| { ui.label(format!("{:.1}", task.RamPercentage)); });
                            row.col(|ui| { ui.label(format!("{}", task.RamBytes)); });
                            row.col(|ui| { ui.label(format!("{:.1}", task.StorageUsagePercentage)); });
                            row.col(|ui| { ui.label(format!("{}", task.StorageUsageBytes)); });
                            row.col(|ui| { ui.label(format!("{:.1}", task.NetworkPercentage)); });
                            row.col(|ui| { ui.label(format!("{}", task.NetworkBytes)); });
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