use rand::Rng;

use crate::{constraints2::distance_constraint::DistanceConstraint, level::{level_builder::LevelBuilderContext, level_builder_operation::LevelBuilderOperation}, math::{vec2::Vec2, vec4::Vec4}, particles::shape_builder::{rectangle::Rectangle, rectangle_stick_grid::RectangleStickGrid, shape_builder::ShapeBuilder}};


pub struct SaggyBridgeOperation {
}

impl LevelBuilderOperation for SaggyBridgeOperation {
    fn type_name(&self) -> &str {"SaggyBridgeOperation"}

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(SaggyBridgeOperation {})
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let rng = &mut level_builder_context.rng;

        let width = rng.random_range(2.0..=5.0);
        let height = 0.0;

        let rect_height = level_builder_context.particle_template.radius * 4.0;

        let cursor_start = level_builder_context.cursor;
        let cursor_end = cursor_start + Vec2::new(width * level_builder_context.x_direction, height);

        let offset = Vec2::new(0.0, level_builder_context.particle_template.radius * 2.0); // lazy way to fix this!
        let rectangle = Rectangle::from_corners(cursor_start + offset, cursor_end + Vec2::new(0.0, -rect_height) + offset);
     
        let particle_vec_start_index = level_builder_context.sim.particles.len();

        let red = Vec4::new(1.0, 0.0,0.0, 1.0);

        let mut sb = ShapeBuilder::new();
        sb.set_particle_template(level_builder_context.particle_template.clone().set_colour(red).set_mass(1.0).set_static(false).clone());
        sb.apply_operation(rectangle.clone());
        
        // set left and right most particles and make them static
        // todo: make this a shape operation?
        let aabb = sb.get_aabb();
        sb.particles.iter_mut().for_each(|particle| {
            if particle.pos.x == aabb.min.x {
                particle.set_mass(0.0);
            }
            if particle.pos.x == aabb.max.x {
                particle.set_mass(0.0);
            }
        });

        sb.create_in_simulation(level_builder_context.sim);
        
        RectangleStickGrid::from_rectangle(rectangle)
            .compute_particle_pairs(sb.particle_radius(), particle_vec_start_index)
            .iter()
            .for_each(|particle_handles| {
                level_builder_context.sim.global_standard_distance_constraints.push(
                    DistanceConstraint::from_particles(particle_handles[0], particle_handles[1], &level_builder_context.sim.particles)
                );
            });

        level_builder_context.cursor = cursor_end;
    }
}
