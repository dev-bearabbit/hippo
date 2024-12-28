use polars::series::Series;
use polars::datatypes::DataType;
use polars::prelude::DataFrame;
use polars::prelude::IntoLazy;
use polars::prelude::col;

pub struct Dropbox {
    pub id: usize,
    pub selected: usize,
}

impl Dropbox {
    pub fn new(id: usize) -> Self {
        Self {
            id: id,
            selected: 0,
        }
    }

    pub fn select_column_dropbox(&mut self, ui: &mut egui::Ui, columns: &Vec<&str>) {

        egui::ComboBox::new(self.id, "")
            .selected_text(columns[self.selected])
            .show_ui(ui, |ui| {
                for (i, &column) in columns.iter().enumerate() {
                    ui.selectable_value(&mut self.selected, i, column);
                }
            });
    }
}

pub fn cast_data_type_as_f64(col_data :&Series) -> Vec<f64> {

    let mut result = Vec::new();
    match col_data.dtype() {
        DataType::Int64 => {
            result = col_data.i64()
                .map(|ca| ca.into_iter().flatten().map(|v| v as f64).collect())
                .unwrap_or_else(|_| Vec::new());
            result
        }
        DataType::Float64 => {
            result = col_data.f64()
                .map(|ca| ca.into_iter().flatten().collect())
                .unwrap_or_else(|_| Vec::new());
            result
        },
        // 그 외 타입은 빈 벡터 반환
        _ => result,
    }
}

// TODO: 데이터 타입 이슈 해결 요망 
pub fn calculate_pie_value(x_series: &Series, y_series: &Series, mhd: &str) ->  (Vec<String>, Vec<f32>) {

        let x_name = x_series.name();
        let y_name = y_series.name();

        let df = DataFrame::new(vec![x_series.clone(), y_series.clone()]).unwrap();
        let lazy_df = df.lazy();
        let result_series = match mhd {
            "count" => lazy_df.group_by([col(x_name)]).agg([col(y_name).count().alias("result")]),
            "sum" => lazy_df.group_by([col(x_name)]).agg([col(y_name).sum().alias("result")]),
            "mean" => lazy_df.group_by([col(x_name)]).agg([col(y_name).mean().alias("result")]),
            "median" => lazy_df.group_by([col(x_name)]).agg([col(y_name).median().alias("result")]),
            "max" => lazy_df.group_by([col(x_name)]).agg([col(y_name).max().alias("result")]),
            "min" => lazy_df.group_by([col(x_name)]).agg([col(y_name).min().alias("result")]),
            "nunique" => lazy_df.group_by([col(x_name)]).agg([col(y_name).n_unique().alias("result")]),
            "first" => lazy_df.group_by([col(x_name)]).agg([col(y_name).first().alias("result")]),
            "last" => lazy_df.group_by([col(x_name)]).agg([col(y_name).last().alias("result")]),
            "select" => lazy_df,
            _ => lazy_df,
        };

        let result_df = result_series.collect().unwrap();

        let x_vals: Vec<String> = result_df
            .column(x_name)
            .unwrap()
            .cast(&polars::prelude::DataType::String)
            .unwrap()
            .str()
            .map(|ca| ca.into_iter().flatten().map(|v| v.to_string()).collect())
            .unwrap_or_else(|_| Vec::new());

        println!("{:?}", result_df.column("result").unwrap());

        let y_vals: Vec<f32> = result_df
            .column("result")
            .unwrap()
            .u32()
            .map(|ca| ca.into_iter().flatten().map(|v| v as f32).collect())
            .unwrap_or_else(|_| Vec::new());

        (x_vals, y_vals)
    }
