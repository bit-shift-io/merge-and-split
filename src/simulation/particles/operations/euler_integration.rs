use crate::{core::math::vec2::Vec2, simulation::particles::{operations::operation::Operation, particle_vec::ParticleVec}};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EulerIntegration {
    pub time_delta: f32, // really we should get this from an OperationContext?
    pub gravity: Vec2,
}

// Euler integration does not conserve energy. I think timesteps determines energy add or loss?
impl EulerIntegration {
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

impl Operation for EulerIntegration {
    fn execute(&mut self, ps: &mut ParticleVec) {
        let particle_count: usize = ps.len();
        for ai in 0..particle_count {
            let p1 = &mut ps[ai];

            // Static and merged particles do not get moved.
            if p1.is_static || p1.is_merged {
                continue;
            }
            
            let force = p1.mass * self.gravity; // F = ma
            let vel = force * self.time_delta;
            p1.vel += vel;

            // todo: Assert that a particle has not moved more than its radius in a timestep, if so we have a problem!
            
            p1.pos += p1.vel * self.time_delta;

            if p1.debug {
                println!("EulerIntegration {}", p1);
            }
        }
    }
}

impl Default for EulerIntegration {
    fn default() -> Self {
        Self {
            time_delta: 0.0,
            gravity: Vec2::new(0.0, -9.8)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::simulation::particles::particle::Particle;

    use super::*;

    #[test]
    fn execute() {
        let mut ps = ParticleVec::new();
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        ps.push(p1);

        // Move by 1 time step.
        let mut o = *EulerIntegration::default().set_time_delta(1.0).set_gravity(Vec2::new(0.0, 0.0));
        o.execute(&mut ps);
        assert_eq!(ps[0].pos, Vec2::new(0.1, 0.0));

        // MOve by 0.5m time steps.
        o.set_time_delta(0.5);
        o.execute(&mut ps);
        assert_eq!(ps[0].pos, Vec2::new(0.15, 0.0));
    }
}