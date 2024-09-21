use eframe::egui;
use polars::prelude::*;
use plotters::prelude::*;

pub enum ChartType {
    Line,
    Bar,
    Pie,
    Scatter,
    Histogram
}

// 차트 타입에 따른 로직을 처리하는 함수
impl ChartType {
    pub fn execute(&self, ui: &mut egui::Ui) {
        match self {
            ChartType::Line => self.render_line_chart(ui),
            ChartType::Bar => self.render_bar_chart(ui),
            ChartType::Pie => self.render_pie_chart(ui),
            ChartType::Scatter => self.render_scatter_chart(ui),
            ChartType::Histogram => self.render_histogram_chart(ui),
        }
    }

    fn render_line_chart(&self, ui: &mut egui::Ui) {
        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Select Value to Draw").size(20.0).strong());
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("This is Line Chart").size(15.0));
                
            });
        });
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
}
