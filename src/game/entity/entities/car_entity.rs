use winit::keyboard::KeyCode;

use crate::{core::math::{unit_conversions::cm_to_m, vec2::Vec2, vec4::Vec4}, game::entity::entity::UpdateContext, simulation::{constraints::distance_constraint::DistanceConstraint, particles::{particle::Particle, particle_manipulator::ParticleManipulator, particle_vec::{ParticleHandle, ParticleVec}, shape_builder::{adjacent_sticks::AdjacentSticks, circle::Circle, shape_builder::ShapeBuilder}, simulation::Simulation}}};

pub struct CarWheel {
    hub_particle_handle: ParticleHandle,
    surface_particle_handles: Vec<ParticleHandle>,
    //stick_vec: StickVec,
}

impl CarWheel {
    pub fn new(origin: Vec2, particle_vec: &mut ParticleVec, sim: &mut Simulation) -> Self {
        let particle_mass = 1.0; //g_to_kg(10.0);
        let green = Vec4::new(0.0, 1.0, 0.0, 1.0);
        //let mut stick_vec = StickVec::new();

        // wheel hub - this is on mask layer zero which is a special no collisions layer
        let hub_particle_handle = {
            let mask = 0x0;
            let particle_radius = cm_to_m(6.0);
            let mut builder = ShapeBuilder::new();
            builder.set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_colour(green).clone());

            builder.add_particle(builder.create_particle().set_pos(origin).clone())
                .create_in_simulation(sim);//.create_in_particle_vec(particle_vec);

            builder.particle_handles.first().unwrap().clone()
        };

        // wheel surface
        let surface_particle_handles = {
            let mask = 0x1;
            let divisions = 20;
            let circle_radius = cm_to_m(35.0); // around a typical car tyre size - 17-18" (once you account for particle radius)
            let particle_radius = cm_to_m(8.0);
            let mut builder = ShapeBuilder::new();
            builder.set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_colour(green).clone());

            builder.apply_operation(Circle::new(origin, circle_radius));


            builder.create_in_simulation(sim); //.create_in_particle_vec(particle_vec); // cause particle_handles to be populated in the shape builder

            AdjacentSticks::new(/*Stick::default().set_stiffness_factor(1.0).clone(),*/ 1, true)
                .apply_to_particle_handles(sim, &builder.particle_handles); // connect adjacent points
            //AdjacentSticks::new(Stick::default().set_stiffness_factor(0.5).clone(), 6, true).apply_to_particle_handles(particle_vec, &builder.particle_handles, &mut stick_vec); // connect every n points for extra stability during collisions
            

            builder.particle_handles.clone()
        };

        /*
        // wheel interior
        let interior_particle_handles = {
            let mask = 0x1;
            let divisions = 14;
            let circle_radius = cm_to_m(14.0);
            let particle_radius = cm_to_m(4.0);
            let mut builder = ShapeBuilder::new();
            builder.set_mass(particle_mass);
            builder.add_circle(origin, circle_radius, particle_radius, divisions)
                //.connect_with_stick_chain(2) // stop the air escaping so easily
                .create_in_particle_vec(particle_vec, mask);
            builder.particle_handles.clone()
            
            //vec![]
        };       */


        // notes:
        // the wheel hub needs a constraint to set its position to the centre of the wheel
        // that is its position should be determined by a few points on the surface wheel.
        // that said, this might cause issues with the air inside the wheel (YES, this is happening!). If this is the case
        // we need a way to disable collisions for the hub (set radius to 0 - no we need to disable collision for the hub with the air - could use collision masks?). Set its layer to zero to mean the no collisions layer?
        // or add a flag to particles to say they are "invisible"?

         
         /* todo: port to v4
        // to optimise this we really only need maybe 4 points to determine the centre of the wheel for the incoming particles
        // we set all particles as output particles so the axle can be pushed by any sticks
        let mut weighted_particles = vec![];
        for particle_handle in surface_particle_handles.iter() {
            weighted_particles.push(WeightedParticle::new(particle_handle.clone(), 1.0));
        }

        // todo: reenable outgoing_particles
        particle_vec.create_attachment_constraint(weighted_particles.clone(), weighted_particles.clone(), hub_particle_handle.clone());
        */

        // instead of the above, which moved the huib to the centre, lets try just connecting the hub to the rim
        // then we can just change the stiffness!
        
        {
            //let mut constraint_container = particle_vec.constraint_container.as_ref().write().unwrap();

            for (idx, surface_particle_handle) in surface_particle_handles.iter().enumerate() {
                let dist = (sim.particles[hub_particle_handle].pos - sim.particles[*surface_particle_handle].pos).magnitude(); 
            
                sim.add_distance_constraint(DistanceConstraint::new(dist, hub_particle_handle, *surface_particle_handle, false));

                // stick_vec.push(*Stick::default()
                //     .set_stiffness_factor(0.5)
                //     .set_length(length)
                //     .set_particle_handles([hub_particle_handle, surface_particle_handle.clone()])
                // );
            }
        }

        Self {
            hub_particle_handle,
            surface_particle_handles,
            //stick_vec,
            //interior_particle_handles
        }
    }

    fn rotate(&mut self, direction: f32, particle_vec: &mut ParticleVec) {
        
        let hub_particle = particle_vec[self.hub_particle_handle];
        let centre = hub_particle.pos;
        let torque = 0.01; // Nm

        let particle_manipulator = ParticleManipulator::new();

        // todo: instead of rotating the points to the wheel tangent. Try moving points towards the next point in the wheel (or where it would be in a perfect wheel).
        // this will stop the wheels expanding outwards as you accelerate
        particle_manipulator.add_torque_around_point(particle_vec, &self.surface_particle_handles, centre, torque * direction);
        //particle_manipulator.add_torque_around_point(particle_vec, &self.interior_particle_handles, centre, torque * direction);
    }

    fn update(&mut self, context: &mut UpdateContext) {
        //self.stick_vec.execute(&mut context.sim.particles, context.time_delta);
    }
}

const NUM_WHEELS: usize = 2;

pub struct CarEntity {
    pub wheels: [CarWheel; NUM_WHEELS],
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CarEntity {
    pub fn new(particle_vec: &mut ParticleVec, sim: &mut Simulation, origin: Vec2) -> Self {
        let wheel_spacing = 1.0 * 0.5; // metres

        let wheel_1 = CarWheel::new(origin + Vec2::new(wheel_spacing, 0.0), particle_vec, sim);
        let wheel_2 = CarWheel::new(origin - Vec2::new(wheel_spacing, 0.0), particle_vec, sim);

        // axle stick to connect the two wheel hubs
        {
            let dist = (sim.particles[wheel_1.hub_particle_handle].pos - sim.particles[wheel_2.hub_particle_handle].pos).magnitude(); 
            //particle_vec.create_stick([&wheel_1.hub_particle_handle, &wheel_2.hub_particle_handle], length, 0.0);

            // let mut constraint_container = particle_vec.constraint_container.as_ref().write().unwrap();
            // constraint_container.add(StickConstraint::default()
            //     .set_length(length)
            //     .set_particle_handles([wheel_1.hub_particle_handle, wheel_2.hub_particle_handle]).box_clone()
            // );

            sim.add_distance_constraint(DistanceConstraint::new(dist, wheel_1.hub_particle_handle, wheel_2.hub_particle_handle, false));
        }

        Self {
            wheels: [wheel_1, wheel_2],
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    fn rotate_wheels(&mut self, direction: f32, particle_vec: &mut ParticleVec) {
        for wheel in self.wheels.iter_mut() { 
            wheel.rotate(direction, particle_vec);
        }
    }

    // pub fn update(&mut self, particle_vec: &mut ParticleVec, keys: Res<ButtonInput<KeyCode>>) {
    //     if keys.pressed(KeyCode::KeyZ) {
    //         self.rotate_wheels(1.0, particle_vec); // ccw
    //     }
    //     if keys.pressed(KeyCode::KeyX) {
    //         self.rotate_wheels(-1.0, particle_vec); // clockwise
    //     }
    // }

    pub fn get_camera_look_at_position(&self, particle_vec: &ParticleVec) -> Vec2 {
        let mut pos = Vec2::new(0.0, 0.0);
        
        for wheel in self.wheels.iter() {
            pos += particle_vec[wheel.hub_particle_handle].pos;
        }
        pos /= NUM_WHEELS as f32;
        //pos.extend(1.0); // homogeneous coordinate
        
        pos
    }

    fn update(&mut self, context: &mut UpdateContext) {
        // Update wheel contraints
        for wheel in self.wheels.iter_mut() {
            wheel.update(context);
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
    }

    fn handle_key(&mut self, key: KeyCode, is_pressed: bool) -> bool {
        match key {
            KeyCode::KeyZ => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCode::KeyX => {
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

    pub fn update(&mut self, context: &mut crate::game::entity::entity::UpdateContext) {
        for e in self.0.iter_mut() {
            e.update(context);
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, is_pressed: bool) {
        for e in self.0.iter_mut() {
            e.handle_key(key, is_pressed);
        }
    }
}
