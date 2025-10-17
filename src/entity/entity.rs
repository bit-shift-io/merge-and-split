use winit::keyboard::KeyCode;

use crate::{particles::particle_vec::ParticleVec, platform::camera::Camera};


pub struct UpdateContext<'a> {
    pub particle_vec: &'a mut ParticleVec, //pub particle_sim: &'a mut ParticleSim,
   // pub level: &'a mut Level,
    pub time_delta: f32,
    pub camera: &'a mut Camera,
}

pub trait Entity {
    // Ugh, having this on every entity sucks. In future add subscribers or similar.
    fn update(&mut self, context: &mut UpdateContext);

    // Ugh, having this on every entity sucks. In future add subscribers or similar.
    fn handle_key(&mut self, key: KeyCode, is_pressed: bool) -> bool;
}


// pub trait EntityUpdate {
//     fn update(&mut self, context: &mut UpdateContext);
// }

// pub trait EntityInput {
//     fn handle_key(&mut self, key: KeyCode, is_pressed: bool) -> bool;
// }