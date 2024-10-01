use crate::models::table::RecordTable;
use crate::apps::file;

#[derive(Default)]
pub struct Menu {
    pub table_data: RecordTable,
    pub save_data: String,
    pub invalid_file: bool
}

impl Menu {
    pub fn update_menu_bar(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {

        ui.horizontal(|ui| {

            ui.menu_button("File", |ui| {

                if ui.button("Get CSV File").clicked() {
                    let file = file::open_csv_file_to_table();
                    match file {
                        Ok(data) => {
                            self.table_data = data;
                        },
                        Err(e) => {
                            println!("Error: {:?}", e);
                            self.invalid_file = true;
                        }
                    }
                    ui.close_menu();
                }
            
                if ui.button("Get Excel File").clicked() {
                    let file = file::open_excel_file_to_table();
                    match file {
                        Ok(data) => {
                            self.table_data = data;
                        },
                        Err(e) => {
                            println!("Error: {:?}", e);
                            self.invalid_file = true;
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
    
            self.theme_light_dark_mode(ui);
            self.show_invalid_file_popup(ctx);
        });

        ui.separator();
    }

    fn show_invalid_file_popup(&mut self, ctx: &egui::Context) {
        if self.invalid_file {
            egui::Window::new("Invalid File")
                .collapsible(false)
                .title_bar(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Invalid File Type").size(18.0).strong());
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("You have selected an invalid file type.").size(13.0));
                    ui.label(egui::RichText::new("Please choose a valid file type.").size(13.0));
                    ui.add_space(10.0);

                    if ui.button("OK").clicked() {
                        self.invalid_file = false;
                        }
                    });
                });
        }
    }

    fn theme_light_dark_mode(&mut self, ui: &mut egui::Ui) {

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::widgets::global_dark_light_mode_buttons(ui);
        });
    }

}