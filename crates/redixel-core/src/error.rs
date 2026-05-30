use thiserror::Error;

use wgpu::CreateSurfaceError;
use wgpu::RequestAdapterError;
use wgpu::RequestDeviceError;
use wgpu::SurfaceError;

use winit::error::EventLoopError;
use winit::error::RequestError;

#[cfg(target_os = "windows")]
use wgpu::rwh::HandleError;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum RedixelError {
    #[error("Failed to create rendering surface: {0}")]
    CreateSurface(#[from] CreateSurfaceError),

    #[cfg(target_os = "windows")]
    #[error("Failed to acquire raw window handle: {0}")]
    WindowHandle(#[from] HandleError),

    #[error("Failed to find a suitable graphics adapter: {0}")]
    RequestAdapter(#[from] RequestAdapterError),

    #[error("Failed to create graphics device: {0}")]
    RequestDevice(#[from] RequestDeviceError),

    #[error("Window system request failed: {0}")]
    WindowRequest(#[from] RequestError),

    #[error("Event loop fatal error: {0}")]
    EventLoop(#[from] EventLoopError),

    #[error("Graphics surface error: {0}")]
    Surface(#[from] SurfaceError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config parse error: {0}")]
    Config(#[from] serde_json::Error),

    #[cfg(target_arch = "wasm32")]
    #[error("JavaScript exception: {0}")]
    JsException(&'static str),

    #[error("Dummy error (test only)")]
    Dummy,
}

#[cfg(target_arch = "wasm32")]
impl From<RedixelError> for JsValue {
    fn from(err: RedixelError) -> JsValue {
        JsValue::from_str(&err.to_string())
    }
}
