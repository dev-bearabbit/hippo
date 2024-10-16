use eframe::egui;
use crate::apps::graph::line::LineGraph;
use crate::apps::graph::bar::BarGraph;
use crate::apps::graph::scatter::ScatterGraph;
use crate::apps::custom::text::TextCustom;
use crate::models::table::RecordTable;


pub enum ChartType {
    Line(LineGraph),
    Bar(BarGraph),
    Pie,
    Scatter(ScatterGraph),
    Histogram,
    Text(TextCustom),
    Image,
    Table
}

// 차트 타입에 따른 로직을 처리하는 함수
impl ChartType {
    pub fn execute(&mut self, ui: &mut egui::Ui, table_data: &RecordTable, edit_mode: bool) {
        match self {
            ChartType::Line(graph) => graph.draw_line_chart(ui, table_data, edit_mode),
            ChartType::Bar(graph) => graph.draw_bar_chart(ui, table_data, edit_mode),
            ChartType::Pie => self.render_pie_chart(ui),
            ChartType::Scatter(graph) => graph.draw_scatter_chart(ui, table_data, edit_mode),
            ChartType::Histogram => self.render_histogram_chart(ui),
            ChartType::Text(graph) => graph.set_up_text_layout(ui, edit_mode),
            ChartType::Image => self.render_image_layout(ui),
            ChartType::Table => self.render_table_layout(ui),
        }
    }

    fn render_pie_chart(&self, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new("This is Pie Chart").size(15.0));
 
        });
    }

    fn render_histogram_chart(&self, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new("This is Histogram Chart").size(15.0));
 
        });
    }

    fn render_image_layout(&self, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new("This is Histogram Chart").size(15.0));
 
        });
    }

    fn render_table_layout(&self, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new("This is Histogram Chart").size(15.0));
 
        });
    }
}
