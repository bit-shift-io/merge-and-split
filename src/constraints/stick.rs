use cgmath::InnerSpace;

use crate::{constraints::fixed_point_spring::compute_velocity_to_spring_to_target_position, math::vec2::Vec2, particles::{particle::Particle, particle_vec::{ParticleHandle, ParticleVec}}};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stick {
    pub particle_indicies: [usize; 2],
    pub length: f32,
    pub stiffness_factor: f32, // stiffness_factor. 0 = fully stiff, any value > 0 is a % per second?
    pub is_enabled: bool
}


impl Stick {
    pub fn set_stiffness_factor(&mut self, stiffness_factor: f32) -> &mut Self {
        self.stiffness_factor = stiffness_factor;
        self
    }

    pub fn set_particle_handles(&mut self, particle_handles: [ParticleHandle; 2]) -> &mut Self {
        self.particle_indicies = particle_handles;
        self
    }

    pub fn set_length(&mut self, length: f32) -> &mut Self {
        self.length = length;
        self
    }
}

impl Default for Stick {
    fn default() -> Self {
        Self {
            particle_indicies: [usize::MAX; 2],
            length: 0.0,
            stiffness_factor: 0.0,
            is_enabled: true,
        }
    }
}


fn compute_movement_weight(a_is_static: bool, b_is_static: bool) -> (f32, f32) {
    // movement weight is used to stop static objects being moved
    let a_movement_weight = if a_is_static { 0.0f32 } else if b_is_static { 1.0f32 } else { 0.5f32 };
    let b_movement_weight = 1.0f32 - a_movement_weight;
    (a_movement_weight, b_movement_weight)
}

pub struct StickVec(Vec<Stick>);

impl StickVec {
    pub fn new() -> Self {
        Self {
            0: vec![],
        }
    }

    pub fn push(&mut self, value: Stick) {
        self.0.push(value);
    }

    pub fn extend(&mut self, other_vec: &StickVec) {
        self.0.extend(other_vec.0.clone()); // Is there a non-clone way to do this?
    }

    pub fn execute(&mut self, ps: &mut ParticleVec, time_delta: f32) {
        for i in 0..self.0.len() {
            let stick = &self.0[i];
            if !stick.is_enabled {
                continue;
            }

            let particle_a = &ps[stick.particle_indicies[0]];
            let particle_b = &ps[stick.particle_indicies[1]];

            // todo: change this to consider the weight of each particle
            let (a_movement_weight, b_movement_weight) = compute_movement_weight(particle_a.is_static, particle_b.is_static);
                    
            let difference = particle_a.pos - particle_b.pos;
            let diff_length = difference.magnitude();
            let diff_factor = (stick.length - diff_length) / diff_length * 0.5;
            debug_assert!(!diff_factor.is_nan());
            let mut offset = difference * diff_factor;
            
            let damping = 10.0;
            let k = 10.0;


            // this bit makes it more like a spring
            if stick.stiffness_factor != 0.0 {
                offset *= time_delta * stick.stiffness_factor;
            }

            let particle_a_target_pos = particle_a.pos + (offset * a_movement_weight);
            let particle_b_target_pos = particle_b.pos - (offset * b_movement_weight);

            // if (offset.x > 0.0 || offset.y > 0.0) {
            //     println!("offset");
            // }

            // // setup the velocity so it till within the time_delta, reach the target position
            // let particle_a_vel = (offset * a_movement_weight) / time_delta;
            // let particle_b_vel = -(offset * b_movement_weight) / time_delta;

            let particle_a_vel = compute_velocity_to_spring_to_target_position(damping, k, particle_a.pos, particle_a.vel, particle_a_target_pos, particle_a.mass, time_delta);
            let particle_b_vel = compute_velocity_to_spring_to_target_position(damping, k, particle_b.pos, particle_b.vel, particle_b_target_pos, particle_b.mass, time_delta);
            
            {
                let particle_a_mut = &mut ps[stick.particle_indicies[0]];
                particle_a_mut.set_vel(particle_a_vel);
            }
            {
                let particle_b_mut = &mut ps[stick.particle_indicies[1]];
                particle_b_mut.set_vel(particle_b_vel);
            }

            // This old code (below) directly moves the particles, which is bad as it causes penetrations
            // Above, I've changed it to work more like a spring to push the particles into position using velocity.

            // // this bit makes it more like a spring
            // if stick.stiffness_factor != 0.0 {
            //     offset *= time_delta * stick.stiffness_factor;
            // }

            // {
            //     let p1mut = &mut ps[stick.particle_indicies[0]];
            //     p1mut.pos += offset * a_movement_weight;
            //     debug_assert!(!p1mut.pos.x.is_nan());
            //     debug_assert!(!p1mut.pos.y.is_nan());
            // }

            // {
            //     let p2mut = &mut ps[stick.particle_indicies[1]];
            //     p2mut.pos -= offset * b_movement_weight;
            //     debug_assert!(!p2mut.pos.x.is_nan());
            //     debug_assert!(!p2mut.pos.y.is_nan());
            // }
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::math::vec2::Vec2;
    use super::*;

    
    #[test]
    fn new() {
        let s = StickVec::new();
        assert_eq!(s.0, vec![]);
    }
}