use crate::{math::vec2::Vec2, particles::particle_vec::{ParticleHandle, ParticleVec}};




/**
 * Utility class to help with particle manipulation.
 */
pub struct ParticleManipulator {

}

impl ParticleManipulator {
    pub fn new() -> Self {
        Self{}
    }

    pub fn add_rotational_force_around_point(&self, particle_vec: &mut ParticleVec, particle_handles: &Vec<ParticleHandle>, pos: Vec2, force_magnitude: f32) {
        for particle_handle in particle_handles.iter() {
            let particle = &mut particle_vec[*particle_handle];
            let delta = particle.pos - pos;
            let adjacent = Vec2::new(-delta[1], delta[0]); // compute a vector at 90 degress to delta

            let force = adjacent * force_magnitude;
            //println!("add force: {}", force);
            particle.add_force(force);
        }
    }
}