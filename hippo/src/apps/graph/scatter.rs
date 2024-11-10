use egui::{Ui, Frame, Margin, RichText};
use egui_plot::{Plot, Points};
use crate::models::table::RecordTable;
use crate::apps::util::{Dropbox, cast_data_type_as_f64};

pub struct ScatterGraph {
    pub x_axis: Dropbox,
    pub y_axis: Dropbox,
    pub x_val: Vec<f64>,
    pub y_val: Vec<f64>,
    pub point_color: [f32; 3],
    pub line_color: [f32; 3],
    pub point_size: f32,
    pub line_width: f32,
}

impl ScatterGraph {
    pub fn new() -> Self {
        Self {
            x_axis: Dropbox::new(1),
            y_axis: Dropbox::new(2),
            x_val: vec![0.0, 1.0, 2.0, 3.0, 5.0, 6.0, 7.0, 8.0],
            y_val: vec![3.0, 2.0, 1.0, 4.0, 5.0, 3.0, 2.0, 4.0],
            point_color: [0.9703065, 0.8227896, 0.8227896],
            line_color: [0.6352784, 0.13847084, 0.13847084],
            point_size: 5.0,
            line_width: 1.0,
        }
    }

    pub fn draw_scatter_chart(&mut self, ui: &mut Ui, table_data: &RecordTable, edit_mode: bool) {
        if edit_mode {
            Frame::default()
                .inner_margin(Margin::same(10.0)) // 패딩 설정
                .show(ui, |ui| {
                    ui.label(RichText::new("Select Value to Draw").size(20.0).strong());
                    let mut columns = table_data.dataframe.get_column_names();

                    if columns.is_empty() {
                        self._check_data_exist(ui);
                    } else {
                        columns.insert(0, "select");
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("X axis").size(15.0));
                            self.x_axis.select_column_dropbox(ui, &columns);

                            ui.label(RichText::new("Y axis").size(15.0));
                            self.y_axis.select_column_dropbox(ui, &columns);
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Point color").size(13.0));
                            ui.color_edit_button_rgb(&mut self.point_color);
                            ui.label(egui::RichText::new("Line color").size(13.0));
                            ui.color_edit_button_rgb(&mut self.line_color);
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Point Size").size(13.0));
                            ui.add(egui::Slider::new(&mut self.point_size, 0.0..=10.0));
                            ui.label(egui::RichText::new("Line Width").size(13.0));
                            ui.add(egui::Slider::new(&mut self.line_width, 0.0..=10.0));
                        });
                    }
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);

                    self.set_scatter_chart(ui, table_data);
                });
        } else {
            self.set_scatter_chart(ui, table_data);
        }
    }

    pub fn set_scatter_chart(&mut self, ui: &mut Ui, table_data: &RecordTable) {
        Frame::default()
            .inner_margin(Margin::same(10.0)) // 패딩 설정
            .show(ui, |ui| {
                if self.x_axis.selected != 0 && self.y_axis.selected != 0 {
                    self._cleaning_data_type(table_data);
                }

                let x_val = &self.x_val;
                let y_val = &self.y_val;
                let min_len = x_val.len().min(y_val.len());

                // 점(point) 데이터 생성
                let points: Vec<[f64; 2]> = x_val
                    .iter()
                    .zip(y_val.iter())
                    .take(min_len)
                    .map(|(&x, &y)| [x, y])
                    .collect();

                // 외곽선 역할을 하는 큰 점 (바깥 점)
                let outer_points = Points::new(points.clone()) 
                    .radius(self.point_size + self.line_width)  // 외곽선 크기
                    .color(egui::Color32::from_rgb(
                        (self.line_color[0] * 255.0).clamp(0.0, 255.0) as u8,
                        (self.line_color[1] * 255.0).clamp(0.0, 255.0) as u8,
                        (self.line_color[2] * 255.0).clamp(0.0, 255.0) as u8,
                    ))         // 외곽선 색상
                    .name("Outer Points");

                // 내부 색상 역할을 하는 작은 점 (안쪽 점)
                let inner_points = Points::new(points)
                    .radius(self.point_size)        // 내부 점 크기
                    .color(egui::Color32::from_rgb(
                        (self.point_color[0] * 255.0).clamp(0.0, 255.0) as u8,
                        (self.point_color[1] * 255.0).clamp(0.0, 255.0) as u8,
                        (self.point_color[2] * 255.0).clamp(0.0, 255.0) as u8,
                    ))        // 내부 점 색상
                    .name("Inner Points");

                Plot::new("scatter_plot")
                    .view_aspect(2.0)
                    .allow_zoom(true)
                    .allow_scroll(true)
                    .show(ui, |plot_ui| {
                        plot_ui.points(outer_points);  // 외곽선 역할
                        plot_ui.points(inner_points);  // 내부점 역할
                    });
            });
    }

    fn _cleaning_data_type(&mut self, table_data: &RecordTable) {
        let x_col = table_data.dataframe.get_column_names()[self.x_axis.selected - 1];
        let y_col = table_data.dataframe.get_column_names()[self.y_axis.selected - 1];

        let x_series = table_data.dataframe.column(x_col).unwrap();
        let y_series = table_data.dataframe.column(y_col).unwrap();

        self.x_val = cast_data_type_as_f64(x_series);
        self.y_val =cast_data_type_as_f64(y_series);
    }

    fn _check_data_exist(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Not Found Data.").size(15.0));
        ui.add_space(5.0);
        ui.label(RichText::new("Please Import CSV or Excel File").size(15.0));
    }
}
