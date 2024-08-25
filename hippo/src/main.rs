use hippo::apps::{file, menu, layout};
use hippo::models::format::Table;

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([1200.0, 800.0])
            .with_transparent(true),

        ..Default::default()
    };
    eframe::run_native(
        "Hippo",
        options,
        Box::new(|_cc| Ok(Box::<Hippo>::default())),
    )
}

#[derive(Default)]
struct Hippo {
    table_data: Table,
}

impl eframe::App for Hippo {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        layout::custom_window_frame(ctx, "Hippo", |ui| {
            menu::update_menu_bar(ui);

            ui.label("FILE OPEN TEST");

            if ui.button("Open File").clicked() {
                self.table_data = file::open_file_to_table();
            }

            if !self.table_data.header.is_empty() {
                file::load_data(&self.table_data, ui);
            }
        });

    }
}