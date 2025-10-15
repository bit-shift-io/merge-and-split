
use std::{fmt, usize};

use cgmath::Array;

use crate::math::{vec2::Vec2, vec4::Vec4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    Particle,
    MetaParticle,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Particle {
    pub index: usize, // for debugging
    pub debug: bool, // for debugging

    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub is_static: bool,

    pub colour: Vec4,

    pub particle_type: ParticleType,
    pub is_merged: bool, // This is a meta particle that is merged with another meta particle, so it hidden from the system.

    // Stuff for meta particles:
    pub energy_delta: f32, // "the change in kinetic energy, ∆E, and store it as a potential energy in a virtual bond between the two colliding particles"
    pub n: Vec2,
    pub left_index: usize, // How many other particle are merged into this particle? 0 = No particles
    pub right_index: usize, // Index of the particle this is currently merged with. usize::MAX if not merged

    pub v_left_initial: Vec2,
    pub v_right_initial: Vec2,
}

impl Particle {
    pub fn set_static(&mut self, is_static: bool) -> &mut Self {
        self.is_static = is_static;
        self
    }

    pub fn set_index(&mut self, index: usize) -> &mut Self {
        self.index = index;
        self
    }

    pub fn set_debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }

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

    pub fn set_colour(&mut self, colour: Vec4) -> &mut Self {
        debug_assert!(!colour.x.is_nan());
        debug_assert!(!colour.y.is_nan());
        debug_assert!(!colour.z.is_nan());
        debug_assert!(!colour.w.is_nan());
        self.colour = colour;
        self
    }

    pub fn set_pos(&mut self, pos: Vec2) -> &mut Self {
        debug_assert!(!pos.x.is_nan());
        debug_assert!(!pos.y.is_nan());
        self.pos = pos;
        //self.pos_prev = pos;
        self
    }

    pub fn add_vel(&mut self, vel: Vec2) -> &mut Self {
        self.set_vel(self.vel + vel)
    }

    pub fn set_vel(&mut self, vel: Vec2) -> &mut Self {
        debug_assert!(!vel.x.is_nan());
        debug_assert!(!vel.y.is_nan());

        if self.debug {
            println!("set_vel called on {}, changing vel to:{},{}", self, vel[0], vel[1]);
        }

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

    pub fn set_merged(&mut self, is_merged: bool) -> &mut Self {
        self.is_merged = is_merged;
        self
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            index: usize::MAX,
            debug: false,

            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
            radius: 0.5,
            mass: 1.0,
            is_static: false,

            colour: Vec4::new(1.0, 1.0, 1.0, 1.0),

            particle_type: ParticleType::Particle,
            is_merged: false,

            energy_delta: 0.0,
            n: Vec2::new(0.0, 0.0),
            left_index: usize::MAX,
            right_index: usize::MAX,

            v_left_initial: Vec2::new(0.0, 0.0),
            v_right_initial: Vec2::new(0.0, 0.0)
        }
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(P:{} POS:{},{} VEL:{},{})", self.index, self.pos[0], self.pos[1], self.vel[0], self.vel[1])
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