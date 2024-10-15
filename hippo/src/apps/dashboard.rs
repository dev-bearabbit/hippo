use eframe::egui;
use crate::models::chart::ChartType;
use crate::models::table::RecordTable;

pub struct Dashboard {
    pub id: usize,
    pub border: bool,
    pub resizable: bool,
    pub viewport: bool,
    pub save: bool,
    pub open: bool,
    pub chart: ChartType,
    pub window_pos: egui::Pos2,
}

impl Dashboard {

    pub fn new(id: usize, resizable: bool, chart: ChartType) -> Self {
        Self {
            id,
            border: true,
            resizable,
            chart,
            viewport: true,
            save: false,
            open: true,
            window_pos: egui::Pos2::new(300.0, 200.0),
        }
    }

    pub fn update_dashboard(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, table_data: &RecordTable) {

        egui::CentralPanel::default().show_inside(ui, |ui| {

            if self.viewport {
                // 창을 표시
                let window_response_q = egui::Window::new(format!("chart setting {}", self.id))
                    .id(egui::Id::new(self.id))
                    .current_pos(self.window_pos) 
                    .resizable(self.resizable)
                    .title_bar(false)
                    .enabled(true)
                    .min_size(egui::vec2(600.0, 400.0))
                    .max_size(egui::vec2(600.0, 400.0))
                    .show(ctx, |ui| {
                                            
                        ChartType::execute(&mut self.chart, ui, table_data, true);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.horizontal(|ui| {

                                if ui.button("Close").clicked() {
                                    self.viewport = false;
                                }
                                if ui.button("Confirm").clicked() {
                                    self.save = true;
                                    self.viewport = false;
                                } 
                                ui.add_space(5.0);
                                ui.checkbox(&mut self.border, "Border");
                            });
                        });
                    });

                // CentralPanel의 경계를 가져옴
                let central_panel_rect = ui.max_rect();
                let res = window_response_q.unwrap().response;
                let window_rect: egui::Rect = res.rect;
                let mut new_pos: egui::Pos2 = window_rect.min;

                self._restrict_graph_area(central_panel_rect, window_rect, &mut new_pos);
            }

            if self.save && self.open {
                // border 옵션
                let mut frame_option = egui::Frame {
                    shadow: egui::Shadow::NONE,
                    stroke: egui::Stroke::new(1.0 ,egui::Color32::GRAY),
                    rounding: egui::Rounding { nw: (5.0), ne: (5.0), sw: (5.0), se: (5.0)},
                    ..Default::default()};

                if self.border == false {
                    frame_option = egui::Frame::none();
                };

                let window_response_c = egui::Window::new(format!("chart {}", self.id))
                    .id(egui::Id::new(self.id))
                    .current_pos(self.window_pos) 
                    .resizable(true)
                    .title_bar(false)
                    .collapsible(false)
                    .enabled(true)
                    .max_size(ui.max_rect().size())
                    .frame(frame_option)
                    .show(ctx, |ui| {

                        if ui.rect_contains_pointer(ui.max_rect()) && ctx.input(|i| 
                            i.pointer.button_double_clicked(egui::PointerButton::Primary)) {
                            self.open = false;
                        }
                        ChartType::execute(&mut self.chart, ui, table_data, false);
                });

                
                let res = window_response_c.unwrap().response;
                let window_rect: egui::Rect = res.rect;
                let mut new_pos: egui::Pos2 = window_rect.min;


                // CentralPanel의 경계를 가져옴
                self._restrict_graph_area(ui.max_rect(),window_rect, &mut new_pos);
                }
        });
    }


    // 창의 위치 제한
    fn _restrict_graph_area(&mut self, central_panel_rect: egui::Rect, window_rect: egui::Rect, new_pos: &mut egui::Pos2) {

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
            self.window_pos = *new_pos;
            } 
}
