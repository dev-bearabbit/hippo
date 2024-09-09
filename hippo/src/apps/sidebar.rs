use eframe::egui;

pub fn custom_sidebar(ui: &mut egui::Ui) {

    ui.horizontal(|ui| {
        // 사이드바 영역
        ui.vertical(|ui| {
            let sidebar_width = 180.0; // 사이드바의 고정된 너비
            let available_height = ui.available_height(); // 사용 가능한 전체 높이

            // 고정된 크기의 사이드바 할당
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(sidebar_width, available_height),
                egui::Sense::hover(),
            );

            // 사이드바 내에서 중앙 정렬된 공간 할당
            let content_size = egui::vec2(sidebar_width, 10.0); // 100은 버튼들 총 높이
            let content_rect = egui::Rect::from_center_size(
                rect.center(), // rect의 중심을 사용
                content_size,
            );

            ui.allocate_ui_at_rect(content_rect, |ui| {

                ui.vertical_centered(|ui| {
                // 사이드바 내부의 UI 요소
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("HIPPO").size(24.0).strong());
                    ui.add_space(10.0);
                    ui.label("High-Integration Plotting and Processing with Rust");
                    ui.add_space(10.0);
                    ui.separator();

                    // 그래프 선택하는 부분
                    ui.label(egui::RichText::new("Data Visualization").size(15.0).strong());
                    ui.add_space(10.0);
                    ui.label("Add a Graph");
                    ui.add_space(10.0);
                    
                    // 첫 번째 버튼 (배경색과 크기 조정)
                    if ui.add_sized(
                        egui::vec2(120.0, 30.0), // 버튼 크기
                        egui::Button::new(egui::RichText::new("Table").size(20.0))
                            .fill(egui::Color32::from_rgb(100, 150, 250)), // 배경색
                    ).clicked() {
                        // 버튼 클릭 시 동작
                        }

                    ui.add_space(5.0);

                    // 두 번째 버튼 (배경색과 크기 조정)
                    if ui.add_sized(
                        egui::vec2(120.0, 30.0), // 버튼 크기
                        egui::Button::new(egui::RichText::new("Table 2").size(20.0))
                            .fill(egui::Color32::from_rgb(50, 100, 200)), // 배경색
                    ).clicked() {
                        // 버튼 클릭 시 동작
                        }
                    
                    ui.add_space(10.0);

                    // 대시보드 꾸미는 기능 선택하는 부분
                    ui.separator();
                    ui.label(egui::RichText::new("Customize Layout").size(15.0).strong());
                    ui.add_space(10.0);
                    ui.label("Add a Layout Setting");
                    ui.add_space(10.0);
                
                    });
                });
            });
        //side bar separator
        ui.separator();
    });
}