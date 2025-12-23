use rand::Rng;
use crate::{
    core::math::{
        vec2::Vec2, 
        bezier_spline::{BezierSpline, CubicBezierCurve}
    }, 
    game::level::{
        level_builder::LevelBuilderContext, 
        level_builder_operation::LevelBuilderOperation
    }, 
    simulation::particles::
        shape_builder::shape_builder::ShapeBuilder
    
};

pub struct HillOperation;

impl LevelBuilderOperation for HillOperation {
    fn type_name(&self) -> &str { "HillOperation" }

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(HillOperation)
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let rng = &mut level_builder_context.rng;

        let total_width = rng.random_range(5.0..=15.0);
        let num_segments = rng.random_range(2..=4);
        let segment_width = total_width / num_segments as f32;
        let direction = level_builder_context.x_direction;

        let start_pos = level_builder_context.cursor;
        let mut current_pos = start_pos;
        let mut prev_cp2 = Vec2::zero(); // Placeholder

        let mut spline = BezierSpline::new();

        for i in 0..num_segments {
            let next_x = current_pos.x + segment_width * direction;
            // Randomize height, but keep it somewhat relative to start y to avoid huge cliffs
            // Maybe trend upwards or downwards or oscillate
            let height_variation = rng.random_range(-2.5..=2.5);
            let next_y = start_pos.y + height_variation; 
            
            // If it's the last segment, maybe bring it back to a standard level? 
            // Or just let it end wherever. Let's let it end wherever.
            
            let next_pos = Vec2::new(next_x, next_y);

            let cp1 = if i == 0 {
                // First segment: control point extends horizontally
                current_pos + Vec2::new(segment_width * 0.5 * direction, 0.0)
            } else {
                // Subsequent segments: reflect previous cp2 for C1 continuity
                // cp1 = current + (current - prev_cp2)
                // We might want to scale the magnitude to be appropriate for this segment
                let tangent = (current_pos - prev_cp2).normalize();
                current_pos + tangent * (segment_width * 0.5)
            };

            let cp2 = next_pos - Vec2::new(segment_width * 0.5 * direction, 0.0);

            let curve = CubicBezierCurve::new(current_pos, cp1, cp2, next_pos);
            spline.add_curve(curve);

            prev_cp2 = cp2;
            current_pos = next_pos;
        }

        // Generate particles
        let particle_radius = level_builder_context.particle_template.radius;

        // adding a gap to allow wheels to grip going up hill
        let gap_multiplier = 2.0;
        let step_size = particle_radius * 2.0 * gap_multiplier;
        
        // We want to fill the area under the curve.
        // Let's define a "floor" level. Maybe a fixed depth below the curve?
        // Or just fill down to a certain y?
        // Let's fill down to min(start_y, end_y) - 5.0 or something.
        // Or just make a thick terrain layer, e.g. 5 units thick.
        //let thickness = 1.0;

        let mut sb = ShapeBuilder::from_particle_template(level_builder_context.particle_template.clone());
        // Make it static (ground)
        sb.particle_template.set_static(true);
        // Set color to something earthy? Or just keep default/random?
        // Let's set a color if we can.
        // sb.particle_template.color = Vec4::new(0.4, 0.3, 0.1, 1.0); // Brownish

        // Sample the spline
        // We need enough samples to cover the x range with particles
        let num_x_steps = (total_width / step_size).ceil() as usize;
        
        for i in 0..=num_x_steps {
            let t = i as f32 / num_x_steps as f32;
            let surface_point = spline.sample(t);
            
            // Fill column downwards
            let num_y_steps = 1; //(thickness / step_size).ceil() as usize;
            for j in 0..num_y_steps {
                let y_offset = j as f32 * step_size;
                let pos = surface_point - Vec2::new(0.0, y_offset);
                sb.add_particle_at_position(pos);
            }
        }

        sb.create_in_simulation(level_builder_context.sim);

        // Update cursor
        level_builder_context.cursor = current_pos;
    }
}
