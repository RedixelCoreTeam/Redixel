use std::{error::Error, sync::Arc};
use wgpu::{
    Adapter, Backends, CreateSurfaceError, Device, ExperimentalFeatures, Features, Instance, InstanceDescriptor,
    MemoryHints, PowerPreference, PresentMode, Queue, RequestAdapterError, RequestAdapterOptions, RequestDeviceError,
    Surface, SurfaceCapabilities, SurfaceConfiguration, TextureFormat, TextureUsages, Trace, wgt::DeviceDescriptor,
};
use winit::{dpi::PhysicalSize, window::Window};

#[derive(Debug)]
pub struct RendererDevice {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
}

impl RendererDevice {
    pub async fn new(window: Arc<dyn Window>) -> Result<Self, Box<dyn Error>> {
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

    fn create_surface(instance: &Instance, window: &Arc<dyn Window>) -> Result<Surface<'static>, CreateSurfaceError> {
        instance.create_surface(window.clone())
    }

    async fn select_adapter(instance: &Instance, surface: &Surface<'static>) -> Result<Adapter, RequestAdapterError> {
        instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
    }

    async fn create_device(adapter: &Adapter) -> Result<(Device, Queue), RequestDeviceError> {
        adapter
            .request_device(&DeviceDescriptor {
                required_features: Features::empty(),
                required_limits: adapter.limits(),
                memory_hints: MemoryHints::Performance,
                label: Some("REDPIXEL_DEVICE"),
                trace: Trace::Off,
                experimental_features: ExperimentalFeatures::disabled(),
            })
            .await
    }

    fn create_surface_config(window: &Arc<dyn Window>, surface: &Surface, adapter: &Adapter) -> SurfaceConfiguration {
        let size: PhysicalSize<u32> = window.surface_size();
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
            width: size.width,
            height: size.height,
            present_mode: surface_present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![surface_format],
            desired_maximum_frame_latency: 2,
        }
    }
}
