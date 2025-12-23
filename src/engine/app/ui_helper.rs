use iced_wgpu::graphics::{Shell, Viewport};
use iced_wgpu::{Engine, Renderer};
use iced_winit::clipboard::Clipboard;
use iced_winit::core::{mouse, Font, Pixels, Size, Theme};
use iced_winit::runtime::user_interface::{self, UserInterface};
use iced_winit::winit;
use crate::engine::app::graphics_helper::GraphicsHelper;
use crate::engine::app::window_helper::WindowHelper;

pub struct UIHelper {
    pub renderer: Renderer,
    pub cache: user_interface::Cache,
    pub viewport: Viewport,
    pub cursor: mouse::Cursor,
    pub clipboard: Clipboard,
    pub events: Vec<iced_winit::core::Event>,
}

impl UIHelper {
    // Improved constructor that takes requirements from Outside
    pub fn new_with_engine(graphics: &GraphicsHelper, window: &WindowHelper, adapter: &wgpu::Adapter) -> Self {
        let physical_size = window.inner_size();
        let viewport = Viewport::with_physical_size(
            Size::new(physical_size.width, physical_size.height),
            window.scale_factor() as f32,
        );
        let clipboard = Clipboard::connect(window.window.clone());

        let renderer = {
            let engine = Engine::new(
                adapter,
                graphics.device.clone(),
                graphics.queue.clone(),
                graphics.format,
                None,
                Shell::headless(),
            );
            Renderer::new(engine, Font::default(), Pixels::from(16))
        };

        Self {
            renderer,
            cache: user_interface::Cache::new(),
            viewport,
            cursor: mouse::Cursor::Unavailable,
            clipboard,
            events: Vec::new(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, scale_factor: f64) {
        self.viewport = Viewport::with_physical_size(
            Size::new(width, height),
            scale_factor as f32,
        );
    }
    
    pub fn handle_event(&mut self, event: &winit::event::WindowEvent, scale_factor: f64) {
        if let Some(iced_event) = iced_winit::conversion::window_event(
            event.clone(),
            scale_factor as f32,
            Default::default(),
        ) {
            self.events.push(iced_event);
        }
        
        if let winit::event::WindowEvent::CursorMoved { position, .. } = event {
            self.cursor = mouse::Cursor::Available(iced_winit::conversion::cursor_position(
                *position,
                scale_factor as f32,
            ));
        }
    }

    pub fn draw<E>(
        &mut self,
        view: iced_winit::core::Element<'_, E, iced_winit::core::Theme, iced_wgpu::Renderer>,
        graphics: &GraphicsHelper,
        target_view: &wgpu::TextureView,
    ) {
        let mut user_interface = UserInterface::build(
            view,
            self.viewport.logical_size(),
            std::mem::take(&mut self.cache),
            &mut self.renderer,
        );

        let (_state, _) = user_interface.update(
            &self.events,
            self.cursor,
            &mut self.renderer,
            &mut self.clipboard,
            &mut Vec::new(),
        );

        user_interface.draw(
            &mut self.renderer,
            &Theme::Dark,
            &Default::default(),
            self.cursor,
        );

        self.cache = user_interface.into_cache();
        self.events.clear();

        self.renderer.present(
            None,
            graphics.config.format,
            target_view,
            &self.viewport,
        );
    }
}
