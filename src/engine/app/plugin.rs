use winit::{event::WindowEvent, keyboard::KeyCode};

use crate::engine::app::app::App;

pub trait Plugin: Sized {
    fn new(state: &crate::engine::app::state::State) -> Self;
    fn window_event(&mut self, app: &mut App<Self>, event: WindowEvent);
    fn handle_key(&mut self, app: &mut App<Self>, key: KeyCode, pressed: bool);
    fn update(&mut self, app: &mut App<Self>);
    fn render(&self, app: &mut App<Self>);
    fn resize(&mut self, app: &mut App<Self>, width: u32, height: u32);
}
