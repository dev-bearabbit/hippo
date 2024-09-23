use plotters::prelude::*;
use plotters_bitmap::BitMapBackend;
use image::{ImageBuffer, Rgba};
use eframe::egui::TextureHandle;

pub struct Graph {
    texture: Option<TextureHandle>,
}

impl Graph {

    pub fn new() -> Self {
        Self {
            texture: None
        }
    }

    pub fn draw_line_chart(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

            ui.heading("Plotters Line Chart Example");

            if self.texture.is_none() {
                // plotter 모듈의 함수를 호출하여 메모리 내에서 이미지 생성
                let (image, width, height) = draw_line_chart_to_memory().expect("Failed to draw line chart");

                // 이미지 데이터를 egui의 Texture로 변환
                let size = [width as usize, height as usize];
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image);
                let texture = ctx.load_texture("plotters_image", color_image, egui::TextureOptions::default());
                self.texture = Some(texture);
            }

            if let Some(texture) = &self.texture {
                // egui 창에 이미지 그리기
                ui.image(texture);
            };
    }
}

// 메모리 상에서 이미지로 그리기
fn draw_line_chart_to_memory() -> Result<(Vec<u8>, u32, u32), Box<dyn std::error::Error>> {
    let width = 500;
    let height = 300;

    // 1. 메모리 버퍼에 직접 그리기
    let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    {
    let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Line Chart", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..10, 0..100)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (0..10).map(|x| (x, x * x)),
        &RED,
    ))?;

    root.present()?;
    }

    // 2. 버퍼에서 이미지 데이터를 Vec<u8>로 변환
    let raw_image_data = buffer.into_raw();

    // 3. 이미지 데이터와 크기 반환
    Ok((raw_image_data, width, height))
}