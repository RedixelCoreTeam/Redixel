use std::sync::Arc;

use wgpu::Backends;
use wgpu::Color;
use wgpu::CommandEncoder;
use wgpu::CommandEncoderDescriptor;
use wgpu::LoadOp;
use wgpu::Operations;
use wgpu::PresentMode;
use wgpu::RenderPass;
use wgpu::RenderPassColorAttachment;
use wgpu::RenderPassDescriptor;
use wgpu::StoreOp;
use wgpu::SurfaceTexture;
use wgpu::TextureView;
use wgpu::TextureViewDescriptor;

use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::device::GpuDevice;
use redixel_core::RedixelError;

/// All renderer settings, resolved from `config.json` by `redixel-runtime`
/// and injected here. The renderer has zero knowledge of the config system.
#[derive(Debug, Clone)]
pub struct RendererConfig {
    pub backends: Backends,
    pub present_mode: PresentMode,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            backends: Backends::all(),
            present_mode: PresentMode::AutoVsync,
        }
    }
}

/// High-level renderer. Owns the [`GpuDevice`] and drives the render loop.
///
/// The device is a private implementation detail: callers interact only
/// through [`render`] and [`resize`].
#[derive(Debug)]
pub struct Renderer {
    device: GpuDevice,
}

impl Renderer {
    pub async fn new(window: Arc<dyn Window>, config: RendererConfig) -> Result<Self, RedixelError> {
        Ok(Self {
            device: GpuDevice::new(window, &config).await?,
        })
    }

    /// Resizes the swap chain. Call whenever the window surface changes.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.device.resize(new_size);
    }

    /// Submits a single frame to the GPU.
    pub fn render(&mut self) -> Result<(), RedixelError> {
        let output: SurfaceTexture = self.device.surface.get_current_texture()?;
        let view: TextureView = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder: CommandEncoder = self.device.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("REDIXEL_ENCODER"),
        });

        let pass: RenderPass<'_> = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("REDIXEL_RENDER_PASS"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
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
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        drop(pass);

        self.device.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
