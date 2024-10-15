use eframe::egui;
use crate::models::chart::ChartType;
use crate::apps::dashboard::Dashboard;
use crate::models::table::RecordTable;

use super::graph::bar::BarGraph;
use super::graph::line::LineGraph;
use super::custom::text::TextCustom;

pub fn custom_sidebar(ui: &mut egui::Ui, table_data: RecordTable, dashboards: &mut Vec<Dashboard>, ctx: &egui::Context) {

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

                    // column 목록 확인 파트
                    ui.label(egui::RichText::new("Column List").size(13.0).strong());
                    ui.add_space(10.0);

                    let col_names = table_data.dataframe.get_column_names();
                    get_column_list(ui, col_names);

                    ui.add_space(10.0);
                    ui.separator();

                    // 그래프 선택하는 부분
                    ui.label(egui::RichText::new("Data Visualization").size(13.0).strong());
                    ui.add_space(10.0);
                    ui.label("Add a Graph");
                    ui.add_space(10.0);

                    // 그래프 추가 버튼
                    add_sidebar_dashboard_button(ui, 110.0, 30.0, "Line graph", 13.0, dashboards, ChartType::Line(LineGraph::new()));
                    ui.add_space(5.0);

                    add_sidebar_dashboard_button(ui, 110.0, 30.0, "Bar graph", 13.0, dashboards, ChartType::Bar(BarGraph::new()));
                    ui.add_space(5.0);

                    add_sidebar_dashboard_button(ui, 110.0, 30.0, "Pie graph", 13.0, dashboards, ChartType::Pie);
                    ui.add_space(5.0);

                    add_sidebar_dashboard_button(ui, 110.0, 30.0, "Scatter graph", 13.0, dashboards, ChartType::Scatter);
                    ui.add_space(5.0);

                    add_sidebar_dashboard_button(ui, 110.0, 30.0, "Histogram graph", 13.0, dashboards, ChartType::Histogram);
                    ui.add_space(5.0);
                
                ui.add_space(10.0);

                // 대시보드 꾸미는 기능 선택하는 부분
                ui.separator();
                ui.label(egui::RichText::new("Customize Layout").size(13.0).strong());
                ui.add_space(10.0);
                ui.label("Add a Layout Setting");
                ui.add_space(10.0);

                add_sidebar_dashboard_button(ui, 110.0, 30.0, "Add a Text", 13.0, dashboards, ChartType::Text(TextCustom::new()));
                ui.add_space(5.0);

                add_sidebar_dashboard_button(ui, 110.0, 30.0, "Add a Image", 13.0, dashboards , ChartType::Image);
                ui.add_space(5.0);

                add_sidebar_dashboard_button(ui, 110.0, 30.0, "Add a Table", 13.0, dashboards, ChartType::Table);

                ui.add_space(10.0);

            });

        });

    // 저장된 모든 대시보드 업데이트 호출
    for dashboard in dashboards.iter_mut() {
        dashboard.update_dashboard(ctx, ui, &table_data);
    }

}

fn add_sidebar_dashboard_button(
        ui: &mut egui::Ui,
        x:f32,
        y:f32,
        text:&str,
        size:f32,
        dashboards: &mut Vec<Dashboard>,
        chart: ChartType) {

    let is_dark_mode = ui.visuals().dark_mode;

    let button_color = if is_dark_mode {
        egui::Color32::from_rgb(0, 92, 138)
    } else {
        egui::Color32::from_rgb(144, 209, 255)
    };

    if ui.add_sized(
        egui::vec2(x, y), // 버튼 크기
        egui::Button::new(egui::RichText::new(text).strong().size(size))
        .fill(button_color), // 배경색
    ).clicked() {
        let new_id = dashboards.len(); // 고유 ID로 벡터 길이 사용
        dashboards.push(Dashboard::new(new_id, false, chart)); // 새로운 Dashboard 추가
        };
}

fn get_column_list(ui: &mut egui::Ui, column_list: Vec<&str>) {

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

