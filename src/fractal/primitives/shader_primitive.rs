use glam::Vec2;
use iced::Rectangle;
use iced::widget::shader::{self, Viewport, wgpu};

use super::controls::Controls;
use super::pipeline::FragmentShaderPipeline;
use super::uniforms::Uniforms;

#[derive(Debug)]
pub struct FragmentShaderPrimitive {
    controls: Controls,
}

impl FragmentShaderPrimitive {
    pub fn new(controls: Controls) -> Self {
        Self { controls }
    }
}

impl shader::Primitive for FragmentShaderPrimitive {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut shader::Storage,
        _bounds: &Rectangle,
        viewport: &Viewport,
    ) {
        let target_size = viewport.logical_size();
        if !storage.has::<FragmentShaderPipeline>() {
            storage.store(FragmentShaderPipeline::new(device, format));
        }
        if let Some(pipeline) = storage.get_mut::<FragmentShaderPipeline>() {
            let uniforms = Uniforms {
                resolution: Vec2::new(target_size.width as f32, target_size.height as f32),
                center: self.controls.center,
                scale: self.controls.scale(),
                max_iter: self.controls.max_iter,
            };
            pipeline.update(queue, &uniforms);
        } else {
            panic!("Failed to prepare fragment shader");
        }
    }

    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        if let Some(pipeline) = storage.get::<FragmentShaderPipeline>() {
            pipeline.render(target, encoder, *clip_bounds);
        } else {
            panic!("Failed to render fragment shader");
        }
    }
}
