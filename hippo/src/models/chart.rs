use eframe::egui;
use crate::apps::graph::Graph;
use crate::apps::custom::Custom;
use crate::models::table::RecordTable;

#[derive(PartialEq)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Scatter,
    Histogram,
    Text,
    Image,
    Table
}

// 차트 타입에 따른 로직을 처리하는 함수
impl ChartType {
    pub fn execute(&self, ui: &mut egui::Ui, custom :&mut Custom, graph:&mut Graph, table_data: &RecordTable, edit_mode: bool) {
        match self {
            ChartType::Line => self.render_line_chart(ui, table_data, edit_mode),
            ChartType::Bar => self.render_bar_chart(ui),
            ChartType::Pie => self.render_pie_chart(ui),
            ChartType::Scatter => self.render_scatter_chart(ui),
            ChartType::Histogram => self.render_histogram_chart(ui),
            ChartType::Text => self.render_text_layout(ui, custom, edit_mode),
            ChartType::Image => self.render_image_layout(ui),
            ChartType::Table => self.render_table_layout(ui),
        }
    }

    fn render_line_chart(&self, ui: &mut egui::Ui, table_data: &RecordTable, edit_mode: bool) {
        if edit_mode {
            Graph::new().draw_line_chart(ui, table_data);
        } else {
            Graph::new().set_line_chart(ui, table_data);
        }
    }

    fn render_bar_chart(&self, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new("This is Bar Chart").size(15.0));
 
        });
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

    fn render_scatter_chart(&self, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(5.0);
            ui.label(egui::RichText::new("This is Scatter Chart").size(15.0));
 
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

    fn render_text_layout(&self, ui: &mut egui::Ui, custom: &mut Custom, edit_mode: bool) {
        if edit_mode {
            custom.set_up_text_layout(ui);
        } else {
            custom.result_text(ui);
        }
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
