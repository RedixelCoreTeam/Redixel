use std::sync::Arc;

use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowAttributes;

use winit::dpi::LogicalSize;

use crate::engine::error::RedixelError;
use crate::engine::settings::EngineSettings;

#[derive(Debug)]
pub struct WindowManager {
    pub window: Arc<dyn Window>,
}

impl WindowManager {
    pub fn new(event_loop: &dyn ActiveEventLoop) -> Result<Self, RedixelError> {
        let settings: std::sync::RwLockReadGuard<'_, EngineSettings> = EngineSettings::global_read();

        let title: String = settings.get_path("app.name", "Redixel".to_string());
        let width: u32 = settings.get_path("window.width", 600);
        let height: u32 = settings.get_path("window.height", 500);
        // TODO implementar fullscreen de acordo com winit::monitor::Fullscreen;
        // let fullscreen: bool = settings.get_path("window.fullscreen", false);

        #[allow(unused_mut)]
        let mut attributes: WindowAttributes = WindowAttributes::default()
            .with_title(title)
            .with_surface_size(LogicalSize::new(width, height));

        #[cfg(target_arch = "wasm32")]
        {
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

            let canvas_element: HtmlCanvasElement = html_element.dyn_into::<HtmlCanvasElement>().map_err(|_| {
                RedixelError::JsException("The element '#redixel-canvas' exists but is NOT a <canvas>.")
            })?;

            let web_attributes: WindowAttributesWeb = WindowAttributesWeb::default().with_canvas(Some(canvas_element));
            attributes = attributes.with_platform_attributes(Box::new(web_attributes));
        }

        Ok(Self {
            window: Arc::from(event_loop.create_window(attributes)?),
        })
    }

    pub fn get_window(&self) -> Arc<dyn Window> {
        self.window.clone() // TODO: No need for cloning.
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn set_title_fps(&self, fps: f64) {
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
