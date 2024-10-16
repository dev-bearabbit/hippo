use egui::{Ui, Frame, Margin, RichText};
use egui_plot::{Plot, PlotPoints, Points};
use crate::models::table::RecordTable;
use crate::apps::util::Dropbox;

pub struct ScatterGraph {
    pub x_axis: Dropbox,
    pub y_axis: Dropbox,
    pub x_val: Vec<f64>,
    pub y_val: Vec<f64>,
}

impl ScatterGraph {
    pub fn new() -> Self {
        Self {
            x_axis: Dropbox::new(1),
            y_axis: Dropbox::new(2),
            x_val: vec![0.0, 1.0, 2.0, 3.0, 5.0, 6.0, 7.0, 8.0],
            y_val: vec![3.0, 2.0, 1.0, 4.0, 5.0, 3.0, 2.0, 4.0],
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
                let points: PlotPoints = x_val
                    .iter()
                    .zip(y_val.iter())
                    .take(min_len)
                    .map(|(&x, &y)| [x, y])
                    .collect();

                // Points 객체를 생성해 점만 표시
                let scatter_points = Points::new(points)
                    .radius(5.0)  // 점의 크기 설정
                    .name("Scatter Plot");

                Plot::new("scatter_plot")
                    .view_aspect(2.0)
                    .allow_zoom(true)
                    .allow_scroll(true)
                    .show(ui, |plot_ui| {
                        plot_ui.points(scatter_points);
                    });
            });
    }

    fn _cleaning_data_type(&mut self, table_data: &RecordTable) {
        let x_col = table_data.dataframe.get_column_names()[self.x_axis.selected - 1];
        let y_col = table_data.dataframe.get_column_names()[self.y_axis.selected - 1];

        let x_series = table_data.dataframe.column(x_col).unwrap();
        let y_series = table_data.dataframe.column(y_col).unwrap();

        self.x_val = x_series
            .i64() // i64로 변환
            .map(|ca| ca.into_iter().flatten().map(|v| v as f64).collect()) // f64로 변환
            .unwrap_or_else(|_| Vec::new());

        self.y_val = y_series
            .i64()
            .map(|ca| ca.into_iter().flatten().map(|v| v as f64).collect())
            .unwrap_or_else(|_| Vec::new());
    }

    fn _check_data_exist(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Not Found Data.").size(15.0));
        ui.add_space(5.0);
        ui.label(RichText::new("Please Import CSV or Excel File").size(15.0));
    }
}
