use cgmath::InnerSpace;

use crate::{math::{float::float_approx_equal, vec2::Vec2}, particles::{operations::operation::Operation, particle_vec::ParticleVec}};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Metrics {
    pub momentum_magnitude: f32,
}

impl Metrics {
    pub fn approx_equal(&self, other: &Metrics) -> bool {
        float_approx_equal(self.momentum_magnitude, other.momentum_magnitude, f32::EPSILON)
    }
}

impl Operation for Metrics {
    fn execute(&mut self, ps: &mut ParticleVec) {
        let particle_count: usize = ps.len();
        let mut momentum_magnitude = 0.0;
        for ai in 0..particle_count {
            let p1 = &ps[ai];

            // Static and merged particles are ignored.
            if p1.is_static || p1.is_merged {
                continue;
            }
            
            let momentum = p1.vel * p1.mass;
            momentum_magnitude += momentum.magnitude(); // leave this mag2 and sqrt later.

            if p1.debug {
                println!("Metrics {}", p1);
            }
        }

        // This helps us see if energy is being added or removed into the system.
        println!("Metrics Momentum Magnitude: {} -> {}", self.momentum_magnitude, momentum_magnitude);
        self.momentum_magnitude = momentum_magnitude;
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            momentum_magnitude: 0.0
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

        let p2 = *Particle::default().set_vel(Vec2::new(-0.1, 0.0));
        ps.push(p2);

        let mut met = Metrics::default();
        met.execute(&mut ps);
        assert_eq!(met.momentum_magnitude, 0.2);
    }
}
