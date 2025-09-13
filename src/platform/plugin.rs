use winit::keyboard::KeyCode;

use crate::platform::app::App;

pub trait Plugin {
    fn init(&self, app: &mut App);
    fn handle_key(&mut self, app: &mut App, key: KeyCode, pressed: bool);
    fn update(&self, app: &mut App);
    fn render(&self, app: &mut App);
}
