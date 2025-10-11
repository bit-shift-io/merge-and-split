use crate::{level::level_entity::{LevelEntity, UpdateContext}, particles::particle_vec::ParticleVec};

pub struct Level {
    pub entities: Vec<Box<dyn LevelEntity>>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            entities: vec![],
        }
    }

    pub fn update(&mut self, particle_vec: &mut ParticleVec, time_delta: f32) {
        let mut context = UpdateContext {
            time_delta,
            particle_vec,
            //level: self,
        };
        for entity in self.entities.iter_mut() {
            entity.update(&mut context);
        }
    }
}