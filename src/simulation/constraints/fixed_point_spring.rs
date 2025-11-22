use crate::{core::math::vec2::Vec2, simulation::particles::{particle::Particle, particle_vec::ParticleVec}};


#[derive(Debug, Copy, Clone)]
pub struct FixedPointSpring {
    pub particle_index: usize,
    pub target_pos: Vec2,
    pub stiffness_factor: f32, // stiffness_factor. 0 = fully stiff, any value > 0 is a % per second?
    pub is_enabled: bool
}

pub struct FixedPointSpringVec(Vec<FixedPointSpring>);

/**
 * k: Spring stiffness constant (higher = stronger pull).
    damping: Damping coefficient (higher = more resistance to motion).
            let damping = 100.0;
            let k = 10000.0;

// // Update velocity based on how far away from the point we are.
            // // If this is to support Verlet, should we calculate an acceleration?
 */
pub fn compute_velocity_to_spring_to_target_position(damping: f32, k: f32, current_pos: Vec2, current_vel: Vec2, target_pos: Vec2, mass: f32, time_delta: f32) -> Vec2 {
    // Update velocity based on how far away from the point we are.
    // If this is to support Verlet, should we calculate an acceleration?
    let displacement = current_pos - target_pos;
    let spring_force = -displacement * k; // Hooke's law: F = -k * x (rest length 0)
    let damping_force = -current_vel * damping;
    let total_force = spring_force + damping_force;
    let acceleration = total_force / mass; // Since mass = 1, a = F. F = ma, a = F/m
    let vel = current_vel + acceleration * time_delta;
    vel
}

impl FixedPointSpringVec {
    pub fn execute(&mut self, ps: &mut ParticleVec, time_delta: f32) {
        for i in 0..self.0.len() {
            let spring = &self.0[i];
            if !spring.is_enabled {
                continue;
            }

            // - k: Spring stiffness constant (higher = stronger pull).
            // - damping: Damping coefficient (higher = more resistance to motion).
            let damping = 100.0;
            let k = 10000.0;

            let particle = &mut ps[spring.particle_index];
            let displacement = particle.pos - spring.target_pos;
            let spring_force = -displacement * k; // Hooke's law: F = -k * x (rest length 0)
            let damping_force = -particle.vel * damping;
            let total_force = spring_force + damping_force;
            let acceleration = total_force / particle.mass; // Since mass = 1, a = F. F = ma, a = F/m
            let vel_old = particle.vel + acceleration * time_delta;

            let vel = compute_velocity_to_spring_to_target_position(damping, k, particle.pos, particle.vel, spring.target_pos, particle.mass, time_delta);
            
            debug_assert!(vel_old.x == vel.x);
            debug_assert!(vel_old.y == vel.y);

            particle.set_vel(vel);
        }
    }

    pub fn from_existing_particle_positions(ps: &[Particle]) -> Self {
        let mut se = Self(Vec::<FixedPointSpring>::new());

        for (i, particle) in ps.iter().enumerate() {
            se.0.push(FixedPointSpring {
                particle_index: particle.index,
                target_pos: particle.pos,
                stiffness_factor: 0.0,
                is_enabled: true
            });
        }

        se
    }
}



#[cfg(test)]
mod tests {
    use crate::core::math::vec2::Vec2;
    use super::*;

    #[test]
    fn from_constructor() {
        let fps = FixedPointSpringVec::from_existing_particle_positions(&[Particle::default(), Particle::default()]);
        assert_eq!(fps.0.len(), 2);
    }
}