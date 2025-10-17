use winit::{event::WindowEvent, keyboard::KeyCode};

use crate::platform::app::App;

pub trait Plugin {
    fn init(&mut self, app: &mut App);
    fn window_event(&mut self, app: &mut App, event: WindowEvent);
    fn handle_key(&mut self, app: &mut App, key: KeyCode, pressed: bool);
    fn update(&mut self, app: &mut App);
    fn render(&self, app: &mut App);
    fn resize(&mut self, app: &mut App, width: u32, height: u32);
}
