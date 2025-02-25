use glam::Vec2;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub resolution: Vec2,
    pub center: Vec2,
    pub scale: f32,
    pub max_iter: u32,
}
