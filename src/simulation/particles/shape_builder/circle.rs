
use crate::core::math::vec2::Vec2;

use super::shape_builder::{ShapeBuilder, ShapeBuilderOperation};

// The method to distribute any left over space.
pub enum SpaceDistribution {
    AdjustRadius, // Adjust the particles radius so that there is no left over space.
    SpaceBetweenParticles, // Spread out any left over space so all particles are evenly spaced out.
}

pub struct Circle {
    centre: Vec2,
    radius: f32,
    space_distribution: SpaceDistribution,
}

impl Circle {
    pub fn new(centre: Vec2, radius: f32, space_distribution: SpaceDistribution) -> Self {
        Self { centre, radius, space_distribution }
    }
}

impl ShapeBuilderOperation for Circle {
    fn apply_to_shape_builder(&self, shape_builder: &mut ShapeBuilder) {
        let original_radius = shape_builder.particle_radius();
        let mut radius = original_radius;

        // putting a smaller circle on the bigger circle, creates 2x isosceles triangles where they intersect
        // so solve to find the half angle
        // https://www.quora.com/How-do-you-find-the-angles-of-an-isosceles-triangle-given-three-sides
        let top = radius * radius; // particle radius ^ 2
        let bottom = 2.0 * self.radius * self.radius; // circle_radius ^ 2
        
        // Ensure we don't take acos of a value outside [-1, 1]
        // 1.0 - (top / bottom) should be >= -1.0 => top/bottom <= 2.0 => radius^2 <= 4 * circle_radius^2 => radius <= 2 * circle_radius
        let val = 1.0 - (top / bottom);
        let clamped_val = val.clamp(-1.0, 1.0);

        let c_angle = f32::acos(clamped_val); // this is the half angle made by the isosceles trangle from the 2 intersecting circles
        let mut theta = c_angle * 2.0;
        
        let divisions = (2.0 * std::f32::consts::PI) / theta;
        let mut count = divisions;

        match self.space_distribution {
            SpaceDistribution::AdjustRadius => {
                count = divisions.round();
                if count > 0.0 {
                    theta = (2.0 * std::f32::consts::PI) / count;
                    
                    // Recalculate radius to fit exactly
                    // theta = 2 * c_angle => c_angle = theta / 2.0
                    // cos(c_angle) = 1.0 - (r^2 / (2 R^2))
                    // r^2 = 2 R^2 * (1.0 - cos(c_angle))
                    let new_c_angle = theta / 2.0;
                    let r_squared = 2.0 * self.radius * self.radius * (1.0 - new_c_angle.cos());
                    radius = r_squared.sqrt();

                    shape_builder.particle_template.radius = radius;
                }
            },
            SpaceDistribution::SpaceBetweenParticles => {
                count = divisions.floor();
                if count > 0.0 {
                    // Increase theta to spread particles evenly
                    theta = (2.0 * std::f32::consts::PI) / count;
                }
            }
        }

        let integer_divisions = count as usize;
        let mut points = Vec::with_capacity(integer_divisions);

        for i in 0..integer_divisions {
            let radians = i as f32 * theta;
            let x = f32::sin(radians);
            let y = f32::cos(radians);
            let pos = self.centre + Vec2::new(x * self.radius, y * self.radius);
            points.push(pos);
        }

        shape_builder.add_particles_from_points(&points);

        // Restore original radius if we changed it
        if (radius - original_radius).abs() > f32::EPSILON {
            shape_builder.particle_template.radius = original_radius;
        }
    }
}