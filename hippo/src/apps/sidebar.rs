use crate::models::format::Table;
use crate::apps::file;

pub fn custom_sidebar(ui: &mut egui::Ui) {

    let mut table_data = Table::new();

    ui.horizontal(|ui| {
        // 사이드바 영역
        ui.vertical(|ui| {
            let sidebar_width = 200.0; // 사이드바의 고정된 너비
            let available_height = ui.available_height(); // 사용 가능한 전체 높이

            // 고정된 크기의 사이드바
            ui.allocate_exact_size(
                egui::vec2(sidebar_width, available_height),
                egui::Sense::hover(),
            );

            // 사이드바 내부의 UI 요소
            ui.label("FILE OPEN TEST");

            if ui.button("Open File").clicked() {
                table_data = file::open_file_to_table();
            }

            if !table_data.header.is_empty() {
                file::load_data(&table_data, ui);
            }
        });
        //side bar separator
        ui.separator();
    });
}