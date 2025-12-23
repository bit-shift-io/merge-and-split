use crate::{core::math::{unit_conversions::cm_to_m, vec2::Vec2, vec4::Vec4}, engine::app::event_system::KeyCodeType, game::{entity::{entities::finish_entity::FinishEntitySystem, entity_system::UpdateContext}}, simulation::{constraints::{spring_constraint::SpringConstraint, volume_constraint::VolumeConstraint}, particles::{particle::Particle, particle_manipulator::ParticleManipulator, particle_vec::{ParticleHandle, ParticleVec}, shape_builder::{adjacent_sticks::AdjacentSticks, circle::{Circle, SpaceDistribution}, shape_builder::ShapeBuilder}, simulation::Simulation}}};

pub struct CarWheel {
    hub_particle_handle: ParticleHandle,
    surface_particle_handles: Vec<ParticleHandle>,
    spring_constraint_ids: Vec<usize>,
    volume_constraint_ids: Vec<usize>,
    surface_constraint_ids: Vec<usize>,
}

impl CarWheel {
    pub fn new(origin: Vec2, _particle_vec: &mut ParticleVec, sim: &mut Simulation) -> Self {
        let particle_mass = 1.0; //g_to_kg(10.0);

        // wheel hub - this is on mask layer zero which is a special no collisions layer
        let hub_particle_handle = {
            //let mask = 0x0;
            let particle_radius = cm_to_m(6.0);
            let mut builder = ShapeBuilder::from_particle_template(
                Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_colour(Vec4::GREEN).clone()
            );
            builder.add_particle(builder.create_particle().set_pos(origin).clone())
                .create_in_simulation(sim);

            builder.particle_handles.first().unwrap().clone()
        };

        // wheel surface
        let (surface_particle_handles, surface_constraint_ids) = {
            //let mask = 0x1;
            //let divisions = 20;
            let circle_radius = cm_to_m(35.0); // around a typical car tyre size - 17-18" (once you account for particle radius)
            let particle_radius = cm_to_m(8.0);
            
            let mut particle_template = Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_colour(Vec4::GREEN).clone();
            particle_template.k_friction = 0.9;
            particle_template.s_friction = 0.9;
            //particle_template.body = -1; // stop surface particles hitting each other!

            let mut builder = ShapeBuilder::from_particle_template(particle_template);
            builder.apply_operation(Circle::new(origin, circle_radius, SpaceDistribution::SpaceBetweenParticles));
            builder.create_in_simulation(sim);
            
            let ids = AdjacentSticks::new(/*Stick::default().set_stiffness_factor(1.0).clone(),*/ 1, true)
                .apply_to_particle_handles(sim, &builder.particle_handles); // connect adjacent points
            //AdjacentSticks::new(Stick::default().set_stiffness_factor(0.5).clone(), 6, true).apply_to_particle_handles(particle_vec, &builder.particle_handles, &mut stick_vec); // connect every n points for extra stability during collisions
            (builder.particle_handles.clone(), ids)
        };

        let mut spring_constraint_ids = vec![];
        let mut volume_constraint_ids = vec![];

        {
            // Connect sufrace of wheels together
            for (_idx, surface_particle_handle) in surface_particle_handles.iter().enumerate() {
                let dist = (sim.particles[hub_particle_handle].pos - sim.particles[*surface_particle_handle].pos).magnitude(); 
                //sim.add_distance_constraint(DistanceConstraint::new(dist, hub_particle_handle, *surface_particle_handle, false));
                let id = sim.add_spring_constraint(SpringConstraint::new(dist, 1000.0, hub_particle_handle, *surface_particle_handle, false));
                spring_constraint_ids.push(id);
            }

            // Add volume constraint
            let compliance = 0.00001; // Tunable parameter
            let id = sim.add_volume_constraint(VolumeConstraint::new(compliance, surface_particle_handles.clone(), &sim.particles));
            volume_constraint_ids.push(id);
        }

        Self {
            hub_particle_handle,
            surface_particle_handles,
            spring_constraint_ids,
            volume_constraint_ids,
            surface_constraint_ids,
        }
    }

    fn rotate(&mut self, direction: f32, particle_vec: &mut ParticleVec) {
        let hub_particle = particle_vec[self.hub_particle_handle];
        let centre = hub_particle.pos;
        let torque = 0.05; // Nm

        let particle_manipulator = ParticleManipulator::new();

        // todo: instead of rotating the points to the wheel tangent. Try moving points towards the next point in the wheel (or where it would be in a perfect wheel).
        // this will stop the wheels expanding outwards as you accelerate
        particle_manipulator.add_torque_around_point(particle_vec, &self.surface_particle_handles, centre, torque * direction);
    }

    fn disable_constraints(&mut self, sim: &mut Simulation) {
        for &id in &self.spring_constraint_ids {
            sim.spring_constraints.0[id].enabled = false;
        }
        for &id in &self.volume_constraint_ids {
            sim.volume_constraints.0[id].enabled = false;
        }
        for &id in &self.surface_constraint_ids {
            sim.distance_constraints.0[id].enabled = false;
        }
    }
}

const NUM_WHEELS: usize = 2;

pub struct CarEntity {
    pub wheels: [CarWheel; NUM_WHEELS],
    is_left_pressed: bool,
    is_right_pressed: bool,
    axle_constraint_id: usize,
    pub game_ended: bool,
}

impl CarEntity {
    pub fn new(particle_vec: &mut ParticleVec, sim: &mut Simulation, origin: Vec2) -> Self {
        // I kind of like it when the wheels can bump into each other a little occasionally, it adds to the challenge:
        // if you go too fast you risk getting bogged in your own wheels.
        let wheel_spacing = 1.2; // metres - 
        let half_wheel_spacing = wheel_spacing * 0.5; // metres

        let wheel_1 = CarWheel::new(origin + Vec2::new(half_wheel_spacing, 0.0), particle_vec, sim);
        let wheel_2 = CarWheel::new(origin - Vec2::new(half_wheel_spacing, 0.0), particle_vec, sim);

        // Axle constraint to connect the two wheel hubs
        let axle_constraint_id = {
            let dist = (sim.particles[wheel_1.hub_particle_handle].pos - sim.particles[wheel_2.hub_particle_handle].pos).magnitude(); 
            //sim.add_distance_constraint(DistanceConstraint::new(dist, wheel_1.hub_particle_handle, wheel_2.hub_particle_handle, false));
            sim.add_spring_constraint(SpringConstraint::new(dist, 2000.0, wheel_1.hub_particle_handle, wheel_2.hub_particle_handle, false))
        };

        Self {
            wheels: [wheel_1, wheel_2],
            is_left_pressed: false,
            is_right_pressed: false,
            axle_constraint_id,
            game_ended: false,
        }
    }

    fn rotate_wheels(&mut self, direction: f32, particle_vec: &mut ParticleVec) {
        for wheel in self.wheels.iter_mut() { 
            wheel.rotate(direction, particle_vec);
        }
    }

    pub fn get_camera_look_at_position(&self, particle_vec: &ParticleVec) -> Vec2 {
        let mut pos = Vec2::new(0.0, 0.0);
        
        for wheel in self.wheels.iter() {
            pos += particle_vec[wheel.hub_particle_handle].pos;
        }
        pos /= NUM_WHEELS as f32;
        //pos.extend(1.0); // homogeneous coordinate
        
        pos
    }

    fn update(&mut self, context: &mut UpdateContext, finish_entity_system: &FinishEntitySystem) {
        if self.game_ended {
            return;
        }

        // Apply input to wheels
        if self.is_left_pressed {
            self.rotate_wheels(1.0, &mut context.sim.particles); // ccw
        }
        if self.is_right_pressed {
            self.rotate_wheels(-1.0, &mut context.sim.particles); // clockwise
        }

        // Update the camera to follow the car
        let look_at_pos = self.get_camera_look_at_position(&mut context.sim.particles);
        context.camera.target = cgmath::Point3::new(look_at_pos.x, look_at_pos.y, 0.0);

        // Check for finish 
        // - todo: check against wheel hub centres so we only need to check 2 points instead of every wheel surface
        //      and/or check against the simulation spatial partition.
        for finish_entity in &finish_entity_system.entities {
            for wheel in &self.wheels {
                for particle_handle in &wheel.surface_particle_handles {
                    let particle = &context.sim.particles[*particle_handle];
                    if particle.pos.x >= finish_entity.aabb.min.x && particle.pos.x <= finish_entity.aabb.max.x &&
                       particle.pos.y >= finish_entity.aabb.min.y && particle.pos.y <= finish_entity.aabb.max.y {
                        self.game_ended = true;
                    }
                }
            }

            if self.game_ended {
                println!("Game Finished! Time: {:.2}s", context.total_time);

                // Break the car apart!
                context.sim.spring_constraints.0[self.axle_constraint_id].enabled = false;
                for wheel in self.wheels.iter_mut() {
                    wheel.disable_constraints(context.sim);
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyCodeType, is_pressed: bool) -> bool {
        match key {
            KeyCodeType::KeyZ => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCodeType::KeyX => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }
}



pub struct CarEntitySystem(pub Vec<CarEntity>);

impl CarEntitySystem {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, c: CarEntity) {
        self.0.push(c);
    }

    pub fn update(&mut self, context: &mut UpdateContext, finish_entity_system: &FinishEntitySystem) {
        for e in self.0.iter_mut() {
            e.update(context, finish_entity_system);
        }
    }

    pub fn handle_key(&mut self, key: KeyCodeType, is_pressed: bool) {
        for e in self.0.iter_mut() {
            e.handle_key(key, is_pressed);
        }
    }
}
