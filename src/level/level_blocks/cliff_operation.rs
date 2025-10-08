use rand::Rng;

use crate::{level::{level_builder::LevelBuilderContext, level_builder_operation::LevelBuilderOperation}, math::vec2::Vec2, particles::shape_builder::{line_segment::LineSegment, shape_builder::ShapeBuilder}};


pub struct CliffOperation {
}

impl LevelBuilderOperation for CliffOperation {
    fn type_name(&self) -> &str {"CliffOperation"}

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(CliffOperation {})
    }

    fn default_spawn_chance(&self) -> f32 {
        0.5
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let rng = &mut level_builder_context.rng;

        let width = 0.0;
        let height = rng.random_range(-2.0..=-0.5);

        let cursor_start = level_builder_context.cursor;
        let cursor_end = cursor_start + Vec2::new(width * level_builder_context.x_direction, height);

        let mut sb = ShapeBuilder::new();
        sb.set_particle_template(level_builder_context.particle_template)
            .apply_operation(LineSegment::new(level_builder_context.cursor, cursor_end)) 
            .apply_operation(LineSegment::new(cursor_end, cursor_end))
            .create_in_particle_vec(level_builder_context.particle_vec);

        level_builder_context.cursor = cursor_end;
    }
}
