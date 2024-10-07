pub struct Custom {
    text: String,

}

impl Custom {

    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn set_up_text_layout(&mut self, ui: &mut egui::Ui) {

        egui::Frame::default()
        .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
        .show(ui, |ui| {

            ui.label(egui::RichText::new("Write Text").size(20.0).strong());
            ui.add_space(5.0);
            ui.text_edit_singleline(&mut self.text);

        });
    }
}