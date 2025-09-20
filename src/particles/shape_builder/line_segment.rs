//use bevy::math::Vec2;

use crate::math::vec2::Vec2;

use super::shape_builder::{radius_divisions_between_points, ShapeBuilder, ShapeBuilderOperation};


pub struct LineSegment {
    p1: Vec2,
    p2: Vec2,
}

impl LineSegment {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self { p1, p2 }
    }

    fn get_points_for_radius(&self, radius: f32) -> Vec::<Vec2> {
        let mut points = vec![];

        let divisions = radius_divisions_between_points(self.p1, self.p2, radius);
        let delta = self.p2 - self.p1;

        for i in 0..divisions { 
            let percent = i as f32 / divisions as f32;
            let pos = self.p1 + (delta * percent);
            points.push(pos);
        }

        points
    }
}

impl ShapeBuilderOperation for LineSegment {
    fn apply_to_shape_builder(&self, shape_builder: &mut ShapeBuilder) {
        let radius = shape_builder.particle_radius();
        let points = self.get_points_for_radius(radius);
        shape_builder.add_particles_from_points(&points);
    }
}