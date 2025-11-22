// use crate::{game::level::{level_builder::LevelBuilderContext, level_builder_operation::LevelBuilderOperation}, core::math::unit_conversions::cm_to_m, simulation::particles::{particle::Particle, shape_builder::shape_builder::ShapeBuilder}};


// pub struct JellyCube {
// }

// impl LevelBuilderOperation for JellyCube {
//     fn type_name(&self) -> &str {"JellyCube"}

//     fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
//         Box::new(JellyCube {})
//     }

//     fn default_spawn_chance(&self) -> f32 {
//         0.5
//     }

//     fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
//         let rng = &mut level_builder_context.rng;

//         let width = 0.0;
//         let height = 0.0; //rng.gen_range(-2.0..=-0.5);

//         let particle_radius = cm_to_m(4.0);
//         let particle_mass = 1.0; //g_to_kg(0.1);

//         let origin = level_builder_context.cursor;

//         // add a jellow cube to the scene
//         ShapeBuilder::new()
//             .set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_color(Color::from(LinearRgba::RED)).clone())
//             //.set_constraint_template(StickConstraint::default().set_stiffness_factor(1.0).clone())// this ignores mass
//             .apply_operation(RectangleStickGrid::from_rectangle(StickConstraint::default().set_stiffness_factor(1.0).clone(), 
//                 Rectangle::from_center_size(origin + vec2(0.0, 0.5), vec2(0.4, 0.8))))//                 //.add_stick_grid(2, 5, particle_radius * 2.2, Vec2::new(-3.0, cm_to_m(50.0)))
//             .create_in_particle_sim(level_builder_context.particle_sim);
//     }
// }
