
pub fn update_menu_bar( ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.menu_button("Click for menu", nested_menu);

        ui.button("Right-click for menu")
            .context_menu(nested_menu);

        if ui.ctx().is_context_menu_open() {
            ui.label("Context menu is open");
        } else {
            ui.label("Context menu is closed");
        }
        theme_light_dark_mode(ui)
    });
}

fn theme_light_dark_mode(ui: &mut egui::Ui) {
    ui.label("LIGHT/DARK MODE");
    ui.horizontal(|ui| {
        egui::widgets::global_dark_light_mode_buttons(ui);
    });
}

fn nested_menu(ui: &mut egui::Ui) {
    ui.set_max_width(200.0); // To make sure we wrap long text

    if ui.button("Open…").clicked() {
        ui.close_menu();
    }
    ui.menu_button("SubMenu", |ui| {
        ui.menu_button("SubMenu", |ui| {
            if ui.button("Open…").clicked() {
                ui.close_menu();
            }
            let _ = ui.button("Item");
        });
        ui.menu_button("SubMenu", |ui| {
            if ui.button("Open…").clicked() {
                ui.close_menu();
            }
            let _ = ui.button("Item");
        });
        let _ = ui.button("Item");
        if ui.button("Open…").clicked() {
            ui.close_menu();
        }
    });
    ui.menu_button("SubMenu", |ui| {
        let _ = ui.button("Item1");
        let _ = ui.button("Item2");
        let _ = ui.button("Item3");
        let _ = ui.button("Item4");
        if ui.button("Open…").clicked() {
            ui.close_menu();
        }
    });
    let _ = ui.button("Very long text for this item that should be wrapped");
}