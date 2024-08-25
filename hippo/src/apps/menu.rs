

pub fn theme_light_dark_mode(ui: &mut egui::Ui) {
    ui.label("LIGHT/DARK MODE");
    ui.horizontal(|ui| {
        egui::widgets::global_dark_light_mode_buttons(ui);
    });
}