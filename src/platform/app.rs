use winit::event_loop::EventLoop;

use crate::platform::app_inner::{AppInner, State};



pub struct App {
    app_inner: Option<AppInner>,
    event_loop: Option<EventLoop<State>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            app_inner: None,
            event_loop: None,
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