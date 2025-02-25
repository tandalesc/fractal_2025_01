use glam::Vec2;
use iced::Rectangle;

use crate::fractal::constants::*;

#[derive(Debug, Clone, Copy)]
pub struct Controls {
    pub max_iter: u32,
    pub zoom: f32,
    pub center: Vec2,
}

impl Controls {
    pub fn scale(&self) -> f32 {
        1.0 / 2.0_f32.powf(self.zoom) / ZOOM_PIXELS_FACTOR
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            max_iter: ITERS_DEFAULT,
            zoom: ZOOM_DEFAULT,
            center: CENTER_DEFAULT,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateMaxIterations(u32),
    UpdateZoom(f32),
    PanningDelta(Vec2),
    ZoomDelta(Vec2, Rectangle, f32),
}

pub enum MouseInteraction {
    Idle,
    Panning(Vec2),
}

impl Default for MouseInteraction {
    fn default() -> Self {
        MouseInteraction::Idle
    }
}
