
use std::{fmt, usize};

use cgmath::Array;

use crate::{core::math::{vec2::Vec2, vec4::Vec4}, simulation::particles::{body::Body, sdf_data::SdfData}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    Particle,
    MetaParticle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    Solid,
    Fluid,
    Gas,
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

    // vars for new impl:
    pub phase: Phase,
    pub pos_guess: Vec2,
    pub force: Vec2,
    pub body: isize, // body (if any) this particle belongs to, for disabling collisions

    pub imass: f32, // inverse mass
    pub tmass: f32, // temporary height-scaled mass
    pub s_friction: f32, // coeffs of friction
    pub k_friction: f32, // coeffs of friction
    pub t: f32,
}

impl Particle {
    pub fn set_static(&mut self, is_static: bool) -> &mut Self {
        self.is_static = is_static;
        if is_static {
            self.set_mass(0.0);
        }
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
        self.set_mass_2(mass);
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

    pub fn add_force(&mut self, force: Vec2) -> &mut Self {
        let acceleration = force / self.mass;
        let new_vel = self.vel + acceleration;
        self.set_vel(new_vel);
        self
    }


    pub fn guess(&self, time_delta: f32) -> Vec2 {
        if self.imass == 0.0 {
            self.pos
        } else {
            self.pos + time_delta * self.vel
        }
    }

    pub fn confirm_guess(&mut self) {
        let delta = self.pos_guess - self.pos;
        let len = delta.magnitude(); // todo: replace with magnitude2
        if len < f32::EPSILON {
            self.vel = Vec2::new(0.0 ,0.0); 
            return;
        }
        self.pos = self.pos_guess;
    }

    pub fn scale_mass(&mut self) {
        if self.imass != 0.0 {
            self.tmass = 1. / ((1. / self.imass) * -self.pos.y.exp());
        } else {
            self.tmass = 0.0;
        }
    }

    // Version of set_mass needed for unified physics paper
    pub fn set_mass_2(&mut self, mass: f32) -> &mut Self {
        if mass <= 0.0 {
            self.imass = -mass;
        } else {
            self.imass = 1.0 / mass;
        }
        self.tmass = self.imass;
        self
    }

    pub fn set_phase(&mut self, phase: Phase) -> &mut Self {
        self.phase = phase;
        self
    }

    pub fn get_sdf_data(&self, bodies: &Vec<Body>, idx: usize) -> SdfData {
        if self.phase != Phase::Solid || self.body < 0 {
            return SdfData::new(Vec2::new(0.0, 0.0), 0.0);
        }

        let body = &bodies[self.body as usize];
        let mut out = match body.sdf.get(&idx) {
            Some(value) => *value,
            None => SdfData::new(Vec2::new(0.0, 0.0), 0.0)
        };

        out.rotate(body.angle);
        return out;
    }

    pub fn get_p(&self, stable: bool) -> Vec2 { 
        if stable {
            self.pos
        } else {
            self.pos_guess
        }
    }
}

impl Default for Particle {
    fn default() -> Self {
        let mut s = Self {
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
            v_right_initial: Vec2::new(0.0, 0.0),

            phase: Phase::Solid,
            pos_guess: Vec2::new(0.0, 0.0),
            force: Vec2::new(0.0, 0.0),
            body: -1,

            imass: 0.0,
            tmass: 0.0,
            s_friction: 0.0,
            k_friction: 0.0,
            t: 4.0,
        };
        s.set_mass_2(s.mass);
        s
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