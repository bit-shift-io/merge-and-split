use crate::{engine::app::app::App, game::event::event_system::GameEvent};

pub trait Plugin: Sized {
    fn new(state: &crate::engine::app::state::State) -> Self;
    fn window_event(&mut self, app: &mut App<Self>, event: GameEvent);
    fn update(&mut self, app: &mut App<Self>);
    fn render(&self, app: &mut App<Self>);
    fn resize(&mut self, app: &mut App<Self>, width: u32, height: u32);
}
