use eframe::{self, egui};
mod models;
use models::computer::Computer;
use models::task::Task;
use egui_extras::TableBuilder;
use egui_extras::Column;
use sysinfo::System;

//holds the base data like fields
struct BaseData 
{
    system: System,
    tasks: Vec<Task>,
    selectedTask: Option<Task>,
    live: bool,
    computer: Computer,
}

//inititializes the BaseData
impl Default for BaseData 
{
    fn default() -> Self 
    {
        let mut data = Self 
        {
            system: System::new_all(),
            tasks: vec![],
            selectedTask: None,
            live: false,
            computer: Computer::new(),
        };
        data.PopulateList();

        data
    }
}
impl BaseData 
{
    fn PopulateList(&mut self)
    {
        self.system.refresh_all();

        self.tasks.clear();

        for (pid, process) in self.system.processes()
        {
            self.tasks.push(
            Task
            {
                Name: process.name().to_string(),
                pid: pid.as_u32(),
                CPUPercentage: process.cpu_usage(),
                RamPercentage: 0.0,  // You can calculate this if you want
                RamBytes: process.memory() * 1024,  // convert KB to bytes
                StorageUsagePercentage: 0.0,  // placeholder for now
                StorageUsageBytes: 0,         // placeholder
                NetworkPercentage: 0.0,       // placeholder
                NetworkBytes: 0,              // placeholder
            });
        }
    }
}

//definies the gui
impl eframe::App for BaseData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.live == true
        {
            self.PopulateList();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            //header
            ui.heading("Rust Bucket");

            ui.group(|ui|
            {

                ui.horizontal(|ui| 
                    {
                        ui.checkbox(&mut self.live, "Live");
                        if ui.button("Refresh").clicked() 
                        {
                            self.PopulateList();
                        }
    
                    });
    
                //selected task display
                if let Some(task) = &self.selectedTask 
                {
                    ui.label(format!("Selected Process: {} : {}", task.Name, task.pid));
                }
                else
                {
                    ui.label("Selected Process:");
                }
            });

            //task table
            ui.group(|ui| {

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
                                row.col(|ui| { ui.label(format!("{:.1}%", task.CPUPercentage)); });
                                row.col(|ui| { ui.label(format!("{:.1}%", task.RamPercentage)); });
                                row.col(|ui| { ui.label(format!("{}", task.RamBytes)); });
                                row.col(|ui| { ui.label(format!("{:.1}%", task.StorageUsagePercentage)); });
                                row.col(|ui| { ui.label(format!("{}", task.StorageUsageBytes)); });
                                row.col(|ui| { ui.label(format!("{:.1}%", task.NetworkPercentage)); });
                                row.col(|ui| { ui.label(format!("{}", task.NetworkBytes)); });
                            });
                        }
                });
            });

        });
    }
}
//this is the main function
fn main() -> Result<(), eframe::Error> 
    {

    let mut options = eframe::NativeOptions::default();

    options.viewport.maximized = Some(true);

    //launches app
    eframe::run_native(
        "Rust Bucket", 
        options, 
        Box::new(|_cc| Box::new(BaseData::default())),
        )
    }