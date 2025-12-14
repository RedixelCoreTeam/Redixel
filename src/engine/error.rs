use std::cell::RefCell;
use std::rc::Rc;

use wgpu::CreateSurfaceError;
use wgpu::RequestAdapterError;
use wgpu::RequestDeviceError;
use wgpu::SurfaceError;
#[cfg(target_os = "windows")]
use wgpu::rwh::HandleError;

use winit::error::EventLoopError;
use winit::error::RequestError;

#[cfg(target_arch = "wasm32")]
use log::SetLoggerError;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

use thiserror::Error;

pub type SharedError = Rc<RefCell<Option<RedixelError>>>;

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

    #[cfg(target_arch = "wasm32")]
    #[error("Failed to initialize logger: {0}")]
    LoggerError(#[from] SetLoggerError),

    #[cfg(target_arch = "wasm32")]
    #[error("JavaScript Exception: {0:?}")]
    JsException(&'static str),

    #[cfg(test)]
    #[error("Intentional error triggered for testing purposes")]
    TestError,
}

#[cfg(target_arch = "wasm32")]
impl From<RedixelError> for JsValue {
    fn from(err: RedixelError) -> JsValue {
        let message = err.to_string();
        JsValue::from_str(&message)
    }
}
