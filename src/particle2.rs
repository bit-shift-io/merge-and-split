

#[derive(Debug, Copy, Clone)]
pub struct Particle2 {
    pub pos: cgmath::Vector2<f32>,
    pub pos_prev: cgmath::Vector2<f32>,
    pub radius: f32,
    // pub mass: f32,
    // pub is_static: bool,
    // //pub color: Color,
    // pub is_enabled: bool,

    // pub force: cgmath::Vector2<f32>, // should this be here? when we apply a force can we not just move the pos?
}

// impl Particle {
//     pub fn new(pos: Vec2, radius: f32, mass: f32, is_static: bool, color: Color) -> Self {
//         debug_assert!(!pos.x.is_nan());
//         debug_assert!(!pos.y.is_nan());
        
//         Self { pos, pos_prev: pos, radius, mass, is_static, color, is_enabled: true, force: vec2(0.0, 0.0) }
//     }

//     pub fn set_radius(&mut self, radius: f32) -> &mut Self {
//         debug_assert!(!radius.is_nan());
//         debug_assert!(radius > 0.0);
//         self.radius = radius;
//         self
//     }

//     pub fn set_mass(&mut self, mass: f32) -> &mut Self {
//         debug_assert!(!mass.is_nan());
//         self.mass = mass;
//         self
//     }

//     pub fn set_position(&mut self, pos: Vec2) -> &mut Self {
//         debug_assert!(!pos.x.is_nan());
//         debug_assert!(!pos.y.is_nan());
//         self.pos = pos;
//         self.pos_prev = pos;
//         self
//     }

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

// }

impl Default for Particle2 {
    fn default() -> Self {
        Self {
            pos: cgmath::Vector2::new(0.0, 0.0),
            pos_prev: cgmath::Vector2::new(0.0, 0.0),
            radius: 0.5,
            // mass: 1.0,
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
        let p = Particle2::default();

        assert_eq!(p.pos, cgmath::Vector2::new(0.0, 0.0));
    }


}