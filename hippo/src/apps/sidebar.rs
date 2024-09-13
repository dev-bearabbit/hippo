use eframe::egui;
use crate::apps::dashboard::Dashboard;

pub fn custom_sidebar(ui: &mut egui::Ui, dashboards: &mut Vec<Dashboard>, ctx: &egui::Context) {

    egui::SidePanel::left("sidebar")
    .resizable(true)
    .default_width(150.0)
    .width_range(150.0..=200.0)
    .show_inside(ui, |ui| {
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
                        egui::Button::new(egui::RichText::new("Table").size(15.0))
                            .fill(egui::Color32::from_rgb(100, 150, 250)), // 배경색
                    ).clicked() {
                        let new_id = dashboards.len(); // 고유 ID로 벡터 길이 사용
                        dashboards.push(Dashboard::new(new_id)); // 새로운 Dashboard 추가
                        }

                    ui.add_space(5.0);

                   // 두 번째 버튼 (배경색과 크기 조정)
                   if ui.add_sized(
                    egui::vec2(120.0, 30.0), // 버튼 크기
                    egui::Button::new(egui::RichText::new("Table 2").size(15.0))
                        .fill(egui::Color32::from_rgb(50, 100, 200)), // 배경색
                ).clicked() {
                    let new_id = dashboards.len(); // 고유 ID로 벡터 길이 사용
                    dashboards.push(Dashboard::new(new_id)); // 새로운 Dashboard 추가
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

    // 저장된 모든 대시보드 업데이트 호출
    for dashboard in dashboards.iter_mut() {
        dashboard.update_dashboard(ctx, ui);
    }

}