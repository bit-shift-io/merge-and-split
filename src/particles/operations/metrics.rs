use cgmath::InnerSpace;

use crate::{math::vec2::Vec2, particles::{operations::operation::Operation, particle_vec::ParticleVec}};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Metrics {
    pub total_velocity_magnitude: f32,
}

impl Metrics {
}

impl Operation for Metrics {
    fn execute(&mut self, ps: &mut ParticleVec) {
        let particle_count: usize = ps.len();
        let mut total_velocity_magnitude = 0.0;
        for ai in 0..particle_count {
            let p1 = &ps[ai];

            // Static and merged particles do not get moved.
            if p1.is_static || p1.is_merged {
                continue;
            }
            
            total_velocity_magnitude += p1.vel.magnitude(); // leave this mag2 and sqrt later.

            if p1.debug {
                println!("Metrics {}", p1);
            }
        }

        // This helps us see if energy is being added or removed into the system.
        println!("Metrics Total Vel Magnitude: {} -> {}", self.total_velocity_magnitude, total_velocity_magnitude);
        self.total_velocity_magnitude = total_velocity_magnitude;
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            total_velocity_magnitude: 0.0
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
        assert_eq!(met.total_velocity_magnitude, 0.2);
    }
}
