use crate::models::table::RecordTable;
use crate::models::error::TableError;

pub fn open_csv_file_to_table() -> Result<RecordTable, TableError> {

    let mut data = RecordTable::new();
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        data = RecordTable::from_csv(data, path)?;
        Ok(data)
    } else {
        Err(TableError::NotFound("No file selected".to_string()))
    }

}

pub fn open_excel_file_to_table() -> Result<RecordTable, TableError> {

    let mut data = RecordTable::new();
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        data = RecordTable::from_excel(data, path)?;
        Ok(data)
    } else {
        Err(TableError::NotFound("No file selected".to_string()))
    }
}

pub fn load_data(table_data: &RecordTable, ui: &mut egui::Ui){

    ui.horizontal(|ui| {
            ui.label("File INFO:");
            let header_string = table_data.dataframe.get_column_names().iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ");
            ui.label(header_string);
        });
}