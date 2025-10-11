use crate::{entity::entity::{Entity, UpdateContext}, particles::particle_vec::ParticleVec};


pub struct EntitySystem {
    pub entities: Vec<Box<dyn Entity>>,
}

impl EntitySystem {
    pub fn new() -> Self {
        Self {
            entities: vec![],
        }
    }

    pub fn push<T: Entity + 'static>(&mut self, entity: T) -> &mut Self {
        self.entities.push(Box::new(entity));
        self
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