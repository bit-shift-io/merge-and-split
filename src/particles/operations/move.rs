use cgmath::InnerSpace;

use crate::particles::{operations::operation::Operation, particle_vec::ParticleVec};



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub time_delta: f32 // really we should get this from an OperationContext?
}

// This is also known as "integration". Move and apply any gravity/force.
impl Move {
    pub fn set_time_delta(&mut self, time_delta: f32) -> &mut Self {
        debug_assert!(!time_delta.is_nan());
        debug_assert!(time_delta > 0.0);
        self.time_delta = time_delta;
        self
    }
}

impl Operation for Move {
    fn execute(&self, ps: &mut ParticleVec) {
        let particle_count: usize = ps.len();
        for ai in 0..particle_count {
            let p1 = &mut ps[ai];
            // todo: update p1.vel adding in any gravity/force component. OR make that a seperate operation?
            p1.pos += p1.vel * self.time_delta;
        }
    }
}

impl Default for Move {
    fn default() -> Self {
        Self {
            time_delta: 0.0
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{math::vec2::Vec2, particles::particle::Particle};

    use super::*;

    #[test]
    fn execute() {
        let mut ps = ParticleVec::default();
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        ps.push(p1);

        // Move by 1 time step.
        let mut o = *Move::default().set_time_delta(1.0);
        o.execute(&mut ps);
        assert_eq!(ps[0].pos, Vec2::new(0.1, 0.0));

        // MOve by 0.5m time steps.
        o.set_time_delta(0.5);
        o.execute(&mut ps);
        assert_eq!(ps[0].pos, Vec2::new(0.15, 0.0));
    }
}