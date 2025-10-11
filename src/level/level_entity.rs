use crate::{level::level::Level, particles::particle_vec::ParticleVec};

use super::level_builder::LevelBuilderContext;



pub struct UpdateContext<'a> {
    pub particle_vec: &'a mut ParticleVec, //pub particle_sim: &'a mut ParticleSim,
   // pub level: &'a mut Level,
    pub time_delta: f32,
}

pub trait LevelEntity {
    fn update(&mut self, context: &mut UpdateContext);
}