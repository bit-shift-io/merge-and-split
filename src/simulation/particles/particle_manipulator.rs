use cgmath::InnerSpace;

use crate::{core::math::vec2::Vec2, simulation::particles::particle_vec::{ParticleHandle, ParticleVec}};


/**
 * Utility class to help with particle manipulation.
 */
pub struct ParticleManipulator {

}

impl ParticleManipulator {
    pub fn new() -> Self {
        Self{}
    }

    /**
     * Adds a rotational force to a set of particles around a given point.
     * 
     * Torque (T) is the rotational equivalent of force and is calculated as:
     * Torque = radius x Force x sin(theta)
     * since we apply force perpendicular to the radius, sin(theta) = 1, so:
     * Torque = radius x Force
     * 
     * We want to find the force to apply to each particle to achieve the desired torque:
     * Force = Torque / radius
     * 
     * Suppose you want to apply a torque T = 100 N·m to a wheel of radius R = 0.5m (points on the 
     * wheel range from r = 0 near the axle to r = 0.5 m at the rim).
     * At a point on the rim (r = 0.5m): F = 100 / 0.5 = 200 N.
     */
    pub fn add_torque_around_point(&self, particle_vec: &mut ParticleVec, particle_handles: &Vec<ParticleHandle>, pos: Vec2, torque: f32) {
        for particle_handle in particle_handles.iter() {
            let particle = &mut particle_vec[*particle_handle];
            let delta = particle.pos - pos;
            let radius = delta.magnitude();
            let force = torque / radius;
            let tangent = Vec2::new(-delta[1], delta[0]).normalize(); // compute a vector at 90 degress to delta

            let force = tangent * force;
            //println!("add force: {}", force);
            particle.add_force(force);
        }
    }
}