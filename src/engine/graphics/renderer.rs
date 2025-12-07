use std::sync::Arc;

use wgpu::Color;
use wgpu::CommandEncoder;
use wgpu::CommandEncoderDescriptor;
use wgpu::LoadOp;
use wgpu::Operations;
use wgpu::RenderPassColorAttachment;
use wgpu::RenderPassDescriptor;
use wgpu::StoreOp;
use wgpu::SurfaceTexture;
use wgpu::TextureView;
use wgpu::TextureViewDescriptor;

use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::renderer_device::RendererDevice;
use crate::engine::error::RedixelError;

#[derive(Debug)]
pub struct Renderer {
    renderer_device: RendererDevice,
}

impl Renderer {
    pub async fn new(window: Arc<dyn Window>) -> Result<Self, RedixelError> {
        let renderer_device: RendererDevice = RendererDevice::new(window).await?;
        Ok(Self { renderer_device })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.renderer_device.config.width = new_size.width;
            self.renderer_device.config.height = new_size.height;
            self.renderer_device
                .surface
                .configure(&self.renderer_device.device, &self.renderer_device.config);
        }
    }

    fn render_pass(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("REDIXEL_RENDER_PASS"),
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
        });
    }

    pub fn render(&mut self) -> Result<(), RedixelError> {
        let output: SurfaceTexture = self.renderer_device.surface.get_current_texture()?;
        let view: TextureView = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder: CommandEncoder =
            self.renderer_device
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("REDIXEL_RENDER_ENCODER"),
                });

        self.render_pass(&mut encoder, &view);
        self.renderer_device.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
