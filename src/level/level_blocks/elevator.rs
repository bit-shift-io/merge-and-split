use cgmath::InnerSpace;
use rand::Rng;

use crate::{entity::entity::{Entity, EntityConstraintSolver}, level::{level_builder::LevelBuilderContext, level_builder_operation::LevelBuilderOperation}, math::vec2::Vec2, particles::{particle::Particle, particle_vec::ParticleVec, sdf_data::SdfData, shape_builder::{line_segment::LineSegment, shape_builder::ShapeBuilder}}};


pub struct ElevatorOperation {
}

impl LevelBuilderOperation for ElevatorOperation {
    fn type_name(&self) -> &str {"ElevatorOperation"}

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(ElevatorOperation {})
    }

    fn default_spawn_chance(&self) -> f32 {
        0.5
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let rng = &mut level_builder_context.rng;

        let width = 1.0;
        let height = rng.random_range(1.0..=4.0);

        let horizontal_movement = Vec2::new(width * level_builder_context.x_direction, 0.0);
        let vertical_movement = Vec2::new(0.0, height);

        let cursor_start = level_builder_context.cursor;
        let cursor_end = cursor_start + horizontal_movement + vertical_movement;

        let mut sb = ShapeBuilder::new();
        sb.set_particle_template(*level_builder_context.particle_template.set_static(true))
            .apply_operation(LineSegment::new(cursor_start, cursor_start + horizontal_movement)) 
            .apply_operation(LineSegment::new(cursor_start + horizontal_movement, cursor_end))
            .create_in_simulation(level_builder_context.sim);


        let particle_rad = sb.particle_radius();
        let particle_diam = particle_rad * 2.0;


        let root2 = f32::sqrt(2.0);
        let mut sdf_data = Vec::<SdfData>::new();
        sdf_data.push(SdfData::new(Vec2::new(-1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(-1.0, 1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(0.0, -1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(0.0, 1.0).normalize(), particle_rad));
        sdf_data.push(SdfData::new(Vec2::new(1.0, -1.0).normalize(), particle_rad * root2));
        sdf_data.push(SdfData::new(Vec2::new(1.0, 1.0).normalize(), particle_rad * root2));


        let offset = cursor_start;
        let x_max = 3;
        let y_max = 2;

        let mut particles = ParticleVec::new();
        for x in 0..x_max {
            let x_val = particle_diam * ((x % x_max) as f32 - x_max as f32 / 2.0);
            for y in 0..y_max {
                let y_val = (y_max + (y % y_max) + 1) as f32 * particle_diam;
                let mass = if x == 0 && y == 0 { 1.0 } else { 1.0 };
                let mut part = *Particle::default().set_radius(particle_rad).set_pos(Vec2::new(x_val, y_val) + offset).set_mass(mass);
                part.vel.x = 5.0;
                part.k_friction = 0.01;
                part.s_friction = 0.1;
                particles.push(part);
            }
        }

        let body_idx = level_builder_context.sim.create_rigid_body(&mut particles, &sdf_data);


        level_builder_context.entity_system.add_constraint_solver(ElevatorEntity {
            body_idx,
            start: cursor_start,
            end: cursor_start + vertical_movement,
            speed: rng.random_range(0.5..=1.5),
            state: ElevatorState::MovingUp,
        });

        level_builder_context.cursor = cursor_end;
    }
}

enum ElevatorState {
    MovingUp,
    MovingDown,
}

pub struct ElevatorEntity {
    body_idx: usize,
    start: Vec2,
    end: Vec2,
    speed: f32,
    state: ElevatorState,
}

// impl Entity for ElevatorEntity {
//     fn update(&mut self, context: &mut crate::entity::entity::UpdateContext) {
//         let pos = context.sim.bodies[self.body_idx].center;

//         // todo: support horizontal movement too
//         let movement = match self.state {
//             ElevatorState::MovingUp => {
//                 if pos.y >= self.end.y {
//                     self.state = ElevatorState::MovingDown;
//                     Vec2::new(0.0, -self.speed)
//                 } else {
//                     Vec2::new(0.0, self.speed)
//                 }
//             }
//             ElevatorState::MovingDown => {
//                 if pos.y <= self.start.y {
//                     self.state = ElevatorState::MovingUp;
//                     Vec2::new(0.0, self.speed)
//                 } else {
//                     Vec2::new(0.0, -self.speed)
//                 }
//             }
//         };

//         let velocity = movement * context.time_delta;

//         context.sim.bodies[self.body_idx].for_each_particle(|particle_idx| {
//             let particle = &mut context.sim.particles[particle_idx];
//             particle.vel = velocity;
//         });
//     }

//     fn handle_key(&mut self, key: winit::keyboard::KeyCode, is_pressed: bool) -> bool {
//         // No key handling for now
//         false
//     }
// }

impl EntityConstraintSolver for ElevatorEntity {
    fn solve_constraints(&mut self, sim: &mut crate::particles::simulation::Simulation, time_delta: f32) {
        let pos = sim.bodies[self.body_idx].center;

        // todo: support horizontal movement too
        let movement = match self.state {
            ElevatorState::MovingUp => {
                if pos.y >= self.end.y {
                    self.state = ElevatorState::MovingDown;
                    Vec2::new(0.0, -self.speed)
                } else {
                    Vec2::new(0.0, self.speed)
                }
            }
            ElevatorState::MovingDown => {
                if pos.y <= self.start.y {
                    self.state = ElevatorState::MovingUp;
                    Vec2::new(0.0, self.speed)
                } else {
                    Vec2::new(0.0, -self.speed)
                }
            }
        };

        let velocity = movement * time_delta;

        sim.bodies[self.body_idx].for_each_particle(|particle_idx| {
            let particle = &mut sim.particles[particle_idx];
            particle.pos_guess += velocity;
        });
    }
}