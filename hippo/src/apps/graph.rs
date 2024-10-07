use egui_plot::{Line, Plot, PlotPoints};
use crate::models::table::RecordTable;
use crate::apps::util;

pub struct Graph {
}

impl Graph {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn draw_line_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable) {

        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);

            let col_cnt = table_data.dataframe.get_column_names().len();
            if col_cnt == 0 {
                self._check_data_exist(ui);
            } else {
                ui.label(egui::RichText::new("TEST").size(15.0));
                util::select_column_dropbox(ui, table_data.dataframe.get_column_names())
            }
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            self.set_line_chart(ui, table_data);

        });
    }

    pub fn set_line_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable) {

        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

        let line_points = PlotPoints::from_explicit_callback(|x| x.sin(), -std::f64::consts::PI..std::f64::consts::PI, 100);
        let line = Line::new(line_points);

        Plot::new("example_plot")
            .view_aspect(2.0) // 그래프의 가로 세로 비율 설정
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
        });
    }


    fn _check_data_exist(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Not Found Data.").size(15.0));
        ui.add_space(5.0);
        ui.label(egui::RichText::new("please Import CSV or Excel File").size(15.0));
    }
}
