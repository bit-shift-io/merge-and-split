use cgmath::InnerSpace;

use crate::{core::math::{float::float_approx_equal, vec2::Vec2}, simulation::particles::{operations::operation::Operation, particle_vec::ParticleVec}};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Metrics {
    pub kinetic_energy: f32, // Tests conservation of kinetic energy: ½ m₁v₁² + ½ m₂v₂² = ½ m₁v₁'² + ½ m₂v₂'²
    pub momentum: Vec2, // Tests conservation of momentum: m₁v₁ + m₂v₂ = m₁v₁' + m₂v₂' 
}

impl Metrics {
    pub fn approx_equal(&self, other: &Metrics) -> bool {
        let conserves_kinetic_energy = float_approx_equal(self.kinetic_energy, other.kinetic_energy, f32::EPSILON);
        let conserves_momentum_x = float_approx_equal(self.momentum[0], other.momentum[0], f32::EPSILON);
        let conserves_momentum_y = float_approx_equal(self.momentum[1], other.momentum[1], f32::EPSILON);
        return conserves_kinetic_energy && conserves_momentum_x && conserves_momentum_y;
    }
}

impl Operation for Metrics {
    fn execute(&mut self, ps: &mut ParticleVec) {
        self.kinetic_energy = 0.0;
        self.momentum = Vec2::new(0.0, 0.0);
        let particle_count: usize = ps.len();
        for ai in 0..particle_count {
            let p1 = &ps[ai];

            // Static and merged particles are ignored.
            if p1.is_static || p1.is_merged {
                continue;
            }
            
            let momentum = p1.vel * p1.mass;
            self.momentum += momentum;

            let kinetic_energy = p1.vel.magnitude2() * p1.mass * 0.5;
            self.kinetic_energy += kinetic_energy;

            if p1.debug {
                println!("Metrics {}", p1);
            }
        }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            kinetic_energy: 0.0,
            momentum: Vec2::new(0.0, 0.0)
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

        let p2 = *Particle::default().set_vel(Vec2::new(-0.1, 0.0));
        ps.push(p2);

        // met_expected = The expected output
        let mut met_expected = Metrics::default();
        met_expected.kinetic_energy = 0.01;
        met_expected.momentum = Vec2::new(0.0, 0.0);

        let mut met = Metrics::default();
        met.execute(&mut ps);
        assert!(met.approx_equal(&met_expected));
    }
}
