pub struct Custom {
    text: String,
    size: f32,
    bold: bool
}

impl Custom {

    pub fn new() -> Self {
        Self {
            text: String::new(),
            size: 0.0,
            bold: false
        }
    }

    pub fn set_up_text_layout(&mut self, ui: &mut egui::Ui) {

        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Write Text").size(20.0).strong());
            ui.add(egui::Slider::new(&mut self.size, 10.0..=100.0).text("Text Size"));
            if ui.checkbox(&mut self.bold, "Bold").clicked() {
                self.bold = true;
            }
            ui.add_space(5.0);
            ui.text_edit_multiline(&mut self.text);

        });
    }

    pub fn result_text(&mut self, ui: &mut egui::Ui) {

        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.add_space(5.0);
            if self.bold {
                ui.label(egui::RichText::new(&self.text).size(self.size).strong());
            } else {
                ui.label(egui::RichText::new(&self.text).size(self.size));
            }
            ui.add_space(5.0);

        });

    }
}