use std::sync::Arc;

use winit::{
    application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::Window
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::engine::{app::{camera::{Camera, CameraController, CameraUniform}, plugin::Plugin, state::State}, renderer::texture};

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    pub state: Option<State>,
    event_loop: Option<EventLoop<State>>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            proxy: None,
            state: None,
            event_loop: None,
            plugins: vec![],
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) -> &mut Self {
        self.plugins.push(plugin);
        self
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

        // "with_user_event" lets us manually send events, or WASM send events to our event handler (https://yutani.rbind.io/post/winit-and-r/).
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

    fn on_state_set(&mut self) {
        // Take the plugins out temporarily to avoid double mutable borrow
        let mut plugins = std::mem::take(&mut self.plugins);
        for plugin in plugins.iter_mut() {
            plugin.init(self);
        }
        self.plugins = plugins;
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        // todo: remove this!
        let state = match &mut self.state {
            Some(s) => s,
            None => return,
        };
        state.handle_key(event_loop, key, pressed);

        // Take the plugins out temporarily to avoid double mutable borrow
        let mut plugins = std::mem::take(&mut self.plugins);
        for plugin in plugins.iter_mut() {
            plugin.handle_key(self, key, pressed);
        }
        self.plugins = plugins;
    }

    pub fn render(&mut self) {

        // Take the plugins out temporarily to avoid double mutable borrow
        let mut plugins = std::mem::take(&mut self.plugins);
        for plugin in plugins.iter_mut() {
            plugin.render(self);
        }
        self.plugins = plugins;


        // match self.render_internal() /*state.render()*/ {
        //     Ok(_) => {}
        //     // Reconfigure the surface if it's lost or outdated
        //     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
        //         let state = match &mut self.state {
        //             Some(s) => s,
        //             None => return,
        //         };

        //         let size = state.window.inner_size();
        //         state.resize(size.width, size.height);
        //     }
        //     Err(e) => {
        //         log::error!("Unable to render {}", e);
        //     }
        // }
    }
}

impl ApplicationHandler<State> for App {
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
            // If we are not on web we can use pollster to
            // await the
            self.state = Some(pollster::block_on(State::new(window)).unwrap());
            self.on_state_set();
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(proxy
                        .send_event(
                            State::new(window)
                                .await
                                .expect("Unable to create canvas!!!")
                        )
                        .is_ok())
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(
                event.window.inner_size().width,
                event.window.inner_size().height,
            );
        }
        self.state = Some(event);
        self.on_state_set();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(s) => s,
            None => return,
        };

        // {
        //     // Take the plugins out temporarily to avoid double mutable borrow
        //     let mut plugins = std::mem::take(&mut self.plugins);
        //     for plugin in plugins.iter_mut() {
        //         plugin.window_event(self, event.clone());
        //     }
        //     self.plugins = plugins;
        // }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                state.resize(size.width, size.height);

                {
                    // Take the plugins out temporarily to avoid double mutable borrow
                    let mut plugins = std::mem::take(&mut self.plugins);
                    for plugin in plugins.iter_mut() {
                        plugin.resize(self, size.width, size.height);
                    }
                    self.plugins = plugins;
                }
            },
            WindowEvent::RedrawRequested => {
                {
                    // Take the plugins out temporarily to avoid double mutable borrow
                    let mut plugins = std::mem::take(&mut self.plugins);
                    for plugin in plugins.iter_mut() {
                        plugin.update(self);
                    }
                    self.plugins = plugins;
                }

                // let state = match &mut self.state {
                //     Some(s) => s,
                //     None => return,
                // };
                // state.update();

                self.render();

                // match state.render() {
                //     Ok(_) => {}
                //     // Reconfigure the surface if it's lost or outdated
                //     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                //         let size = state.window.inner_size();
                //         state.resize(size.width, size.height);
                //     }
                //     Err(e) => {
                //         log::error!("Unable to render {}", e);
                //     }
                // }
            }
            WindowEvent::MouseInput { state, button, .. } => match (button, state.is_pressed()) {
                (MouseButton::Left, true) => {}
                (MouseButton::Left, false) => {}
                _ => {}
            },
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => self.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
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