use winit::keyboard::KeyCode;

use crate::{simulation::particles::{particle_vec::ParticleVec, simulation::Simulation}, engine::app::camera::Camera};


pub struct UpdateContext<'a> {
    pub particle_vec: &'a mut ParticleVec, //pub particle_sim: &'a mut ParticleSim,
    pub sim: &'a mut Simulation,
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

// pub trait EntityConstraintSolver {
//     fn update_counts(&mut self, sim: &mut Simulation);
//     fn solve_constraints(&mut self, sim: &mut Simulation, time_delta: f32);
// }

// pub trait EntityUpdate {
//     fn update(&mut self, context: &mut UpdateContext);
// }