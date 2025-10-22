use crate::math::vec2::{rotate_vector_deg, Vec2};

// Signed distance field data for rigid-body collisions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SdfData {
    pub gradient: Vec2,
    pub distance: f32,
}

impl SdfData {
    pub fn new(gradient: Vec2, distance: f32) -> Self {
        Self {
            gradient,
            distance,
        }
    }

    pub fn rotate(&mut self, angle_deg: f32) {
        let rotated_vector = rotate_vector_deg(self.gradient, angle_deg);
        self.gradient = rotated_vector;
    }
}