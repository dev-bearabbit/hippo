use egui_plot::{Line, Plot, PlotPoints};
use crate::models::table::RecordTable;
use crate::apps::util::Dropbox;

pub struct LineGraph {
    pub x_axis: Dropbox,
    pub y_axis: Dropbox,
    pub x_val: Vec<f64>,
    pub y_val: Vec<f64>,
    pub line_color: egui::Color32,
    pub line_width: f32,
}


impl LineGraph {
    pub fn new() -> Self {
        Self {
            x_axis: Dropbox::new(1),
            y_axis: Dropbox::new(2),
            x_val: vec![0.0, 1.0, 2.0, 3.0, 5.0, 6.0, 7.0, 8.0],
            y_val: vec![3.0, 2.0, 1.0, 4.0, 5.0, 3.0, 2.0, 4.0],
            line_color: egui::Color32::default(),
            line_width: 2.0,
        }
    }

    pub fn draw_line_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable, edit_mode: bool) {

        if edit_mode {
            egui::Frame::default()
            .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
            .show(ui, |ui| {

                ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
                ui.add_space(10.0);
                let mut columns = table_data.dataframe.get_column_names();

                if columns.is_empty() {
                    self._check_data_exist(ui);
                } else {
                    columns.insert(0, "select");
                    ui.horizontal(|ui| { 
                        ui.label(egui::RichText::new("X axis").size(13.0));
                        self.x_axis.select_column_dropbox(ui, &columns);
                        ui.allocate_space(egui::Vec2::new(10.0, 0.0));
                        ui.label(egui::RichText::new("Y axis").size(13.0));
                        self.y_axis.select_column_dropbox(ui, &columns);
                    });
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Line color").size(13.0));
                        ui.color_edit_button_srgba(&mut self.line_color);
                        ui.allocate_space(egui::Vec2::new(10.0, 0.0));
                        ui.label(egui::RichText::new("Line width").size(13.0));
                        ui.add(egui::Slider::new(&mut self.line_width, 0.0..=10.0));
                    });

                }
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
        
                self.set_line_chart(ui,table_data);
            });
        } else {
            self.set_line_chart(ui,table_data);
        }
    }

    pub fn set_line_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable) {

        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            if self.x_axis.selected != 0 && self.y_axis.selected != 0 {
                self._cleaning_data_type(table_data);
            }

            let x_val = &self.x_val;
            let y_val = &self.y_val;

            let min_len = x_val.len().min(y_val.len());
            let points: PlotPoints = x_val
                .into_iter()
                .zip(y_val.into_iter())
                .take(min_len)
                .map(|(&x, &y)| [x, y])
                .collect();

            let line = Line::new(points)
                .color(self.line_color)
                .width(self.line_width);

            Plot::new("line_chart")
                .view_aspect(2.0)
                .allow_zoom(false)  // 줌 비활성화 (선택 사항)
                .allow_scroll(false)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
            });
        });
    }

    fn _cleaning_data_type(&mut self, table_data: &RecordTable) {

        let x_col = table_data.dataframe.get_column_names()[self.x_axis.selected-1];
        let y_col = table_data.dataframe.get_column_names()[self.y_axis.selected-1];

        let x_series = table_data.dataframe.column(x_col).unwrap();
        let y_series = table_data.dataframe.column(y_col).unwrap();

        self.x_val = x_series
        .i64()  // i64 타입으로 가져옴
        .map(|ca| ca.into_iter().flatten().map(|v| v as f64).collect())  // f64로 변환
        .unwrap_or_else(|_| Vec::new());

        self.y_val = y_series
        .i64()  // i64 타입으로 가져옴
        .map(|ca| ca.into_iter().flatten().map(|v| v as f64).collect())  // f64로 변환
        .unwrap_or_else(|_| Vec::new());
    }

    fn _check_data_exist(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Not Found Data.").size(15.0));
        ui.add_space(5.0);
        ui.label(egui::RichText::new("please Import CSV or Excel File").size(15.0));
    }
}
