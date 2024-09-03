use hippo::apps::menu::Menu;
use hippo::apps::{file,layout};
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
    menu: Menu
}

impl eframe::App for Hippo {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {        
        layout::custom_window_frame(ctx, "Hippo", |ui| {
            self.menu.update_menu_bar(ui);

            ui.horizontal(|ui| {
                // 사이드바 영역
                ui.vertical(|ui| {
                    let sidebar_width = 200.0; // 사이드바의 고정된 너비
                    let available_height = ui.available_height(); // 사용 가능한 전체 높이
    
                    // 고정된 크기의 사이드바
                    ui.allocate_exact_size(
                        egui::vec2(sidebar_width, available_height),
                        egui::Sense::hover(),
                    );

                    if self.menu.table_data.dataframe.width() != 0 {
                        file::load_data(&self.menu.table_data, ui);
                    }

                });
                //side bar separator
                ui.separator();
            });
        });
    }
}
