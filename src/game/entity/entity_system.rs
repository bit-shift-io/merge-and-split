use winit::{keyboard::KeyCode};

use crate::{engine::app::camera::Camera, game::{entity::{entities::car_entity::CarEntitySystem, entity::UpdateContext}, level::level_blocks::elevator::ElevatorEntitySystem}, simulation::particles::{particle_vec::ParticleVec, simulation::Simulation}};

pub struct EntitySystem {
    pub elevator_entity_system: ElevatorEntitySystem,
    pub car_entity_system: CarEntitySystem,
}

impl EntitySystem {
    pub fn new() -> Self {
        Self {
            elevator_entity_system: ElevatorEntitySystem::new(),
            car_entity_system: CarEntitySystem::new(),
        }
    }

    pub fn update(&mut self, particle_vec: &mut ParticleVec, sim: &mut Simulation, camera: &mut Camera, time_delta: f32) {
        let mut context = UpdateContext {
            time_delta,
            particle_vec,
            sim,
            camera,
        };

        self.elevator_entity_system.update(&mut context);
        self.car_entity_system.update(&mut context);
    }

    pub fn handle_key(&mut self, key: KeyCode, pressed: bool) {
        self.car_entity_system.handle_key(key, pressed);
    }
}