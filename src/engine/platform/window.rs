use std::sync::Arc;

use winit::error::OsError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowAttributes;

#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowAttributesExtWebSys;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[derive(Debug)]
pub struct WindowManager {
    pub window: Arc<Window>,
}

impl WindowManager {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self, OsError> {
        #[allow(unused)]
        let mut attributes: WindowAttributes = WindowAttributes::default().with_title("RedPixel Engine");

        // GET Big big CANVAS
        #[cfg(target_arch = "wasm32")]
        {
            let win = web_sys::window().expect("No global window found");
            let doc = win.document().expect("No document found");
            if let Some(canvas_element) = doc.get_element_by_id("canvas") {
                let canvas: web_sys::HtmlCanvasElement = canvas_element.unchecked_into::<web_sys::HtmlCanvasElement>();
                attributes = attributes.with_canvas(Some(canvas));
            }
        }

        let window: Box<Window> = Box::new(event_loop.create_window(attributes).unwrap());

        Ok(Self {
            window: Arc::from(window),
        })
    }

    pub fn get_window(&self) -> Arc<Window> {
        self.window.clone() // TODO: See if there is a better way than cloning here
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
}
