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

    #[cfg(not(target_arch = "wasm32"))]
    fn format_title(fps: f64) -> String {
        format!("Redixel - FPS: {fps:.0}")
    }

    pub fn set_title_fps(&self, #[allow(unused_variables)] fps: f64) {
        #[cfg(not(target_arch = "wasm32"))]
        self.window.set_title(&Self::format_title(fps));
    }

    pub fn should_handle(event: &WindowEvent) -> bool {
        matches!(event, WindowEvent::Focused(_) | WindowEvent::ScaleFactorChanged { .. })
    }

    pub fn is_window_event(&self, event: &WindowEvent) -> bool {
        Self::should_handle(event)
    }

    pub fn handle_window_event(&self, event: &WindowEvent) {
        match event {
            WindowEvent::Focused(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::dpi::PhysicalPosition;
    use winit::event::DeviceId;
    use winit::event::PointerSource;
    use winit::event::WindowEvent;

    #[test]
    fn test_fps_title_formatting() {
        assert_eq!(WindowManager::format_title(60.0), "Redixel - FPS: 60");
        assert_eq!(WindowManager::format_title(59.99), "Redixel - FPS: 60");
        assert_eq!(WindowManager::format_title(144.1), "Redixel - FPS: 144");
    }

    #[test]
    fn test_event_filter_logic() {
        let event_focused: WindowEvent = WindowEvent::Focused(true);

        let event_cursor: WindowEvent = WindowEvent::PointerMoved {
            primary: true,
            source: PointerSource::Mouse,
            device_id: Some(DeviceId::from_raw(1)),
            position: PhysicalPosition::new(0.0, 0.0),
        };

        assert!(WindowManager::should_handle(&event_focused));
        assert!(!WindowManager::should_handle(&event_cursor));
    }
}
