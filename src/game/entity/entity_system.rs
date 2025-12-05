use crate::{engine::app::camera::Camera, game::{entity::entities::{car_entity::CarEntitySystem, finish_entity::FinishEntitySystem}, event::event_system::KeyCodeType, level::level_blocks::elevator::ElevatorEntitySystem}, simulation::particles::{particle_vec::ParticleVec, simulation::Simulation}};

pub struct UpdateContext<'a> {
    pub particle_vec: &'a mut ParticleVec,
    pub sim: &'a mut Simulation,
    pub time_delta: f32,
    pub total_time: f32,
    pub camera: &'a mut Camera
}


pub struct EntitySystem {
    pub elevator_entity_system: ElevatorEntitySystem,
    pub car_entity_system: CarEntitySystem,
    pub finish_entity_system: FinishEntitySystem,
}

impl EntitySystem {
    pub fn new() -> Self {
        Self {
            elevator_entity_system: ElevatorEntitySystem::new(),
            car_entity_system: CarEntitySystem::new(),
            finish_entity_system: FinishEntitySystem::new(),
        }
    }

    pub fn update(&mut self, particle_vec: &mut ParticleVec, sim: &mut Simulation, camera: &mut Camera, time_delta: f32, total_time: f32) {
        let mut context = UpdateContext {
            time_delta,
            total_time,
            particle_vec,
            sim,
            camera,
        };

        self.elevator_entity_system.update(&mut context);
        self.car_entity_system.update(&mut context, &self.finish_entity_system);
    }

    pub fn handle_key(&mut self, key: KeyCodeType, pressed: bool) {
        self.car_entity_system.handle_key(key, pressed);
    }
}