use thiserror::Error;
use wgpu::{CreateSurfaceError, RequestAdapterError, RequestDeviceError, SurfaceError};
use winit::error::{EventLoopError, RequestError};

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

    #[error("Logger initialization failed: {0}")]
    Logger(String),

    #[cfg(target_arch = "wasm32")]
    #[error("JavaScript exception: {0}")]
    JsException(String),

    #[error("Dummy error (test only)")]
    Dummy,
}

impl From<log::SetLoggerError> for RedixelError {
    fn from(e: log::SetLoggerError) -> Self {
        RedixelError::Logger(e.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
impl From<RedixelError> for JsValue {
    fn from(e: RedixelError) -> JsValue {
        JsValue::from_str(&e.to_string())
    }
}
