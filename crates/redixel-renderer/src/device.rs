use std::sync::Arc;

use wgpu::{
    Adapter, Backends, Device, ExperimentalFeatures, Features, Instance, InstanceDescriptor, MemoryHints,
    PowerPreference, PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceCapabilities, SurfaceConfiguration,
    TextureFormat, TextureUsages, Trace,
    wgt::{DeviceDescriptor, SurfaceConfiguration as WgtSurfaceConfiguration},
};

use winit::{dpi::PhysicalSize, window::Window};

use redixel_core::RedixelError;

use crate::renderer::RendererConfig;

/// Owns the WGPU logical device, presentation surface, and submission queue.
///
/// This is a pure graphics-layer type with no knowledge of the windowing
/// system beyond the `Arc<dyn Window>` it receives at construction time.
/// All configurable behaviour is injected via [`RendererConfig`].
#[derive(Debug)]
pub(crate) struct GpuDevice {
    pub(crate) surface: Surface<'static>,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) config: SurfaceConfiguration,
}

impl GpuDevice {
    pub(crate) async fn new(window: Arc<dyn Window>, cfg: &RendererConfig) -> Result<Self, RedixelError> {
        let instance: Instance = Self::create_instance(cfg.backends);
        let surface: Surface<'_> = Self::create_surface(&instance, &window)?;
        let adapter: Adapter = Self::request_adapter(&instance, &surface).await?;
        let (device, queue) = Self::request_device(&adapter).await?;
        let config: WgtSurfaceConfiguration<Vec<TextureFormat>> =
            Self::build_surface_config(&window, &surface, &adapter, cfg.present_mode);
        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
        })
    }

    /// Reconfigures the swap chain to match a new window size.
    /// No-ops for zero-area sizes (minimised window).
    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn create_instance(backends: Backends) -> Instance {
        Instance::new(&InstanceDescriptor {
            backends,
            ..Default::default()
        })
    }

    fn create_surface(instance: &Instance, window: &Arc<dyn Window>) -> Result<Surface<'static>, RedixelError> {
        #[cfg(target_os = "windows")]
        {
            // On Windows, `raw_window_handle()` enforces thread identity and will
            // return `HandleError::Unavailable` when called from outside the event-
            // loop thread. We initialise on a background thread, so we use
            // `window_handle_any_thread` to bypass the guard safely — the `Arc`
            // guarantees the window outlives this call.
            use wgpu::SurfaceTargetUnsafe;
            use wgpu::rwh::HasDisplayHandle;
            use winit::platform::windows::WindowExtWindows;

            unsafe {
                instance
                    .create_surface_unsafe(SurfaceTargetUnsafe::RawHandle {
                        raw_display_handle: window.display_handle()?.as_raw(),
                        raw_window_handle: window.window_handle_any_thread()?.as_raw(),
                    })
                    .map_err(RedixelError::from)
            }
        }

        #[cfg(not(target_os = "windows"))]
        instance.create_surface(window.clone()).map_err(RedixelError::from)
    }

    async fn request_adapter(instance: &Instance, surface: &Surface<'static>) -> Result<Adapter, RedixelError> {
        instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(RedixelError::from)
    }

    async fn request_device(adapter: &Adapter) -> Result<(Device, Queue), RedixelError> {
        adapter
            .request_device(&DeviceDescriptor {
                label: Some("REDIXEL_DEVICE"),
                required_features: Features::empty(),
                required_limits: adapter.limits(),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
                experimental_features: ExperimentalFeatures::default(),
            })
            .await
            .map_err(RedixelError::from)
    }

    fn build_surface_config(
        window: &Arc<dyn Window>,
        surface: &Surface,
        adapter: &Adapter,
        desired_present_mode: PresentMode,
    ) -> SurfaceConfiguration {
        let size: PhysicalSize<u32> = window.surface_size();
        let caps: SurfaceCapabilities = surface.get_capabilities(adapter);

        let format: TextureFormat = caps
            .formats
            .iter()
            .copied()
            .find(|f: &TextureFormat| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let present_mode: PresentMode = caps
            .present_modes
            .iter()
            .copied()
            .find(|&m: &PresentMode| m == desired_present_mode)
            .unwrap_or(caps.present_modes[0]);

        SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![format],
            desired_maximum_frame_latency: 2,
        }
    }
}
