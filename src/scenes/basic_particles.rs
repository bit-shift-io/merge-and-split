use winit::keyboard::KeyCode;

use crate::platform::{app::App, camera::CameraController, plugin::Plugin};


pub struct BasicParticles {
    camera_controller: CameraController,
}

impl BasicParticles {
    pub fn new() -> Self {
        let camera_controller = CameraController::new(0.2);
        Self {
            camera_controller
        }
    }
}

impl Plugin for BasicParticles {
    fn init(&self, app: &mut App) {
        println!("basic particles init");
    }

    fn handle_key(&mut self, app: &mut App, key: KeyCode, pressed: bool) {
        println!("basic particles handle_key");
        self.camera_controller.handle_key(key, pressed);
    }

    fn update(&self, app: &mut App) {
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        self.camera_controller.update_camera(&mut state.camera);
    }

    fn render(&self, app: &mut App) {
        println!("basic particles render");
    }
}