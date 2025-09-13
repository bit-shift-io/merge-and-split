use winit::event_loop::EventLoop;

use crate::platform::{app_inner::AppInner, plugin::Plugin, state::State};



pub struct App {
    app_inner: Option<AppInner>,
    event_loop: Option<EventLoop<State>>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            app_inner: None,
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
        self.app_inner = Some(AppInner::new(
            #[cfg(target_arch = "wasm32")]
            &self.event_loop,
        ));
        self.event_loop
            .take()
            .unwrap()
            .run_app(self.app_inner.as_mut().unwrap())?;

        Ok(())

    }
}