use eframe::egui;
use crate::apps::graph::Graph;
use crate::models::table::RecordTable;

pub enum ChartType {
    Line,
    Bar,
    Pie,
    Scatter,
    Histogram
}

// 차트 타입에 따른 로직을 처리하는 함수
impl ChartType {
    pub fn execute(&self, ctx: &egui::Context, ui: &mut egui::Ui, table_data: &RecordTable) {
        match self {
            ChartType::Line => self.render_line_chart(ctx, ui, table_data),
            ChartType::Bar => self.render_bar_chart(ctx, ui),
            ChartType::Pie => self.render_pie_chart(ctx, ui),
            ChartType::Scatter => self.render_scatter_chart(ctx, ui),
            ChartType::Histogram => self.render_histogram_chart(ctx, ui),
        }
    }

    fn render_line_chart(&self, ctx: &egui::Context, ui: &mut egui::Ui, table_data: &RecordTable) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new(table_data.dataframe.get_column_names().concat()).size(15.0));
            ui.add_space(5.0);
            ui.separator();
            Graph::new().draw_line_chart(ctx, ui);
        
        });
    }

    fn render_bar_chart(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new("This is Bar Chart").size(15.0));
 
        });
    }

    fn render_pie_chart(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new("This is Pie Chart").size(15.0));
 
        });
    }

    fn render_scatter_chart(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new("This is Scatter Chart").size(15.0));
 
        });
    }

    fn render_histogram_chart(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new("This is Histogram Chart").size(15.0));
 
        });
    }
}
