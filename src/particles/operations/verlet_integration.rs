use cgmath::InnerSpace;

use crate::{math::vec2::Vec2, particles::{operations::operation::Operation, particle_vec::ParticleVec}};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VerletIntegration {
    pub time_delta: f32, // really we should get this from an OperationContext?
    pub gravity: Vec2,
}

// Verlet integration conserves energy (Euler does not).
impl VerletIntegration {
    pub fn set_time_delta(&mut self, time_delta: f32) -> &mut Self {
        debug_assert!(!time_delta.is_nan());
        debug_assert!(time_delta > 0.0);
        self.time_delta = time_delta;
        self
    }

    pub fn set_gravity(&mut self, gravity: Vec2) -> &mut Self {
        debug_assert!(!gravity.x.is_nan());
        debug_assert!(!gravity.y.is_nan());
        self.gravity = gravity;
        self
    }
}

impl Operation for VerletIntegration {
    fn execute(&mut self, ps: &mut ParticleVec) {
        let particle_count: usize = ps.len();
        for ai in 0..particle_count {
            let p1 = &mut ps[ai];

            // Static and merged particles do not get moved.
            if p1.is_static || p1.is_merged {
                continue;
            }
            
            let force = self.gravity * p1.mass;
            let accel = force / p1.mass; // F = ma, rearranged to a = F / m

            // Update position
            let pos_new = p1.pos + p1.vel * self.time_delta + 0.5 * accel * self.time_delta.powi(2);

            // New acceleration (constant for gravity, but computed for generality)
            let a_new = accel; //force(x_new) / m
    
            // Update velocity
            let vel_new = p1.vel + 0.5 * (accel + a_new) * self.time_delta;

            // todo: Assert that a particle has not moved more than its radius in a timestep, if so we have a problem!
            
            p1.set_pos(pos_new).set_vel(vel_new);

            if p1.debug {
                println!("VerletIntegration {}", p1);
            }
        }
    }
}

impl Default for VerletIntegration {
    fn default() -> Self {
        Self {
            time_delta: 0.0,
            gravity: Vec2::new(0.0, -9.8)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{math::vec2::Vec2, particles::particle::Particle};

    use super::*;

    #[test]
    fn execute() {
        let mut ps = ParticleVec::new();
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        ps.push(p1);

        // Move by 1 time step.
        let mut o = *VerletIntegration::default().set_time_delta(1.0).set_gravity(Vec2::new(0.0, 0.0));
        o.execute(&mut ps);
        assert_eq!(ps[0].pos, Vec2::new(0.1, 0.0));

        // Move by 0.5m time steps.
        o.set_time_delta(0.5);
        o.execute(&mut ps);
        assert_eq!(ps[0].pos, Vec2::new(0.15, 0.0));
    }
}
