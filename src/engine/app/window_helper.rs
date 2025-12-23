use std::sync::Arc;
use winit::window::Window;

pub struct WindowHelper {
    pub window: Arc<Window>,
}

impl WindowHelper {
    pub fn new(window: Arc<Window>) -> Self {
        Self { window }
    }

    pub fn inner_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
