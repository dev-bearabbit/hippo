use egui::{Color32, FontId, Pos2, Vec2};
use std::f32::consts::PI;
use crate::models::table::RecordTable;
use crate::apps::util::{Dropbox, cast_data_type_as_f64};

pub struct PieGraph {
    pub value_col: Dropbox,
    pub label_col: Dropbox,
    pub value_val: Vec<f32>,
    pub label_val: Vec<String>,
    pub sector_colors: Vec<Color32>,
    pub sector_gap: f32,
    pub stroke_width: f32,
    pub stroke_colors: Vec<egui::Color32>,
}

impl PieGraph {
    pub fn new() -> Self {
        Self { 
               value_col:Dropbox::new(1),
               label_col: Dropbox::new(2),
               value_val: vec![30.0, 20.0, 10.0, 15.0, 10.0, 5.0, 15.0],
               label_val: vec!["A", "B", "C", "D", "E", "F", "G"].into_iter().map(|s| s.to_string()).collect(),
               sector_colors: vec![egui::Color32::default(); 7],
               sector_gap: 0.0,
               stroke_width: 1.0,
               stroke_colors: vec![egui::Color32::BLACK; 7], 
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
                    ui.horizontal(|ui| { 
                        columns.insert(0, "select");
                        ui.label(egui::RichText::new("Label").size(13.0));
                        self.label_col.select_column_dropbox(ui, &columns);
                        ui.allocate_space(egui::Vec2::new(10.0, 0.0));
                        ui.label(egui::RichText::new("Value").size(13.0));
                        self.value_col.select_column_dropbox(ui, &columns);
                    });
                ui.add_space(5.0);
                ui.horizontal(|ui| { 
                    ui.label(egui::RichText::new("Line Width").size(13.0));
                    ui.add(egui::Slider::new(&mut self.stroke_width, 0.0..=3.0));
                    ui.label(egui::RichText::new("Pie Sector Distance").size(13.0));
                    ui.add(egui::Slider::new(&mut self.sector_gap, 0.0..=10.0));
                });
                };
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
                
                ui.horizontal(|ui| { 
                    let total_width = ui.available_width();
                    let color_panel_width = total_width * 0.3; // 적절한 비율로 색상 선택 패널 너비 설정
                    let pie_chart_width = total_width * 0.7;   // 나머지 공간을 파이 차트에 할당
                    // Pie Chart Layout
                    ui.allocate_ui_with_layout(
                        Vec2::new(pie_chart_width, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            self.set_pie_chart(ui, table_data);
                    });
                    ui.separator();
                    egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.allocate_ui_with_layout(
                            Vec2::new(color_panel_width, ui.available_height()),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                self.select_sector_color(ui);
                            },
                        );
                    });
                    ui.add_space(5.0);
                });
            });
        } else {
            self.set_pie_chart(ui, table_data);
        }
    }

    pub fn set_pie_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable) {

        if self.value_col.selected != 0 && self.label_col.selected != 0 {
            self._cleaning_data_type(table_data);
        }

        // 파이 차트의 중심과 반지름 설정
        let min_size = Vec2::new(250.0, 250.0); // 최소 크기를 정의합니다.
        let available_size = ui.available_size();
        let desired_size = Vec2::new(
            available_size.x.max(min_size.x), // 최소값보다 크면 커지도록
            available_size.y.max(min_size.y),
        );
        let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
        let center = rect.center();
        let radius = desired_size.x.min(desired_size.y) * 0.4;

        let total: f32 = self.value_val.iter().sum();
        let mut start_angle = 0.0;    

        // 파이 섹터 그리기
        for (i, &value) in self.value_val.iter().enumerate() {
            let sweep_angle = (value / total) * 2.0 * PI;
            let end_angle = start_angle + sweep_angle;
    
            // 섹터의 중간 각도를 계산
            let mid_angle = start_angle + (sweep_angle / 2.0);
    
            // 중심에서 약간 이동된 중심점 계산
            let offset_x = mid_angle.cos() * self.sector_gap;
            let offset_y = mid_angle.sin() * self.sector_gap;
            let sector_center = Pos2::new(center.x + offset_x, center.y + offset_y);

            // 클릭을 감지하기 위해 Response를 수집
            self.draw_pie_sector(ui, sector_center, radius, start_angle, end_angle, self.sector_colors[i], self.stroke_colors[i]);

            // 섹터 레이블 표시
            let label_angle = start_angle + sweep_angle / 2.0;

            let label_pos = Pos2::new(
                sector_center.x + label_angle.cos() * radius * 0.7,
                sector_center.y + label_angle.sin() * radius * 0.7,
            );

            let font_id = FontId::proportional(14.0);  // 글자 크기 지정
            ui.painter().text(label_pos, egui::Align2::CENTER_CENTER, &self.label_val[i], font_id, Color32::BLACK);

            start_angle = end_angle; // 다음 섹터 시작점
        }
    }

    fn select_sector_color(&mut self, ui: &mut egui::Ui,) {

        for (i, &_value) in self.value_val.iter().enumerate() {
            // 섹터가 클릭되면 색상 선택 UI와 테두리 굵기 선택 UI를 표시
                ui.vertical(|ui| { 
                    ui.label(egui::RichText::new("Col Name").size(13.0).strong());
                    ui.horizontal(|ui| { 
                        ui.label("Sector Color:");
                        ui.color_edit_button_srgba(&mut self.sector_colors[i]);
                    });
                    ui.horizontal(|ui| { 
                        ui.label("Stroke Color:");
                        ui.color_edit_button_srgba(&mut self.stroke_colors[i]);
                    });
                    ui.add_space(3.0);
                });
        };
    }

    fn _cleaning_data_type(&mut self, table_data: &RecordTable) {
        let l_col = table_data.dataframe.get_column_names()[self.label_col.selected - 1];
        let v_col = table_data.dataframe.get_column_names()[self.value_col.selected - 1];

        let l_series = table_data.dataframe.column(l_col).unwrap();
        let v_series = table_data.dataframe.column(v_col).unwrap();

        self.label_val = l_series
        .cast(&polars::prelude::DataType::String)
        .unwrap()
        .str()
        .map(|ca| ca.into_iter().flatten().map(|v| v.to_string()).collect())
        .unwrap_or_else(|_| Vec::new());
        let value_f64 = cast_data_type_as_f64(v_series);
        self.value_val = value_f64.iter().map(|&x| x as f32).collect();

        // 섹터의 길이에 맞게 색상 벡터를 조정
        let num_sectors = self.value_val.len();
        self.sector_colors.resize(num_sectors, Color32::default());
        self.stroke_colors.resize(num_sectors, Color32::BLACK);
    }
    
    // 파이 섹터 그리기 함수 (클릭 감지용 Response 반환)
    fn draw_pie_sector(&self, ui: &mut egui::Ui, center: Pos2, radius: f32, start_angle: f32, end_angle: f32, color: Color32, stroke_color: Color32) -> egui::Response {
        let segments = 64; // 파이 섹터를 그리기 위한 분할 수
        let angle_step = (end_angle - start_angle) / segments as f32;

        let mut points = vec![center];
        for i in 0..=segments {
            let angle = start_angle + i as f32 * angle_step;
            let x = center.x + angle.cos() * radius;
            let y = center.y + angle.sin() * radius;
            points.push(Pos2::new(x, y));
        }

        // 섹터의 Rect를 기반으로 클릭 감지
        let rect = ui.min_rect();
        let response = ui.interact(rect, ui.id(), egui::Sense::click());

        // 파이 섹터를 그리는 Shape 추가
        let painter = ui.painter();
        painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::new(self.stroke_width, stroke_color)));

        response
    }

    fn _check_data_exist(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Not Found Data.").size(15.0));
        ui.add_space(5.0);
        ui.label(egui::RichText::new("Please Import CSV or Excel File").size(15.0));
    }

}
