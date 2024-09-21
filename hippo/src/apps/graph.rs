use crate::models::chart::ChartType;
use polars::prelude::*;
use plotters::prelude::*;
use eframe::egui::TextureHandle;

struct Graph {
    texture: Option<TextureHandle>,
}

impl Default for Graph {
    fn default() -> Self {
        Self { texture: None }
    }

} 
