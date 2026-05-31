use wgpu::BindGroup;
use wgpu::BindGroupDescriptor;
use wgpu::BindGroupEntry;
use wgpu::BindGroupLayout;
use wgpu::BindGroupLayoutDescriptor;
use wgpu::BindGroupLayoutEntry;
use wgpu::BindingResource;
use wgpu::BindingType;
use wgpu::BlendState;
use wgpu::Buffer;
use wgpu::BufferBindingType;
use wgpu::BufferDescriptor;
use wgpu::BufferUsages;
use wgpu::ColorTargetState;
use wgpu::ColorWrites;
use wgpu::Device;
use wgpu::FragmentState;
use wgpu::FrontFace;
use wgpu::MultisampleState;
use wgpu::PipelineLayout;
use wgpu::PipelineLayoutDescriptor;
use wgpu::PolygonMode;
use wgpu::PrimitiveState;
use wgpu::PrimitiveTopology;
use wgpu::Queue;
use wgpu::RenderPipeline;
use wgpu::RenderPipelineDescriptor;
use wgpu::ShaderModule;
use wgpu::ShaderModuleDescriptor;
use wgpu::ShaderSource;
use wgpu::ShaderStages;
use wgpu::TextureFormat;
use wgpu::VertexAttribute;
use wgpu::VertexBufferLayout;
use wgpu::VertexState;
use wgpu::VertexStepMode;

const SHADER_SRC: &str = include_str!("../shaders/shape.wgsl");

/// A single vertex in the shape batch: position + RGBA colour.
///
/// `repr(C)` + packed fields → safe to cast to `&[u8]` via `bytemuck`.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    const ATTRIBUTES: [VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x4,
    ];

    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

/// The uniform buffer fed to `group(0) binding(0)` in the shader.
/// Contains a column-major 4×4 orthographic projection matrix.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn from_mat4(m: [[f32; 4]; 4]) -> Self {
        Self { projection: m }
    }
}

/// Owns the WGPU render pipeline, camera uniform buffer, and bind group
/// for drawing coloured 2D shapes.
pub struct ShapePipeline {
    pub pipeline: RenderPipeline,
    pub camera_buffer: Buffer,
    pub camera_bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,
}

impl ShapePipeline {
    pub fn new(device: &Device, surface_format: TextureFormat) -> Self {
        let shader: ShaderModule = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("REDIXEL_SHAPE_SHADER"),
            source: ShaderSource::Wgsl(SHADER_SRC.into()),
        });

        let camera_buffer: Buffer = device.create_buffer(&BufferDescriptor {
            label: Some("REDIXEL_CAMERA_BUFFER"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout: BindGroupLayout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("REDIXEL_CAMERA_BIND_GROUP_LAYOUT"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let camera_bind_group: BindGroup = device.create_bind_group(&BindGroupDescriptor {
            label: Some("REDIXEL_CAMERA_BIND_GROUP"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(camera_buffer.as_entire_buffer_binding()),
            }],
        });

        let pipeline_layout: PipelineLayout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("REDIXEL_SHAPE_PIPELINE_LAYOUT"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline: RenderPipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("REDIXEL_SHAPE_PIPELINE"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: surface_format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                front_face: FrontFace::Ccw,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            camera_buffer,
            camera_bind_group,
            bind_group_layout,
        }
    }

    /// Uploads the current orthographic matrix to the GPU uniform buffer.
    pub fn update_camera(&self, queue: &Queue, projection: [[f32; 4]; 4]) {
        let uniform: CameraUniform = CameraUniform::from_mat4(projection);
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::bytes_of(&uniform));
    }
}
