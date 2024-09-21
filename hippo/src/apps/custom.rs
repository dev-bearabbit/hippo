
pub fn get_column_list(ui: &mut egui::Ui, column_list: Vec<&str>) {

    let text_style = egui::TextStyle::Body;
    let row_height = ui.text_style_height(&text_style);
    let num_rows = column_list.len();

    if num_rows == 0 {
        egui::ScrollArea::vertical().auto_shrink([false; 2])
        .max_height(50.0)
        .show(
            ui,
            |ui| {
                ui.label("Not Found Data");
            }
        );
    } else {
        egui::ScrollArea::vertical().auto_shrink([false; 2])
        .max_height(50.0)
        .show_rows(
            ui,
            row_height,
            num_rows,
            |ui, row_range| {
                for row in row_range {
                    ui.label(column_list[row]);
                }
            },
        );
    }
}