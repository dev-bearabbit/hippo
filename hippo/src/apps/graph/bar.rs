use egui_plot::{BarChart, Bar, Plot, Text};
use crate::models::table::RecordTable;
use crate::apps::util::Dropbox;


pub struct BarGraph {
    pub x_axis: Dropbox,
    pub y_axis: Dropbox,
    pub x_val: Vec<String>,
    pub y_val: Vec<f64>,
}

impl BarGraph {
    pub fn new() -> Self {
        Self {
            x_axis: Dropbox::new(1),
            y_axis: Dropbox::new(2),
            x_val: vec!["A", "B", "C", "D", "E", "F", "G", "N"].into_iter().map(|s| s.to_string()).collect(),
            y_val: vec![3.0, 2.0, 1.0, 4.0, 5.0, 3.0, 2.0, 4.0],
        }
    }
    pub fn draw_bar_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable, edit_mode: bool) {
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

                    ui.horizontal(|ui| { 
                        ui.label(egui::RichText::new("X axis").size(15.0));
                        self.x_axis.select_column_dropbox(ui, &columns);

                        ui.label(egui::RichText::new("Y axis").size(15.0));
                        self.y_axis.select_column_dropbox(ui, &columns);
                    });
                }
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
        
                self.set_bar_chart(ui, table_data);
            });
        } else {
            self.set_bar_chart(ui, table_data);
        }
    }

    pub fn set_bar_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable) {
        egui::Frame::default()
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                if self.x_axis.selected != 0 && self.y_axis.selected != 0 {
                    self._cleaning_data_type(table_data);
                }

                let min_len = self.x_val.len().min(self.y_val.len());
                let bars: Vec<Bar> = (0..min_len)
                .map(|i| {
                    let x = i as f64;
                    let y = self.y_val[i];
                    Bar::new(x, y)  // Bar 생성
                }).collect();

                let bar_chart = BarChart::new(bars).width(0.5);

                Plot::new("bar_chart")
                    .view_aspect(2.0)
                    .allow_zoom(false)  // 줌 비활성화 (선택 사항)
                    .allow_scroll(false)
                    .show_axes([false, true])
                    .show_grid([false, true])
                    .show(ui, |plot_ui| {
                        plot_ui.bar_chart(bar_chart);

                        // X축에 문자열 레이블 추가
                        for (i, label) in self.x_val.iter().enumerate() {
                            let x = i as f64;
                            let y = -0.5;  // 막대 아래에 레이블 위치

                            plot_ui.text(Text::new([x, y].into(), label.clone()));
                        }
                    });
                });
        }

    fn _cleaning_data_type(&mut self, table_data: &RecordTable) {
        let x_col = table_data.dataframe.get_column_names()[self.x_axis.selected - 1];
        let y_col = table_data.dataframe.get_column_names()[self.y_axis.selected - 1];

        let x_series = table_data.dataframe.column(x_col).unwrap();
        let y_series = table_data.dataframe.column(y_col).unwrap();

        self.x_val = x_series
        .cast(&polars::prelude::DataType::String)
        .unwrap()
        .str()
        .map(|ca| ca.into_iter().flatten().map(|v| v.to_string()).collect())
        .unwrap_or_else(|_| Vec::new());

        self.y_val = y_series
            .i64()
            .map(|ca| ca.into_iter().flatten().map(|v| v as f64).collect())
            .unwrap_or_else(|_| Vec::new());
    }

    fn _check_data_exist(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Not Found Data.").size(15.0));
        ui.add_space(5.0);
        ui.label(egui::RichText::new("Please Import CSV or Excel File").size(15.0));
    }
}