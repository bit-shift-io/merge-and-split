use std::sync::Arc;

use winit::{
    application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, window::Window
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::engine::app::{
    context::Context,
    game_loop::GameLoop,
    graphics_helper::GraphicsHelper,
    window_helper::WindowHelper,
    event_system::EventSystem,
    ui_helper::UIHelper,
};

pub struct App<L: GameLoop> {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<Context>>,
    pub ctx: Option<Context>,
    event_loop: Option<EventLoop<Context>>,
    pub game_logic: Option<L>,
    pub last_frame_time: Option<std::time::Instant>,
}

impl<L: GameLoop> App<L> {
    pub fn new() -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            proxy: None,
            ctx: None,
            event_loop: None,
            game_logic: None,
            last_frame_time: None,
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::init();
        }
        #[cfg(target_arch = "wasm32")]
        {
            console_log::init_with_level(log::Level::Info).unwrap_throw();
        }

        self.event_loop = Some(EventLoop::with_user_event().build()?);
        
        #[cfg(target_arch = "wasm32")]
        {
            self.proxy = Some(self.event_loop.as_ref().unwrap().create_proxy());
        }

        self.event_loop
            .take()
            .unwrap()
            .run_app(self)?;

        Ok(())
    }

    fn on_context_set(&mut self) {
        if let Some(ctx) = &mut self.ctx {
            self.game_logic = Some(L::new(ctx));
        }
    }
}

impl<L: GameLoop> ApplicationHandler<Context> for App<L> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;
            const CANVAS_ID: &str = "canvas";
            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            let graphics = pollster::block_on(GraphicsHelper::new(window.clone())).unwrap();
            let window_helper = WindowHelper::new(window);
            let event_system = EventSystem::new();
            let ui = UIHelper::new_with_engine(&graphics, &window_helper, &graphics.adapter);
            
            self.ctx = Some(Context::new(graphics, window_helper, event_system, ui));
            self.on_context_set();
        }

        #[cfg(target_arch = "wasm32")]
        {
            // WASM handling would need similar helper-based init
            // For now, let's focus on the primary architecture refactor
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut ctx: Context) {
        self.ctx = Some(ctx);
        self.on_context_set();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let ctx = match &mut self.ctx {
            Some(c) => c,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                ctx.graphics.resize(size.width, size.height);
                ctx.ui.resize(size.width, size.height, ctx.window.scale_factor());
            },
            WindowEvent::RedrawRequested => {
                let now = std::time::Instant::now();
                if let Some(last_time) = self.last_frame_time {
                    ctx.dt = (now - last_time).as_secs_f32();
                }
                self.last_frame_time = Some(now);
                ctx.frame_count += 1;

                if let Some(game_logic) = &mut self.game_logic {
                    game_logic.update(ctx);
                    game_logic.render(ctx);
                }

                ctx.window.request_redraw();
            }
            _ => {
                ctx.event_system.handle_window_event(&event, ctx.window.scale_factor());
                ctx.ui.handle_event(&event, ctx.window.scale_factor());
            }
        }
    }
}

// pub fn run() -> anyhow::Result<()> {
//     #[cfg(not(target_arch = "wasm32"))]
//     {
//         env_logger::init();
//     }
//     #[cfg(target_arch = "wasm32")]
//     {
//         console_log::init_with_level(log::Level::Info).unwrap_throw();
//     }

//     let event_loop = EventLoop::with_user_event().build()?;
//     let mut app = App::new(
//         #[cfg(target_arch = "wasm32")]
//         &event_loop,
//     );
//     event_loop.run_app(&mut app)?;

//     Ok(())
// }

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen(start)]
// pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
//     console_error_panic_hook::set_once();
//     run().unwrap_throw();

//     Ok(())
// }