use egui::{Color32, FontId, Painter, Pos2, Vec2};
use std::f32::consts::PI;
use crate::models::table::RecordTable;
use crate::apps::util::Dropbox;

pub struct PieGraph {
    pub value_col: Dropbox,
    pub label_col: Dropbox,
    pub value_val: Vec<f32>,   // 각 섹터의 값
    pub label_val: Vec<String>, // 각 섹터의 레이블
    pub colors: Vec<Color32>, // 각 섹터의 색상
}

impl PieGraph {
    pub fn new() -> Self {
        Self { 
               value_col:Dropbox::new(1),
               label_col: Dropbox::new(2),
               value_val: vec![30.0, 20.0, 50.0],
               label_val: vec!["A".to_string(), "B".to_string(), "C".to_string()],
               colors: vec![Color32::RED, Color32::GREEN, Color32::BLUE],
            }
    }

    pub fn draw_pie_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable, edit_mode: bool) {
        if edit_mode {
            egui::Frame::default()
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
                ui.add_space(5.0);
                let mut columns = table_data.dataframe.get_column_names();

                if columns.is_empty() {
                    self._check_data_exist(ui);
                } else {
                    columns.insert(0, "select");
                    ui.label(egui::RichText::new("Label").size(13.0));
                    self.label_col.select_column_dropbox(ui, &columns);
                    ui.allocate_space(egui::Vec2::new(10.0, 0.0));
                    ui.label(egui::RichText::new("Value").size(13.0));
                    self.value_col.select_column_dropbox(ui, &columns);
                };
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
        
                self.set_pie_chart(ui, table_data);
            });
        } else {
            self.set_pie_chart(ui, table_data);
        }
    }

    pub fn set_pie_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable) {

        // 파이 차트의 중심과 반지름 설정
        let (rect, _response) = ui.allocate_exact_size(Vec2::new(200.0, 200.0), egui::Sense::hover());
        let center = rect.center();
        let radius = rect.width() * 0.4;

        let total: f32 = self.value_val.iter().sum();
        let mut start_angle = 0.0;

        // 파이 섹터 그리기
        for (i, &value) in self.value_val.iter().enumerate() {
            let sweep_angle = (value / total) * 2.0 * PI;
            let end_angle = start_angle + sweep_angle;

            // 각 섹터 그리기
            self.draw_pie_sector(ui.painter(), center, radius, start_angle, end_angle, self.colors[i]);

            // 섹터 레이블 표시
            let label_angle = start_angle + sweep_angle / 2.0;
            let label_pos = Pos2::new(
                center.x + label_angle.cos() * radius * 0.7,
                center.y + label_angle.sin() * radius * 0.7,
            );

            let font_id = FontId::proportional(14.0);  // 글자 크기 지정
            ui.painter().text(label_pos, egui::Align2::CENTER_CENTER, &self.label_val[i], font_id, Color32::BLACK);

            start_angle = end_angle; // 다음 섹터 시작점
        }
    }

    // 파이 섹터 그리기 함수
    fn draw_pie_sector(&self, painter: &Painter, center: Pos2, radius: f32, start_angle: f32, end_angle: f32, color: Color32) {
        let segments = 64; // 파이 섹터를 그리기 위한 분할 수
        let angle_step = (end_angle - start_angle) / segments as f32;

        let mut points = vec![center];
        for i in 0..=segments {
            let angle = start_angle + i as f32 * angle_step;
            let x = center.x + angle.cos() * radius;
            let y = center.y + angle.sin() * radius;
            points.push(Pos2::new(x, y));
        }

        painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::NONE));
    }

    fn _check_data_exist(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Not Found Data.").size(15.0));
        ui.add_space(5.0);
        ui.label(egui::RichText::new("Please Import CSV or Excel File").size(15.0));
    }

}
