// Follow the bevy Aabb2d API: https://docs.rs/bevy_math/latest/bevy_math/bounding/struct.Aabb2d.html

use crate::core::math::vec2::Vec2;

pub struct Aabb2d {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb2d {
    pub fn from_point_cloud(points: &[Vec2]) -> Self {
        let mut points_iter = points.iter().map(|p| *p);
        
        let first = points_iter
            .next()
            .expect("point cloud must contain at least one point for Aabb2d construction");

        let (min, max) = points_iter.fold((first, first), |(prev_min, prev_max), point| {
            (Vec2::min(point, prev_min), Vec2::max(point, prev_max))
        });

        Self {
            min,
            max,
        }
    }
}