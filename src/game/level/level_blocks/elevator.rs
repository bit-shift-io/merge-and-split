use rand::Rng;

use crate::{core::math::{vec2::Vec2, vec4::Vec4}, game::{entity::entity_system::UpdateContext, level::{level_builder::LevelBuilderContext, level_builder_operation::LevelBuilderOperation}}, simulation::particles::shape_builder::{line_segment::LineSegment, shape_builder::ShapeBuilder}};

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

        let width = 2.0;
        let height = rng.random_range(1.0..=4.0);

        let horizontal_movement = Vec2::new(width * level_builder_context.x_direction, 0.0);
        let vertical_movement = Vec2::new(0.0, height);

        let cursor_start = level_builder_context.cursor;
        let cursor_end = cursor_start + horizontal_movement + vertical_movement;

        let elevator_start = cursor_start + horizontal_movement * 0.5;
        let elevator_end = elevator_start + vertical_movement;

        let vertical_diameter_offset = Vec2::new(0.0, level_builder_context.particle_template.radius * 2.0);
        ShapeBuilder::from_particle_template(*level_builder_context.particle_template.clone().set_static(true))
            // Floor:
            .apply_operation(LineSegment::new(cursor_start - vertical_diameter_offset, cursor_start + horizontal_movement - vertical_diameter_offset)) 
            // Wall:
            .apply_operation(LineSegment::new(cursor_start + horizontal_movement, cursor_end))
            .create_in_simulation(level_builder_context.sim);


        // Moving platform - todo: make this a rigid body or soft body? it crushes the player forcing the game to end which is no fun.
        let mut platform = ShapeBuilder::from_particle_template(*level_builder_context.particle_template.clone().set_static(true).set_colour(Vec4::GREEN));
        platform.apply_operation(LineSegment::new(cursor_start, cursor_start + horizontal_movement))
            .create_in_simulation(level_builder_context.sim);

        let first_particle_offset = platform.particles[0].pos - elevator_start;

        level_builder_context.entity_system.elevator_entity_system.push(ElevatorEntity {
            start: elevator_start,
            end: elevator_end,
            speed: rng.random_range(1.0..=2.0),
            state: ElevatorState::MovingUp,
            pos: elevator_start,
            particle_indicies: platform.particle_handles,
            first_particle_offset,
            wait_time: 2.0,
            wait_timer: 0.0,
            particle_radius: level_builder_context.particle_template.radius,
        });

        level_builder_context.cursor = cursor_end;
    }
}

enum ElevatorState {
    MovingUp,
    AtTopWaiting,
    MovingDown,
    AtBottomWaiting,
}

pub struct ElevatorEntity {
    start: Vec2,
    end: Vec2,
    speed: f32,
    state: ElevatorState,
    pos: Vec2,
    particle_indicies: Vec<usize>,
    first_particle_offset: Vec2,
    wait_time: f32,
    wait_timer: f32,
    particle_radius: f32,
}

pub struct ElevatorEntitySystem(pub Vec<ElevatorEntity>);

impl ElevatorEntitySystem {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, c: ElevatorEntity) {
        self.0.push(c);
    }

    pub fn update(&mut self, context: &mut UpdateContext) {
        for e in self.0.iter_mut() {
            // todo: support horizontal movement too
            match e.state {
                ElevatorState::MovingUp => {
                    if e.pos.y >= e.end.y {
                        e.state = ElevatorState::AtTopWaiting;
                        e.wait_timer = e.wait_time;
                        e.pos = e.end;
                    } else {
                        e.pos += Vec2::new(0.0, e.speed) * context.time_delta;
                    }
                }

                ElevatorState::AtTopWaiting => {
                    e.wait_timer -= context.time_delta;
                    if e.wait_timer <= 0.0 {
                        e.state = ElevatorState::MovingDown;
                    }
                }

                ElevatorState::MovingDown => {
                    if e.pos.y <= e.start.y {
                        e.state = ElevatorState::AtBottomWaiting;
                        e.wait_timer = e.wait_time;
                        e.pos = e.start;
                    } else {
                        e.pos += Vec2::new(0.0, -e.speed) * context.time_delta;
                    }
                }
                
                ElevatorState::AtBottomWaiting => {
                    e.wait_timer -= context.time_delta;
                    if e.wait_timer <= 0.0 {
                        e.state = ElevatorState::MovingUp;
                    }
                },
            };
        }
    }

    pub fn update_counts(&mut self, sim: &mut crate::simulation::particles::simulation::Simulation) {
        // for e in self.0.iter_mut() {
        //     sim.bodies[e.body_idx].update_counts(&mut sim.counts, 1);
        // }

        for e in self.0.iter_mut() {
            for pi in e.particle_indicies.iter() {
                sim.counts[*pi] += 1;
            }
        }
    }

    pub fn solve_constraints(&mut self, sim: &mut crate::simulation::particles::simulation::Simulation, _time_delta: f32) {
        // for e in self.0.iter_mut() {
        //     let body_pos = sim.bodies[e.body_idx].center;
        //     let offset = e.pos - body_pos;

        //     sim.bodies[e.body_idx].for_each_particle(|particle_idx| {
        //         let particle = &mut sim.particles[particle_idx];
        //         particle.pos_guess += offset / sim.counts[particle_idx] as f32;
        //     });
        // }

        for e in self.0.iter_mut() {
            for (idx, pi) in e.particle_indicies.iter().enumerate() {
                let pos_guess = sim.particles[*pi].pos_guess;
                let where_particle_pos_guess_should_be = e.pos + e.first_particle_offset + Vec2::new(e.particle_radius * 2.0 * (idx as f32), 0.0);

                let offset = where_particle_pos_guess_should_be - pos_guess;

                sim.particles[*pi].pos_guess += offset / sim.counts[*pi] as f32;
            }
        }
    }
}
