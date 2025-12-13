use std::sync::Arc;

use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowAttributes;

use crate::engine::error::RedixelError;

#[derive(Debug)]
pub struct WindowManager {
    pub window: Arc<dyn Window>,
}

impl WindowManager {
    pub fn new(event_loop: &dyn ActiveEventLoop) -> Result<Self, RedixelError> {
        let attributes: WindowAttributes = Self::create_window_attributes()?;

        Ok(Self {
            window: Arc::from(event_loop.create_window(attributes)?),
        })
    }

    fn create_window_attributes() -> Result<WindowAttributes, RedixelError> {
        let attributes: WindowAttributes = WindowAttributes::default().with_title("Redixel");
        Self::apply_platform_attributes(attributes)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn apply_platform_attributes(attributes: WindowAttributes) -> Result<WindowAttributes, RedixelError> {
        Ok(attributes)
    }

    #[cfg(target_arch = "wasm32")]
    fn apply_platform_attributes(attributes: WindowAttributes) -> Result<WindowAttributes, RedixelError> {
        use web_sys::Document;
        use web_sys::Element;
        use web_sys::HtmlCanvasElement;
        use web_sys::Window;
        use web_sys::wasm_bindgen::JsCast;
        use winit::platform::web::WindowAttributesWeb;

        let window: Window =
            web_sys::window().ok_or_else(|| RedixelError::JsException("Global 'window' object not found."))?;

        let document: Document = window
            .document()
            .ok_or_else(|| RedixelError::JsException("Global 'document' object not found."))?;

        let html_element: Element = document
            .get_element_by_id("redixel-canvas")
            .ok_or_else(|| RedixelError::JsException("Could not find element '#redixel-canvas' in the DOM."))?;

        let canvas_element: HtmlCanvasElement = html_element
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| RedixelError::JsException("The element '#redixel-canvas' exists but is NOT a <canvas>."))?;

        let web_attributes: WindowAttributesWeb = WindowAttributesWeb::default().with_canvas(Some(canvas_element));
        Ok(attributes.with_platform_attributes(Box::new(web_attributes)))
    }

    pub fn get_window(&self) -> Arc<dyn Window> {
        self.window.clone()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn set_title_fps(&self, #[allow(unused_variables)] fps: f64) {
        #[cfg(not(target_arch = "wasm32"))]
        self.window.set_title(&format!("Redixel - FPS: {fps:.0}"));
    }

    pub fn handle_window_event(&self, event: &WindowEvent) {
        match event {
            WindowEvent::Focused(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            _ => {}
        }
    }

    pub fn is_window_event(&self, event: &WindowEvent) -> bool {
        matches!(event, WindowEvent::Focused(_) | WindowEvent::ScaleFactorChanged { .. })
    }
}
