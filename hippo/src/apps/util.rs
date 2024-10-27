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

#[derive(Debug)]
pub enum CalculateType {
    Sum,
    Count,
    Mean,
    Median,
    Max,
    Min,
    Nunique,
    First,
    Last
}

impl CalculateType {

    pub fn list_calculate_types(&self) -> Vec<&str> {
        vec!["select", "count", "sum", "mean", "median", "max", "min", "nunique", "first", "last"]
    }

    pub fn execute(&mut self, x_series: Series, y_series: Series) ->  (Vec<String>, Vec<f32>) {

        let df = DataFrame::new(vec![x_series, y_series]).unwrap();
        let lazy_df = df.lazy();
    
        // 그룹화한 결과를 CalculateType에 따라 계산
        let result_series = match self {
            CalculateType::Count => lazy_df.group_by([col("x_series")]).agg([col("y_series").count().alias("result")]),
            CalculateType::Sum => lazy_df.group_by([col("x_series")]).agg([col("y_series").sum().alias("result")]),
            CalculateType::Mean => lazy_df.group_by([col("x_series")]).agg([col("y_series").mean().alias("result")]),
            CalculateType::Median => lazy_df.group_by([col("x_series")]).agg([col("y_series").median().alias("result")]),
            CalculateType::Max => lazy_df.group_by([col("x_series")]).agg([col("y_series").max().alias("result")]),
            CalculateType::Min => lazy_df.group_by([col("x_series")]).agg([col("y_series").min().alias("result")]),
            CalculateType::Nunique => lazy_df.group_by([col("x_series")]).agg([col("y_series").n_unique().alias("result")]),
            CalculateType::First => lazy_df.group_by([col("x_series")]).agg([col("y_series").first().alias("result")]),
            CalculateType::Last => lazy_df.group_by([col("x_series")]).agg([col("y_series").last().alias("result")]),

        };

        let result_df = result_series.collect().unwrap();

        let x_vals: Vec<String> = result_df
            .column("x_val")
            .unwrap()
            .cast(&polars::prelude::DataType::String)
            .unwrap()
            .str()
            .map(|ca| ca.into_iter().flatten().map(|v| v.to_string()).collect())
            .unwrap_or_else(|_| Vec::new());

        let y_vals: Vec<f32> = result_df
            .column("result")
            .unwrap()
            .f32()
            .map(|ca| ca.into_iter().flatten().collect())
            .unwrap_or_else(|_| Vec::new());

        (x_vals, y_vals)
    }
}

impl TryFrom<&str> for CalculateType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "sum" => Ok(CalculateType::Sum),
            "count" => Ok(CalculateType::Count),
            "mean" => Ok(CalculateType::Mean),
            "median" => Ok(CalculateType::Median),
            "max" => Ok(CalculateType::Max),
            "min" => Ok(CalculateType::Min),
            "nunique" => Ok(CalculateType::Nunique),
            "first" => Ok(CalculateType::First),
            "last" => Ok(CalculateType::Last),
            _ => Err("Invalid calculation type"),
        }
    }
}
