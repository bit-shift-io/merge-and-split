//use bevy::math::{vec2, Rect, Vec2};

use crate::math::{rect::Rect, vec2::Vec2};

use super::shape_builder::{radius_divisions_between_points, ShapeBuilder, ShapeBuilderOperation};

#[derive(Clone)]
pub struct Rectangle {
    pub rect: Rect
}

impl Rectangle {
    pub fn from_center_size(centre: Vec2, size: Vec2) -> Self {
        Self { rect: Rect::from_center_size(centre, size) }
    }

    pub fn from_corners(p0: Vec2, p1: Vec2) -> Self {
        Self { rect: Rect::from_corners(p0, p1) }
    }

    pub fn get_divisions_and_deltas_for_radius(&self, radius: f32) -> (usize, usize, Vec2, Vec2) {
        let min = self.rect.min;
        let x_max = self.rect.min + Vec2::new(self.rect.width(), 0.0);
        let y_max = self.rect.min + Vec2::new(0.0, self.rect.height());

        //println!("min, x_max, y_max: {}, {}, {}", min, x_max, y_max);
        //println!("radius {}", radius);

        let x_divisions = radius_divisions_between_points(min, x_max, radius);
        let y_divisions = radius_divisions_between_points(min, y_max, radius);

        let x_delta = x_max - min;
        let y_delta = y_max - min;

        (x_divisions, y_divisions, x_delta, y_delta)
    }

    fn get_points_for_radius(&self, radius: f32) -> Vec::<Vec2> {
        let mut points = vec![];

        let min = self.rect.min;
        let (x_divisions, y_divisions, x_delta, y_delta) = self.get_divisions_and_deltas_for_radius(radius);

        for yi in 0..y_divisions { 
            let y_percent = yi as f32 / y_divisions as f32;
            let y_offset = y_delta * y_percent;

            for xi in 0..x_divisions { 
                let x_percent = xi as f32 / x_divisions as f32;
                let x_offset = x_delta * x_percent;

                let pos = min + y_offset + x_offset;
                points.push(pos);
            }
        }

        points
    }
}

impl ShapeBuilderOperation for Rectangle {
    fn apply_to_shape_builder(&self, shape_builder: &mut ShapeBuilder) {
        let radius = shape_builder.particle_radius();
        let points = self.get_points_for_radius(radius);
        shape_builder.add_particles_from_points(&points);
    }
}