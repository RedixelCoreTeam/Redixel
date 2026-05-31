use std::sync::Arc;

use wgpu::Backends;
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

use redixel_core::RedixelError;
use redixel_math::Color;
use redixel_math::Mat4;
use redixel_math::Vec2;

use crate::batch::SpriteBatch;
use crate::device::GpuDevice;
use crate::pipeline::ShapePipeline;

/// All renderer settings resolved from `config.json` by `redixel-runtime`
/// and injected at construction time. The renderer never touches the config system.
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

/// Commands collected during `on_render`, submitted to the GPU in one pass.
pub struct DrawQueue {
    pub clear: Color,
    pub batch: SpriteBatch,
}

/// High-level renderer. Owns the GPU device, shape pipeline, and sprite batch.
///
/// Interaction model:
/// 1. `begin_frame()` — resets the draw queue, returns a `&mut DrawQueue`
/// 2. Game code calls `queue.batch.draw_rect(...)` freely
/// 3. `end_frame()` — flushes the queue and presents the frame
pub struct Renderer {
    device: GpuDevice,
    pipeline: ShapePipeline,
    queue: DrawQueue,
}

impl Renderer {
    pub async fn new(window: Arc<dyn Window>, config: RendererConfig) -> Result<Self, RedixelError> {
        let device: GpuDevice = GpuDevice::new(window, &config).await?;
        let pipeline: ShapePipeline = ShapePipeline::new(&device.device, device.config.format);
        let batch: SpriteBatch = SpriteBatch::new(&device.device);

        Ok(Self {
            device,
            pipeline,
            queue: DrawQueue {
                clear: Color::rgb(0.1, 0.2, 0.3),
                batch,
            },
        })
    }

    /// Resizes the swap chain. Call whenever the window surface changes.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.device.resize(new_size);
    }

    /// Returns the current surface size in pixels.
    pub fn surface_size(&self) -> (u32, u32) {
        (self.device.config.width, self.device.config.height)
    }

    /// Returns a mutable reference to the draw queue.
    /// Game code queues draw calls here during `on_render`.
    pub fn draw_queue_mut(&mut self) -> &mut DrawQueue {
        &mut self.queue
    }

    /// Sets the clear colour for the next frame.
    pub fn set_clear_color(&mut self, color: Color) {
        self.queue.clear = color;
    }

    /// Queues a filled rectangle.
    pub fn draw_rect(&mut self, position: Vec2, size: Vec2, color: Color) {
        self.queue.batch.draw_rect(position, size, color);
    }

    /// Queues a filled triangle.
    pub fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        self.queue.batch.draw_triangle(p1, p2, p3, color);
    }

    /// Flushes all queued draw calls and presents the frame.
    ///
    /// 1. Uploads the orthographic camera matrix
    /// 2. Begins the render pass (clear)
    /// 3. Flushes the sprite batch (one draw call)
    /// 4. Submits commands and presents
    pub fn render(&mut self) -> Result<(), RedixelError> {
        let (w, h) = self.surface_size();

        let projection: Mat4 = Mat4::orthographic(0.0, w as f32, h as f32, 0.0, -1.0, 1.0);
        self.pipeline.update_camera(&self.device.queue, projection.cols);

        let output: SurfaceTexture = self.device.surface.get_current_texture()?;
        let view: TextureView = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder: CommandEncoder = self.device.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("REDIXEL_ENCODER"),
        });

        {
            let clear: wgpu::Color = self.queue.clear.into();

            let mut pass: RenderPass<'_> = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("REDIXEL_RENDER_PASS"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(clear),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.pipeline.pipeline);
            pass.set_bind_group(0, &self.pipeline.camera_bind_group, &[]);
            self.queue.batch.flush(&self.device.queue, &mut pass);
        }

        self.device.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
