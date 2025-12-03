use std::error::Error;
use std::sync::Arc;

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
use wgpu::wgt::DeviceDescriptor;

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
    pub async fn new(window: Arc<dyn Window>) -> Result<Self, Box<dyn Error>> {
        let instance: Instance = Self::create_instance();

        #[cfg(not(target_os = "windows"))]
        let surface: Surface = Self::create_surface(&instance, &window)?;

        #[cfg(target_os = "windows")]
        // Winit's Windows backend explicitly checks thread identity. If `raw_window_handle`
        // is called outside the event loop thread, it returns `HandleError::Unavailable`
        // to prevent race conditions (see `winit::platform::windows`).
        // Since we are initializing on a background thread but can guarantee the window
        // remains valid during this process, we use `window_handle_any_thread` to
        // bypass Winit's thread guard and access the raw HWND directly.
        let surface: Surface = unsafe {
            use wgpu::SurfaceTargetUnsafe;
            use wgpu::rwh::HasDisplayHandle;
            use winit::platform::windows::WindowExtWindows;
            instance.create_surface_unsafe(SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: window.display_handle()?.as_raw(),
                raw_window_handle: window.window_handle_any_thread()?.as_raw(),
            })
        }?;

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

    #[allow(dead_code)]
    fn create_surface(instance: &Instance, window: &Arc<dyn Window>) -> Result<Surface<'static>, CreateSurfaceError> {
        instance.create_surface(window.clone()) // TODO: No need for cloning.
    }

    async fn select_adapter(instance: &Instance, surface: &Surface<'static>) -> Result<Adapter, RequestAdapterError> {
        instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
    }

    async fn create_device(adapter: &Adapter) -> Result<(Device, Queue), RequestDeviceError> {
        adapter
            .request_device(&DeviceDescriptor {
                required_features: Features::empty(),
                required_limits: adapter.limits(),
                memory_hints: MemoryHints::default(),
                label: Some("REDPIXEL_DEVICE"),
                trace: Trace::Off,
                experimental_features: ExperimentalFeatures::default(),
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
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: surface_present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![surface_format],
            desired_maximum_frame_latency: 2,
        }
    }
}
