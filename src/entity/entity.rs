use crate::particles::particle_vec::ParticleVec;


pub struct UpdateContext<'a> {
    pub particle_vec: &'a mut ParticleVec, //pub particle_sim: &'a mut ParticleSim,
   // pub level: &'a mut Level,
    pub time_delta: f32,
}

pub trait Entity {
    fn update(&mut self, context: &mut UpdateContext);
}