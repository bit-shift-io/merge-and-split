
use crate::math::Vec2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,

    pub energy_delta: f32, // "the change in kinetic energy, ∆E, and store it as a potential energy in a virtual bond between the two colliding particles"

    //pub pos_prev: Vec2,
    pub radius: f32,
    pub mass: f32,

    pub merge_count: usize, // How many other particle are merged into this particle? 0 = No particles
    pub merge_index: usize, // Index of the particle this is currently merged with.
    // pub is_static: bool,
    // //pub color: Color,
    // pub is_enabled: bool,

    // pub force: Vec2, // should this be here? when we apply a force can we not just move the pos?
}

impl Particle {
    // pub fn new(pos: Vec2, vel: Vec2, radius: f32, mass: f32) -> Self {
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

//     pub fn set_static(&mut self, is_static: bool) -> &mut Self {
//         self.is_static = is_static;
//         self
//     }

//     pub fn set_color(&mut self, color: Color) -> &mut Self {
//         self.color = color;
//         self
//     }

//     pub fn get_aabb(&self) -> Aabb2d {
//         debug_assert!(!self.pos.x.is_nan());
//         debug_assert!(!self.pos.y.is_nan());
//         debug_assert!(!self.radius.is_nan());
//         debug_assert!(self.radius > 0.0);

//         let aabb = Aabb2d {
//             min: self.pos - vec2(self.radius, self.radius),
//             max: self.pos + vec2(self.radius, self.radius),
//         };
//         debug_assert!(aabb.min.x <= aabb.max.x && aabb.min.y <= aabb.max.y);
//         aabb
//     }

//     pub fn acceleration_to_force(acc: Vec2, mass: f32) -> Vec2 {
//         acc * mass
//     }

//     pub fn set_force_based_on_acceleration(&mut self, acceleration: Vec2) -> &mut Self {
//         self.force = acceleration * self.mass;
//         self
//     }

//     pub fn add_force(&mut self, force: Vec2) -> &mut Self {
//         self.force += force;
//         self
//     }

}

impl Default for Particle {
    fn default() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),

            energy_delta: 0.0,

            merge_count: 0,
            merge_index: 0,

            //pos_prev: cgmath::Vector2::new(0.0, 0.0),
            radius: 0.5,
            mass: 1.0,
            // is_static: false,
            // color: Color::WHITE,
            // is_enabled: true,
            // force: vec2(0.0, 0.0),
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