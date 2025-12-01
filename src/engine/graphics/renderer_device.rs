use std::error::Error;
use std::sync::Arc;

use wgpu::wgt::DeviceDescriptor;
use wgpu::Adapter;
use wgpu::Backends;
use wgpu::CreateSurfaceError;
use wgpu::Device;
use wgpu::ExperimentalFeatures;
use wgpu::Features;
use wgpu::Instance;
use wgpu::InstanceDescriptor;
use wgpu::MemoryHints;
use wgpu::PowerPreference;
use wgpu::PresentMode;
use wgpu::Queue;
use wgpu::RequestAdapterError;
use wgpu::RequestAdapterOptions;
use wgpu::RequestDeviceError;
use wgpu::Surface;
use wgpu::SurfaceCapabilities;
use wgpu::SurfaceConfiguration;
use wgpu::TextureFormat;
use wgpu::TextureUsages;
use wgpu::Trace;

use winit::dpi::PhysicalSize;
use winit::window::Window;

#[derive(Debug)]
pub struct RendererDevice {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
}

impl RendererDevice {
    pub async fn new(window: Arc<Window>) -> Result<Self, Box<dyn Error>> {
        let instance: Instance = Self::create_instance();
        let surface: Surface = Self::create_surface(&instance, &window)?;

        let adapter: Adapter = Self::select_adapter(&instance, &surface).await?;
        let (device, queue): (Device, Queue) = Self::create_device(&adapter).await?;

        let config: SurfaceConfiguration = Self::create_surface_config(&window, &surface, &adapter);
        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
        })
    }

    fn create_instance() -> Instance {
        Instance::new(&InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        })
    }

    fn create_surface(instance: &Instance, window: &Arc<Window>) -> Result<Surface<'static>, CreateSurfaceError> {
        instance.create_surface(window.clone())
    }

    async fn select_adapter(instance: &Instance, surface: &Surface<'static>) -> Result<Adapter, RequestAdapterError> {
        instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
    }

    async fn create_device(adapter: &Adapter) -> Result<(Device, Queue), RequestDeviceError> {
        adapter
            .request_device(&DeviceDescriptor {
                required_features: Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: MemoryHints::Performance,
                label: Some("REDPIXEL_DEVICE"),
                trace: Trace::Off,
                experimental_features: ExperimentalFeatures::disabled(),
            })
            .await
    }

    fn create_surface_config(window: &Arc<Window>, surface: &Surface, adapter: &Adapter) -> SurfaceConfiguration {
        let size: PhysicalSize<u32> = window.inner_size();
        let surface_caps: SurfaceCapabilities = surface.get_capabilities(adapter);

        let surface_format: TextureFormat = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f: &TextureFormat| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_present_mode: PresentMode = surface_caps
            .present_modes
            .iter()
            .copied()
            .find(|m: &PresentMode| *m == PresentMode::Fifo)
            .unwrap_or(surface_caps.present_modes[0]);

        SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: surface_present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![surface_format],
            desired_maximum_frame_latency: 2,
        }
    }
}
