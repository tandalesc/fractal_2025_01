use glam::Vec2;

pub const ZOOM_MIN: f32 = 1.0;
pub const ZOOM_DEFAULT: f32 = 2.0;
pub const ZOOM_MAX: f32 = 17.0;

pub const ZOOM_PIXELS_FACTOR: f32 = 200.0;
pub const ZOOM_WHEEL_SCALE: f32 = 0.2;

pub const ITERS_MIN: u32 = 20;
pub const ITERS_DEFAULT: u32 = 20;
pub const ITERS_MAX: u32 = 200;

pub const CENTER_DEFAULT: Vec2 = Vec2::new(-1.5, 0.0);