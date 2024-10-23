use polars::series::Series;
use polars::prelude::DataType;

pub struct Dropbox {
    pub id: usize,
    pub selected: usize,
}

impl Dropbox {
    pub fn new(id: usize) -> Self {
        Self {
            id: id,
            selected: 0,
        }
    }

    pub fn select_column_dropbox(&mut self, ui: &mut egui::Ui, columns: &Vec<&str>) {

        egui::ComboBox::new(self.id, "")
            .selected_text(columns[self.selected])
            .show_ui(ui, |ui| {
                for (i, &column) in columns.iter().enumerate() {
                    ui.selectable_value(&mut self.selected, i, column);
                }
            });
    }
}

pub fn cast_data_type_as_f64(col_data :&Series) -> Vec<f64> {

    let mut result = Vec::new();
    match col_data.dtype() {
        DataType::Int64 => {
            result = col_data.i64()
                .map(|ca| ca.into_iter().flatten().map(|v| v as f64).collect())
                .unwrap_or_else(|_| Vec::new());
            result
        }
        DataType::Float64 => {
            result = col_data.f64()
                .map(|ca| ca.into_iter().flatten().collect())
                .unwrap_or_else(|_| Vec::new());
            result
        },
        // 그 외 타입은 빈 벡터 반환
        _ => result,
    }
}
