use std::sync::Arc;

use winit::error::RequestError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowAttributes;

#[derive(Debug)]
pub struct WindowManager {
    pub window: Arc<dyn Window>,
}

impl WindowManager {
    pub fn new(event_loop: &dyn ActiveEventLoop) -> Result<Self, RequestError> {
        #[allow(unused_mut)]
        let mut attributes: WindowAttributes = WindowAttributes::default().with_title("Redixel");

        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesWeb;

            let window = web_sys::window().expect("Global 'window' object not found.");
            let document = window.document().expect("Global 'document' object not found.");

            let html_element = document
                .get_element_by_id("redixel-canvas")
                .expect("Could not find element '#redixel-canvas' in the DOM.");

            let canvas_element = html_element
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("The element '#redixel-canvas' exists but is NOT a <canvas>.");

            let web_attributes = WindowAttributesWeb::default().with_canvas(Some(canvas_element));
            attributes = attributes.with_platform_attributes(Box::new(web_attributes));
        }

        Ok(Self {
            window: Arc::from(event_loop.create_window(attributes)?),
        })
    }

    pub fn get_window(&self) -> Arc<dyn Window> {
        self.window.clone() // TODO: No need for cloning.
    }

    pub fn handle_window_event(&self, event: &WindowEvent) {
        match event {
            WindowEvent::Focused(..) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            _ => {}
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn is_window_event(&self, event: &WindowEvent) -> bool {
        matches!(event, WindowEvent::Focused { .. } | WindowEvent::ScaleFactorChanged { .. })
    }
}
