use std::collections::HashSet;
use winit::event::{ElementState, WindowEvent, KeyEvent};
use winit::keyboard::KeyCode;
use crate::core::math::vec2::Vec2;

pub struct InputHelper {
    keys_pressed: HashSet<KeyCode>,
    pub mouse_position: Vec2,
}

impl InputHelper {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            mouse_position: Vec2::new(0.0, 0.0),
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn handle_event(&mut self, event: &WindowEvent, _scale_factor: f64) {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: winit::keyboard::PhysicalKey::Code(key_code),
                    state,
                    ..
                },
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        self.keys_pressed.insert(*key_code);
                    }
                    ElementState::Released => {
                        self.keys_pressed.remove(key_code);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = Vec2::new(position.x as f32, position.y as f32);
            }
            _ => {}
        }
    }
}
