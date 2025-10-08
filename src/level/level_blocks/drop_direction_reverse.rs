use rand::Rng;

use crate::{level::{level_builder::LevelBuilderContext, level_builder_operation::LevelBuilderOperation}, math::vec2::Vec2, particles::shape_builder::{line_segment::LineSegment, shape_builder::ShapeBuilder}};


pub struct DropDirectionReverse {
}

impl LevelBuilderOperation for DropDirectionReverse {
    fn type_name(&self) -> &str {"DropDirectionReverse"}

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(DropDirectionReverse {})
    }

    fn default_spawn_chance(&self) -> f32 {
        0.3
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let rng = &mut level_builder_context.rng;

        let width = 3.0;
        let height = rng.random_range(-2.5..=-2.0);

        let cursor_start = level_builder_context.cursor;
        let cursor_end = cursor_start + Vec2::new(level_builder_context.x_direction * width, height);

        // todo: might need a downward slope to ease us down lower
        let mut sb = ShapeBuilder::new();
        sb.set_particle_template(level_builder_context.particle_template) 
            .apply_operation(LineSegment::new(cursor_start, cursor_start + Vec2::new(1.5 * level_builder_context.x_direction, -1.5))) // steep downward slope
            .apply_operation(LineSegment::new(cursor_end, cursor_end + Vec2::new(0.0, -height + 2.0))) // a wall to stop the user escaping the map
            .create_in_particle_vec(level_builder_context.particle_vec);

        level_builder_context.cursor = cursor_end;
        level_builder_context.x_direction = -level_builder_context.x_direction;
        level_builder_context.x_direction_changed = true; // this helps protect us from spawning certain operations that take up lots of vertical space - FluidFunnel!
    }
}
