use crate::parsers::csv_parser;
use crate::models::format::Table;


pub fn open_file_to_table() -> Table {

    let mut data =Table::new();
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        data = csv_parser::load_csv(path);
    }
    data
}


pub fn load_data(table_data: &Table, ui: &mut egui::Ui){

    ui.horizontal(|ui| {
            ui.label("File INFO:");
            let header_string = table_data.header.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ");
            ui.label(header_string);
        });
}