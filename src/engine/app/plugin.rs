use crate::{engine::app::app::App, game::event::event_system::GameEvent};

pub trait Plugin: Sized {
    fn new(state: &crate::engine::app::state::State) -> Self;
    fn window_event(&mut self, app: &mut App<Self>, event: &winit::event::WindowEvent);
    fn update(&mut self, app: &mut App<Self>);
    fn render(&mut self, app: &mut App<Self>);
    fn resize(&mut self, app: &mut App<Self>, width: u32, height: u32);
}
