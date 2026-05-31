use wgpu::Buffer;
use wgpu::BufferDescriptor;
use wgpu::BufferUsages;
use wgpu::Device;
use wgpu::Queue;
use wgpu::RenderPass;

use crate::pipeline::Vertex;

use redixel_math::Color;
use redixel_math::Vec2;

const MAX_QUADS: usize = 10_000;
const MAX_VERTICES: usize = MAX_QUADS * 6;

/// Accumulates `draw_rect` calls per frame and submits them to the GPU in a
/// single draw call on `flush()`.
///
/// This is the standard 2D batch-rendering pattern: minimise draw calls by
/// grouping same-pipeline geometry together.
pub struct SpriteBatch {
    vertex_buffer: Buffer,
    vertices: Vec<Vertex>,
}

impl SpriteBatch {
    pub fn new(device: &Device) -> Self {
        let vertex_buffer: Buffer = device.create_buffer(&BufferDescriptor {
            label: Some("REDIXEL_SPRITE_BATCH_VB"),
            size: (MAX_VERTICES * std::mem::size_of::<Vertex>()) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            vertex_buffer,
            vertices: Vec::with_capacity(MAX_VERTICES),
        }
    }

    /// Queues a filled rectangle for drawing.
    ///
    /// - `position` — top-left corner in world coordinates (y-down)
    /// - `size`     — width × height in world units
    /// - `color`    — RGBA fill colour
    pub fn draw_rect(&mut self, position: Vec2, size: Vec2, color: Color) {
        if self.vertices.len() + 6 > MAX_VERTICES {
            log::warn!("SpriteBatch: MAX_QUADS ({MAX_QUADS}) exceeded — quad dropped.");
            return;
        }

        let x0: f32 = position.x;
        let y0: f32 = position.y;
        let x1: f32 = position.x + size.x;
        let y1: f32 = position.y + size.y;
        let c: [f32; 4] = color.to_array();

        let tl: Vertex = Vertex {
            position: [x0, y0],
            color: c,
        };

        let tr: Vertex = Vertex {
            position: [x1, y0],
            color: c,
        };

        let bl: Vertex = Vertex {
            position: [x0, y1],
            color: c,
        };

        let br: Vertex = Vertex {
            position: [x1, y1],
            color: c,
        };

        self.vertices.extend_from_slice(&[tl, bl, tr, tr, bl, br]);
    }

    /// Queues a filled triangle for drawing.
    ///
    /// - `p1`, `p2`, `p3` — The three corners of the triangle in world coordinates
    /// - `color`          — RGBA fill colour
    pub fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        if self.vertices.len() + 3 > MAX_VERTICES {
            log::warn!("SpriteBatch: MAX_VERTICES ({MAX_VERTICES}) exceeded — triangle dropped.");
            return;
        }

        let c: [f32; 4] = color.to_array();

        let v1: Vertex = Vertex {
            position: [p1.x, p1.y],
            color: c,
        };

        let v2: Vertex = Vertex {
            position: [p2.x, p2.y],
            color: c,
        };

        let v3: Vertex = Vertex {
            position: [p3.x, p3.y],
            color: c,
        };

        self.vertices.extend_from_slice(&[v1, v2, v3]);
    }

    /// Returns the number of vertices currently queued.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Uploads queued vertices to the GPU and records the draw call.
    ///
    /// Must be called **inside** an active `RenderPass`.
    /// Clears the internal queue after submission.
    pub fn flush<'rp>(&mut self, queue: &Queue, pass: &mut RenderPass<'rp>)
    where
        Self: 'rp,
    {
        let count: usize = self.vertices.len();
        if count == 0 {
            return;
        }

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..count as u32, 0..1);

        self.vertices.clear();
    }
}
