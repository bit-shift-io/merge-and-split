// use crate::{core::math::{float::float_approx_equal, vec2::Vec2}, simulation::particles::particle_vec::{ParticleHandle, ParticleVec}};

// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Stick {
//     pub particle_indicies: [usize; 2],
//     pub length: f32,
//     pub stiffness_factor: f32, // stiffness_factor. Higher is stiffer, values around 0.1-0.3 often work well to avoid overshoot or instability.
//     pub is_enabled: bool
// }


// impl Stick {
//     pub fn from_particles(particle_vec: &ParticleVec, particle_handles: [ParticleHandle; 2]) -> Self {
//         let p1 = &particle_vec[particle_handles[0]];
//         let p2 = &particle_vec[particle_handles[1]];
//         let length = (p1.pos - p2.pos).magnitude();
//         Self {
//             particle_indicies: [particle_handles[0], particle_handles[1]],
//             length,
//             stiffness_factor: 1.0,
//             is_enabled: true,
//         }
//     }

//     pub fn set_stiffness_factor(&mut self, stiffness_factor: f32) -> &mut Self {
//         self.stiffness_factor = stiffness_factor;
//         self
//     }

//     pub fn set_particle_handles(&mut self, particle_handles: [ParticleHandle; 2]) -> &mut Self {
//         self.particle_indicies = particle_handles;
//         self
//     }

//     pub fn set_length(&mut self, length: f32) -> &mut Self {
//         self.length = length;
//         self
//     }
// }

// impl Default for Stick {
//     fn default() -> Self {
//         Self {
//             particle_indicies: [usize::MAX; 2],
//             length: 0.0,
//             stiffness_factor: 0.0,
//             is_enabled: true,
//         }
//     }
// }


// fn compute_movement_weight(a_is_static: bool, b_is_static: bool) -> (f32, f32) {
//     // movement weight is used to stop static objects being moved
//     let a_movement_weight = if a_is_static { 0.0f32 } else if b_is_static { 1.0f32 } else { 0.5f32 };
//     let b_movement_weight = 1.0f32 - a_movement_weight;
//     (a_movement_weight, b_movement_weight)
// }


// // import numpy as np

// // def compute_velocity_corrections(pos_a, pos_b, vel_a, vel_b, target_dist, dt, alpha=0.2, masses=None):
// //     """
// //     Compute velocity corrections to add to particles A and B to maintain a target distance.
    
// //     This enforces the distance constraint at the velocity level with Baumgarte stabilization
// //     for position error correction. The velocity delta includes a term to gradually correct
// //     any deviation from the target distance.
    
// //     Parameters:
// //     - pos_a, pos_b: numpy arrays for positions of A and B (same shape, e.g., (3,) for 3D).
// //     - vel_a, vel_b: numpy arrays for velocities of A and B (same shape as positions).
// //     - target_dist: The fixed distance to maintain (float).
// //     - dt: Time step size (float), used for stabilization.
// //     - alpha: Baumgarte stabilization coefficient (float, default 0.2; typical range 0.1-0.3).
// //     - masses: optional tuple (m_a, m_b). If None, assumes equal masses (m_a = m_b = 1).
    
// //     Returns:
// //     - delta_vel_a: correction to add to vel_a.
// //     - delta_vel_b: correction to add to vel_b.
// //     """
// //     r = pos_b - pos_a
// //     r_mag = np.linalg.norm(r)
// //     if r_mag == 0:
// //         return np.zeros_like(vel_a), np.zeros_like(vel_b)
    
// //     n = r / r_mag
// //     v_rel = vel_b - vel_a
// //     radial_vel = np.dot(n, v_rel)
    
// //     C = r_mag - target_dist  # Position error
// //     beta = alpha / dt
// //     target_radial_vel = -beta * C  # Bias for position correction
    
// //     delta_radial = target_radial_vel - radial_vel
// //     delta_v_rel = delta_radial * n
    
// //     if masses is None:
// //         m_a = m_b = 1.0
// //     else:
// //         m_a, m_b = masses
// //     total_mass = m_a + m_b
    
// //     # Conserve momentum: delta_vel_a = - (m_b / total_mass) * delta_v_rel
// //     # delta_vel_b = (m_a / total_mass) * delta_v_rel
// //     delta_vel_a = - (m_b / total_mass) * delta_v_rel
// //     delta_vel_b = (m_a / total_mass) * delta_v_rel
    
// //     return delta_vel_a, delta_vel_b

// // # Example usage:
// // # Assume 2D positions and velocities, target distance 1.0, but current is 1.1 (drifted apart)
// // pos_a = np.array([0.0, 0.0])
// // pos_b = np.array([1.1, 0.0])
// // vel_a = np.array([0.0, 0.0])
// // vel_b = np.array([0.1, 0.0])  # Slightly separating
// // target_dist = 1.0
// // dt = 0.1

// // delta_a, delta_b = compute_velocity_corrections(pos_a, pos_b, vel_a, vel_b, target_dist, dt)
// // new_vel_a = vel_a + delta_a
// // new_vel_b = vel_b + delta_b

// // print("Delta vel A:", delta_a)
// // print("Delta vel B:", delta_b)
// // print("New vel A:", new_vel_a)
// // print("New vel B:", new_vel_b)
// // # Expected: Corrections pull them together to correct the 0.1 excess distance over time.

// // Uses Baumgarte stabilization.
// // Tune alpha based on your simulation; values around 0.1-0.3 often work well to avoid overshoot or instability.
// fn compute_velocity_corrections(pos_a: Vec2, pos_b: Vec2, vel_a: Vec2, vel_b: Vec2, mass_a: f32, mass_b: f32, target_dist: f32, dt: f32, alpha: f32) -> (Vec2, Vec2) {
//     let r = pos_b - pos_a;
//     let r_mag = r.magnitude();
//     if float_approx_equal(r_mag, 0.0, f32::EPSILON) {
//         return (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
//     }

//     let n = r / r_mag;
//     let v_rel = vel_b - vel_a;
//     let radial_vel = n.dot(v_rel);

//     let c = r_mag - target_dist; // position error
//     let beta = alpha / dt;
//     let target_radial_vel = -beta * c; // bias for position correction

//     let delta_radial = target_radial_vel - radial_vel;
//     let delta_v_rel = delta_radial * n;

//     let total_mass = mass_a + mass_b;

//     // conserve momentum: delta_vel_a = - (m_b / total_mass) * delta_v_rel
//     // delta_vel_b = (m_a / total_mass) * delta_v_rel
//     let delta_vel_a = - (mass_b / total_mass) * delta_v_rel;
//     let delta_vel_b = (mass_a / total_mass) * delta_v_rel;

//     (delta_vel_a, delta_vel_b)
   
// }


// pub struct StickVec(Vec<Stick>);

// impl StickVec {
//     pub fn new() -> Self {
//         Self {
//             0: vec![],
//         }
//     }

//     pub fn push(&mut self, value: Stick) {
//         self.0.push(value);
//     }

//     pub fn extend(&mut self, other_vec: &StickVec) {
//         self.0.extend(other_vec.0.clone()); // Is there a non-clone way to do this?
//     }

//     pub fn execute(&mut self, ps: &mut ParticleVec, time_delta: f32) {
//         for i in 0..self.0.len() {
//             let stick = &self.0[i];
//             if !stick.is_enabled {
//                 continue;
//             }

//             let particle_a = &ps[stick.particle_indicies[0]];
//             let particle_b = &ps[stick.particle_indicies[1]];

//             let (delta_vel_a, delta_vel_b) = compute_velocity_corrections(
//                 particle_a.pos, particle_b.pos, particle_a.vel, particle_b.vel, 
//                 particle_a.mass, particle_b.mass,
//                 stick.length, time_delta, stick.stiffness_factor);

//             // // todo: change this to consider the weight of each particle
//             // let (a_movement_weight, b_movement_weight) = compute_movement_weight(particle_a.is_static, particle_b.is_static);
                    
//             // let difference = particle_a.pos - particle_b.pos;
//             // let diff_length = difference.magnitude();
//             // let diff_factor = (stick.length - diff_length) / diff_length * 0.5;
//             // if float_approx_equal(diff_factor, 0.0, f32::EPSILON) {
//             //     continue; // only correct if the stick is compressed or stretched
//             // }
//             // debug_assert!(!diff_factor.is_nan());
//             // let mut offset = difference * diff_factor;
            
//             // let damping = 10.0;
//             // let k = 10.0;


//             // // this bit makes it more like a spring
//             // if stick.stiffness_factor != 0.0 {
//             //     offset *= time_delta * stick.stiffness_factor;
//             // }

//             // // I have a particle A and B.
//             // // Both particles have their own position and velocity.
//             // // I want to compute a velocity to add to the existing velocity value such that they maintain a fixed distance between the two particles.

//             // let particle_a_target_pos = particle_a.pos + (offset * a_movement_weight) + (particle_a.vel * time_delta);
//             // let particle_b_target_pos = particle_b.pos - (offset * b_movement_weight) + (particle_b.vel * time_delta);

//             // if (offset.x > 0.0 || offset.y > 0.0) {
//             //     println!("offset");
//             // }

//             // // setup the velocity so it till within the time_delta, reach the target position
//             // let particle_a_vel = (offset * a_movement_weight) / time_delta;
//             // let particle_b_vel = -(offset * b_movement_weight) / time_delta;

//             // // let particle_a_vel = compute_velocity_to_spring_to_target_position(damping, k, particle_a.pos, particle_a.vel, particle_a_target_pos, particle_a.mass, time_delta);
//             // // let particle_b_vel = compute_velocity_to_spring_to_target_position(damping, k, particle_b.pos, particle_b.vel, particle_b_target_pos, particle_b.mass, time_delta);
            
//             {
//                 let particle_a_mut = &mut ps[stick.particle_indicies[0]];
//                 particle_a_mut.add_vel(delta_vel_a);
//             }
//             {
//                 let particle_b_mut = &mut ps[stick.particle_indicies[1]];
//                 particle_b_mut.add_vel(delta_vel_b);
//             }

//             // This old code (below) directly moves the particles, which is bad as it causes penetrations
//             // Above, I've changed it to work more like a spring to push the particles into position using velocity.

//             // // this bit makes it more like a spring
//             // if stick.stiffness_factor != 0.0 {
//             //     offset *= time_delta * stick.stiffness_factor;
//             // }

//             // {
//             //     let p1mut = &mut ps[stick.particle_indicies[0]];
//             //     p1mut.pos += offset * a_movement_weight;
//             //     debug_assert!(!p1mut.pos.x.is_nan());
//             //     debug_assert!(!p1mut.pos.y.is_nan());
//             // }

//             // {
//             //     let p2mut = &mut ps[stick.particle_indicies[1]];
//             //     p2mut.pos -= offset * b_movement_weight;
//             //     debug_assert!(!p2mut.pos.x.is_nan());
//             //     debug_assert!(!p2mut.pos.y.is_nan());
//             // }
//         }
//     }
// }



// #[cfg(test)]
// mod tests {
//     use crate::core::math::vec2::Vec2;
//     use super::*;

    
//     #[test]
//     fn new() {
//         let s = StickVec::new();
//         assert_eq!(s.0, vec![]);
//     }
// }