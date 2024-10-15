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
