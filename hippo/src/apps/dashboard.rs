use eframe::egui;
use crate::models::chart::ChartType;
use crate::models::table::RecordTable;
use crate::apps::graph::Graph;
use crate::apps::custom::Custom;

pub struct Dashboard {
    pub id: usize,
    pub resizable: bool,
    pub viewport: bool,
    pub save: bool,
    pub open: bool,
    pub chart: ChartType,
    pub window_pos: egui::Pos2,
    pub custom: Custom,
    pub graph: Graph
}

impl Dashboard {

    pub fn new(id: usize, resizable: bool, chart: ChartType) -> Self {
        Self {
            id,
            resizable,
            chart,
            viewport: true,
            save: false,
            open: true,
            window_pos: egui::Pos2::new(300.0, 200.0),
            custom: Custom::new(),
            graph: Graph::new()
        }
    }

    pub fn update_dashboard(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, table_data: &RecordTable) {

        egui::CentralPanel::default().show_inside(ui, |ui| {

            if self.viewport {
                // 창을 표시
                let window_response = egui::Window::new(format!("chart setting {}", self.id))
                    .id(egui::Id::new(self.id))
                    .current_pos(self.window_pos) 
                    .resizable(self.resizable)
                    .title_bar(false)
                    .enabled(true)
                    .min_size(egui::vec2(600.0, 400.0))
                    .max_size(egui::vec2(600.0, 400.0))
                    .show(ctx, |ui| {

                        ChartType::execute(&self.chart, ui, &mut self.custom, &mut self.graph, table_data, true);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Close").clicked() {
                                    self.viewport = false;
                                }
                                if ui.button("Confirm").clicked() {
                                    self.save = true;
                                    self.viewport = false;
                                } 
                            });
                        });
                    });

                // CentralPanel의 경계를 가져옴
                let central_panel_rect = ui.max_rect();
                self._restrict_graph_area(central_panel_rect, window_response);
            }

            if self.save {
                // 창을 표시
                let chart_id = format!("chart {}", self.id);
                let window_response = egui::Window::new("")
                    .id(egui::Id::new(&chart_id))
                    .current_pos(self.window_pos) 
                    .resizable(true)
                    .title_bar(true)
                    .collapsible(false)
                    .open(&mut self.open)
                    .enabled(true)
                    .max_size(ui.max_rect().size())
                    .frame(egui::Frame {
                        shadow: egui::Shadow::NONE,
                        stroke: egui::Stroke::new(1.0 ,egui::Color32::GRAY),
                        rounding: egui::Rounding { nw: (5.0), ne: (5.0), sw: (5.0), se: (5.0)},
                        ..Default::default()
                    })
                    .show(ctx, |ui| {

                        ChartType::execute(&self.chart, ui, &mut self.custom, &mut self.graph, table_data, false);
                    });

                // CentralPanel의 경계를 가져옴
                self._restrict_graph_area(ui.max_rect(), window_response);
            }

        });
    }

    // 창의 위치 제한
    fn _restrict_graph_area(&mut self, central_panel_rect: egui::Rect, window_response: Option<egui::InnerResponse<Option<()>>>) {
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
}