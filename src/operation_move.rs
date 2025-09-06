use cgmath::InnerSpace;

use crate::{operation::Operation, particle::Particle, particle_system::ParticleSystem};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OperationMove {
    pub time_delta: f32 // really we should get this from an OperationContext?
}

impl OperationMove {
    pub fn set_time_delta(&mut self, time_delta: f32) -> &mut Self {
        debug_assert!(!time_delta.is_nan());
        debug_assert!(time_delta > 0.0);
        self.time_delta = time_delta;
        self
    }
}

impl Operation for OperationMove {
    fn execute(&self, ps: &mut ParticleSystem) {
        let particle_count: usize = ps.len();
        for ai in 0..particle_count {
            let p1 = &mut ps.particles[ai];
            p1.pos += p1.vel * self.time_delta;
        }
    }
}

impl Default for OperationMove {
    fn default() -> Self {
        Self {
            time_delta: 0.0
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{math::Vec2, particle_system::ParticleSystem};
    use super::*;

    #[test]
    fn execute() {
        let mut ps = ParticleSystem::default();
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        ps.particles.push(p1);

        // Move by 1 time step.
        let mut o = *OperationMove::default().set_time_delta(1.0);
        o.execute(&mut ps);
        assert_eq!(ps.particles[0].pos, Vec2::new(0.1, 0.0));

        // MOve by 0.5m time steps.
        o.set_time_delta(0.5);
        o.execute(&mut ps);
        assert_eq!(ps.particles[0].pos, Vec2::new(0.15, 0.0));
    }
}