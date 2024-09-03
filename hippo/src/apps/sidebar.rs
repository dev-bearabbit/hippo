
pub fn custom_sidebar(ui: &mut egui::Ui) {

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
            }
        });
        //side bar separator
        ui.separator();
    });
}