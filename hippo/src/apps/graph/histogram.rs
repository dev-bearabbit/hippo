use egui_plot::{BarChart, Bar, Plot};
use crate::models::table::RecordTable;
use crate::apps::util::{Dropbox, cast_data_type_as_f64};

pub struct HistogramGraph {
    pub x_axis: Dropbox,
    pub x_val: Vec<f64>,  // 연속형 데이터는 숫자로 되어 있어야 함
    pub bar_color: egui::Color32,
    pub line_stroke: egui::Stroke,
}

impl HistogramGraph {
    pub fn new() -> Self {
        Self {
            x_axis: Dropbox::new(1),
            x_val: vec![41.93, 60.49, 37.24, 49.70, 52.71, 51.23, 50.63, 49.42, 46.73, 43.49,
                        52.43, 47.16, 43.33, 49.49, 41.52, 56.04, 55.20, 46.79, 56.55, 45.95],
            bar_color: egui::Color32::default(),
            line_stroke: egui::Stroke::new(1.0, egui::Color32::default()),
        }
    }

    pub fn draw_histogram_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable, edit_mode: bool) {
        if edit_mode {
            egui::Frame::default()
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Select Value to Draw Histogram").size(20.0).strong());
                    ui.add_space(5.0);
                    let mut columns = table_data.dataframe.get_column_names();

                    if columns.is_empty() {
                        self._check_data_exist(ui);
                    } else {
                        columns.insert(0, "select");

                        ui.horizontal(|ui| { 
                            ui.label(egui::RichText::new("X axis").size(13.0));
                            self.x_axis.select_column_dropbox(ui, &columns);
                        });

                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Bar color").size(13.0));
                            ui.color_edit_button_srgba(&mut self.bar_color);
                            ui.allocate_space(egui::Vec2::new(10.0, 0.0));
                            ui.label(egui::RichText::new("Line color").size(13.0));
                            ui.color_edit_button_srgba(&mut self.line_stroke.color);
                            ui.allocate_space(egui::Vec2::new(10.0, 0.0));
                            ui.label(egui::RichText::new("Line width").size(13.0));
                            ui.add(egui::Slider::new(&mut self.line_stroke.width, 0.0..=10.0));
                        });
                    }
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);

                    self.set_histogram_chart(ui, table_data);
                });
        } else {
            self.set_histogram_chart(ui, table_data);
        }
    }

    pub fn set_histogram_chart(&mut self, ui: &mut egui::Ui, table_data: &RecordTable) {
        egui::Frame::default()
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                
                if self.x_axis.selected != 0 {
                    self._cleaning_data_type(table_data);
                }

                // 히스토그램 데이터를 구간으로 나눠서 처리
                let data_count = self.x_val.len() as f64;
                let num_bins = (1.0 + 3.322 * data_count.log10()).ceil() as usize;
                // min과 max 값 계산
                let (min, max) = self.x_val.iter().cloned().fold((f64::MAX, f64::MIN), |(min, max), val| {
                    (min.min(val), max.max(val))
                });

                // 구간의 너비 계산
                let bin_width = (max - min) / num_bins as f64;
                let mut bins = vec![0; num_bins];

                // 각 데이터를 적절한 구간에 배치
                for &val in &self.x_val {
                    let bin_idx = ((val - min) / bin_width).floor() as usize;
                    if bin_idx < num_bins {
                        bins[bin_idx] += 1;
                    } else {
                        bins[num_bins - 1] += 1;  // max 값은 마지막 구간에 포함
                    }
                }

                let bars: Vec<Bar> = bins.into_iter()
                    .enumerate()
                    .map(|(i, count)| {
                        let x = min + i as f64 * bin_width + bin_width / 2.0;
                        let y = count as f64;
                        Bar::new(x, y)
                            .fill(self.bar_color)
                            .stroke(self.line_stroke)
                    })
                    .collect();

                let bar_chart = BarChart::new(bars).width(bin_width);

                Plot::new("histogram")
                    .view_aspect(2.0)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .show_axes([true, true])
                    .show_grid([true, true])
                    .show(ui, |plot_ui| {
                        plot_ui.bar_chart(bar_chart);
                    });
            });
    }

    fn _cleaning_data_type(&mut self, table_data: &RecordTable) {
        let x_col = table_data.dataframe.get_column_names()[self.x_axis.selected - 1];

        let x_series = table_data.dataframe.column(x_col).unwrap();

        self.x_val = cast_data_type_as_f64(x_series);
    }

    fn _check_data_exist(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Not Found Data.").size(15.0));
        ui.add_space(5.0);
        ui.label(egui::RichText::new("Please Import CSV or Excel File").size(15.0));
    }
}
