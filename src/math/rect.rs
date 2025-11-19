// Use bevy math rect api: https://docs.rs/bevy/latest/bevy/math/prelude/struct.Rect.html

use crate::math::vec2::Vec2;

#[derive(Clone)]
#[repr(C)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}


impl Rect {
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    pub fn from_corners(p0: Vec2, p1: Vec2) -> Self {
        let min = Vec2::new(p0.x.min(p1.x), p0.y.min(p1.y));
        let max = Vec2::new(p0.x.max(p1.x), p0.y.max(p1.y));
        Self { min, max }
    }

    pub fn width(&self) -> f32 {
        (self.max.x - self.min.x).abs()
    }

    pub fn height(&self) -> f32 {
        (self.max.y - self.min.y).abs()
    }
}