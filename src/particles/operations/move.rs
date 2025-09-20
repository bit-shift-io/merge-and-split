use cgmath::InnerSpace;

use crate::{math::vec2::Vec2, particles::{operations::operation::Operation, particle_vec::ParticleVec}};



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub time_delta: f32, // really we should get this from an OperationContext?
    pub gravity: Vec2,
}

// This is also known as "integration". Move and apply any gravity/force.
impl Move {
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

impl Operation for Move {
    fn execute(&self, ps: &mut ParticleVec) {
        let particle_count: usize = ps.len();
        for ai in 0..particle_count {
            let p1 = &mut ps[ai];
            if p1.is_static {
                continue;
            }
            
            let force = p1.mass * self.gravity; // F = ma
            let vel = force * self.time_delta;
            p1.vel += vel;

            p1.pos += p1.vel * self.time_delta;
        }
    }
}

impl Default for Move {
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
        let mut ps = ParticleVec::default();
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        ps.push(p1);

        // Move by 1 time step.
        let mut o = *Move::default().set_time_delta(1.0).set_gravity(Vec2::new(0.0, 0.0));
        o.execute(&mut ps);
        assert_eq!(ps[0].pos, Vec2::new(0.1, 0.0));

        // MOve by 0.5m time steps.
        o.set_time_delta(0.5);
        o.execute(&mut ps);
        assert_eq!(ps[0].pos, Vec2::new(0.15, 0.0));
    }
}


// // Simple integration: gravity only, explicit Euler
// fn integrate(metas: &mut Vec<MetaParticle>, dt: f64) {
//     let gravity = [0.0, 0.0, -9.8];
//     for meta in metas.iter_mut() {
//         let a = gravity; // Force computation stub; in real, sum forces / mass
//         let mut vel = meta.get_velocity();
//         vel = add(vel, scale(dt, a));
//         let mut pos = meta.get_position();
//         pos = add(pos, scale(dt, vel));
//         meta.set_velocity(vel);
//         meta.set_position(pos);
//     }
// }