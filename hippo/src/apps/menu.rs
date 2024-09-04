use crate::models::table::RecordTable;
use crate::apps::file;

#[derive(Default)]
pub struct Menu {
    pub table_data: RecordTable,
    pub save_data: String
}

impl Menu {
    pub fn update_menu_bar(&mut self, ui: &mut egui::Ui) {

        ui.horizontal(|ui| {

            ui.menu_button("File", |ui| {
                if ui.button("Get CSV").clicked() {
                    let file = file::open_csv_file_to_table();
                    match file {
                        Ok(data) => {
                            self.table_data = data;
                        },
                        Err(e) => {
                            println!("Error: {:?}", e);
                        }
                    }
                    ui.close_menu();
                }
            
                if ui.button("Get Excel").clicked() {
                    let file = file::open_excel_file_to_table();
                    match file {
                        Ok(data) => {
                            self.table_data = data;
                        },
                        Err(e) => {
                            println!("Error: {:?}", e);
                        }
                    }
                    ui.close_menu();
                }
            });

            ui.menu_button("Save", |ui| {
                if ui.button("Export Raw File").clicked() {
                    ui.close_menu();
                }
                if ui.button("Export as PDF").clicked() {
                    ui.close_menu();
                }
            });
    
            theme_light_dark_mode(ui)
        });

        ui.separator();
    }
}

fn theme_light_dark_mode(ui: &mut egui::Ui) {

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        egui::widgets::global_dark_light_mode_buttons(ui);
    });
}