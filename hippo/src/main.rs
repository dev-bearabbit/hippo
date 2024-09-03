use hippo::apps::menu::Menu;
use hippo::apps::{sidebar,layout};
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

            sidebar::custom_sidebar(ui);
        });
    }
}
