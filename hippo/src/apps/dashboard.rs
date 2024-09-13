use eframe::egui;
pub struct Dashboard {
    pub id: usize,
    pub viewport: bool,
    pub window_pos: egui::Pos2,
}

impl Dashboard {

    pub fn new(id: usize) -> Self {
        Self {
            id,
            viewport: true,
            window_pos: egui::Pos2::new(500.0, 500.0),
        }
    }

    pub fn update_dashboard(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

        egui::CentralPanel::default().show_inside(ui, |ui| {

            if self.viewport {
                // CentralPanel의 경계를 가져옴
                let central_panel_rect = ui.max_rect();

                // 창을 표시
                let window_response = egui::Window::new(format!("Immediate Viewport {}", self.id))
                    .id(egui::Id::new(self.id))
                    .current_pos(self.window_pos)  // 저장된 위치에 창을 표시
                    .resizable(true)  // 창 크기 조절 가능
                    .title_bar(false)
                    .enabled(true)
                    .show(ctx, |ui| {
                        ui.label("Hello from immediate viewport");

                        if ui.button("Close").clicked() {
                            self.viewport = false;
                        }
                    });

                // 창의 위치 제한
                if let Some(window_rect) = window_response.map(|r| r.response.rect) {
                    let mut new_pos = window_rect.min;

                    // 창이 CentralPanel의 경계를 넘지 않도록 제어
                    if new_pos.x < central_panel_rect.left() {
                        new_pos.x = central_panel_rect.left();
                    }
                    if new_pos.x + window_rect.width() > central_panel_rect.right() {
                        new_pos.x = central_panel_rect.right() - window_rect.width();
                    }
                    if new_pos.y < central_panel_rect.top() {
                        new_pos.y = central_panel_rect.top();
                    }
                    if new_pos.y + window_rect.height() > central_panel_rect.bottom() {
                        new_pos.y = central_panel_rect.bottom() - window_rect.height();
                    }

                    // 창의 위치 업데이트
                    self.window_pos = new_pos;

                }
            }
        });
    }
}