use crate::models::format::Table;
use crate::apps::file;

#[derive(Default)]
pub struct Menu {
    pub table_data: Table,
    pub save_data: String
}

impl Menu {
    pub fn update_menu_bar(&mut self, ui: &mut egui::Ui) {

        ui.horizontal(|ui| {

            ui.menu_button("File", |ui| {
                if ui.button("Get CSV").clicked() {
                    self.table_data = open_csv();
                    ui.close_menu();
                }
            
                if ui.button("Get Excel").clicked() {
                    self.table_data = open_csv();
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

fn open_csv() -> Table {
    return file::open_file_to_table();
}

fn open_excel(ui: &mut egui::Ui) {
}


fn nested_menu(ui: &mut egui::Ui) {
    ui.set_max_width(200.0); // To make sure we wrap long text

    if ui.button("Open File").clicked() {
        ui.close_menu();
    }
    ui.menu_button("SubMenu", |ui| {
        ui.menu_button("SubMenu", |ui| {
            if ui.button("Open…").clicked() {
                ui.close_menu();
            }
            let _ = ui.button("Item");
        });
        ui.menu_button("SubMenu", |ui| {
            if ui.button("Open…").clicked() {
                ui.close_menu();
            }
            let _ = ui.button("Item");
        });
        let _ = ui.button("Item");
        if ui.button("Open…").clicked() {
            ui.close_menu();
        }
    });
}