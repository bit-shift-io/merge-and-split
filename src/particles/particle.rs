
use crate::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    Particle,
    MetaParticle,
    MergedParticle, // i.e. this particle is hidden from the system as the meta particle is acting on its behalf.
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub mass: f32,

    pub particle_type: ParticleType,

    pub energy_delta: f32, // "the change in kinetic energy, ∆E, and store it as a potential energy in a virtual bond between the two colliding particles"
    pub n: Vec2,
    pub left_index: usize, // How many other particle are merged into this particle? 0 = No particles
    pub right_index: usize, // Index of the particle this is currently merged with. usize::MAX if not merged
}

impl Particle {
    // pub fn from_meta_particle(pos: Vec2, vel: Vec2, radius: f32, mass: f32) -> Self {
    //     debug_assert!(!pos.x.is_nan());
    //     debug_assert!(!pos.y.is_nan());
    //     Self { pos, vel, radius, mass }
    // }

    pub fn set_radius(&mut self, radius: f32) -> &mut Self {
        debug_assert!(!radius.is_nan());
        debug_assert!(radius > 0.0);
        self.radius = radius;
        self
    }

    pub fn set_mass(&mut self, mass: f32) -> &mut Self {
        debug_assert!(!mass.is_nan());
        self.mass = mass;
        self
    }

    pub fn set_pos(&mut self, pos: Vec2) -> &mut Self {
        debug_assert!(!pos.x.is_nan());
        debug_assert!(!pos.y.is_nan());
        self.pos = pos;
        //self.pos_prev = pos;
        self
    }

    pub fn set_vel(&mut self, vel: Vec2) -> &mut Self {
        debug_assert!(!vel.x.is_nan());
        debug_assert!(!vel.y.is_nan());
        self.vel = vel;
        //self.pos_prev = pos;
        self
    }

    pub fn set_energy_delta(&mut self, energy_delta: f32) -> &mut Self {
        debug_assert!(!energy_delta.is_nan());
        debug_assert!(!energy_delta.is_nan());
        self.energy_delta = energy_delta;
        self
    }

    pub fn set_n(&mut self, n: Vec2) -> &mut Self {
        debug_assert!(!n.x.is_nan());
        debug_assert!(!n.y.is_nan());
        self.n = n;
        self
    }

    pub fn set_particle_type(&mut self, particle_type: ParticleType) -> &mut Self {
        self.particle_type = particle_type;
        self
    }

    pub fn set_left_index(&mut self, left_index: usize) -> &mut Self {
        self.left_index = left_index;
        self
    }

    pub fn set_right_index(&mut self, right_index: usize) -> &mut Self {
        self.right_index = right_index;
        self
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
            radius: 0.5,
            mass: 1.0,

            particle_type: ParticleType::Particle,

            energy_delta: 0.0,
            n: Vec2::new(0.0, 0.0),
            left_index: usize::MAX,
            right_index: usize::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let p = Particle::default();
        assert_eq!(p.pos, Vec2::new(0.0, 0.0));
    }
}