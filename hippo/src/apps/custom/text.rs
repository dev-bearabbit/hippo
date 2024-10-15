pub struct TextCustom {
    pub text: String,
    pub size: f32,
    pub bold: bool,
    pub border: bool
}

impl TextCustom {

    pub fn new() -> Self {
        Self {
            text: String::new(),
            size: 0.0,
            bold: false,
            border: false
        }
    }

    pub fn set_up_text_layout(&mut self, ui: &mut egui::Ui, edit_mode: bool) {

        if edit_mode {
            egui::Frame::default()
                .inner_margin(egui::Margin::same(10.0)) // 패딩 설정
                .show(ui, |ui| {
                    
                    ui.vertical_centered_justified(|ui| {
                        ui.label(egui::RichText::new("Write Text").size(20.0).strong());
                        ui.add_space(15.0);
            
                        ui.horizontal(|ui| { 
                            ui.add(egui::Slider::new(&mut self.size, 10.0..=100.0)
                                .logarithmic(true)
                                .text("Text Size"));
                            
                            ui.checkbox(&mut self.bold, "Bold");
                        });
            
                        ui.add_space(5.0);
                        ui.add(egui::TextEdit::multiline(&mut self.text)
                        .desired_rows(10));
                    });
                });
            } else {
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
}