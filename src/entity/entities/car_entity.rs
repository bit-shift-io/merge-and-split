use crate::{entity::entity::{Entity, UpdateContext}, math::vec2::Vec2, particles::particle_vec::ParticleVec};


pub struct CarWheel {
    // hub_particle_handle: ParticleHandle,
    // surface_particle_handles: Vec<ParticleHandle>,
}

impl CarWheel {
    pub fn new(origin: Vec2, particle_vec: &mut ParticleVec) -> Self {
        // let particle_mass = 1.0; //g_to_kg(10.0);

        // // wheel hub - this is on mask layer zero which is a special no collisions layer
        // let hub_particle_handle = {
        //     let mask = 0x0;
        //     let particle_radius = cm_to_m(4.0);
        //     let mut builder = ShapeBuilder::new();
        //     builder.set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_color(Color::from(LinearRgba::GREEN)).clone());

        //     builder.add_particle(builder.create_particle().set_position(origin).clone())
        //         .create_in_particle_vec(particle_vec);

        //     builder.particle_handles.first().unwrap().clone()
        // };

        // // wheel surface
        // let surface_particle_handles = {
        //     let mask = 0x1;
        //     let divisions = 20;
        //     let circle_radius = cm_to_m(35.0); // around a typical car tyre size - 17-18" (once you account for particle radius)
        //     let particle_radius = cm_to_m(4.0);
        //     let mut builder = ShapeBuilder::new();
        //     builder.set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_color(Color::from(LinearRgba::GREEN)).clone());

        //     builder.apply_operation(Circle::new(origin, circle_radius));
        //     builder.apply_operation(AdjacentSticks::new(StickConstraint::default().clone(), 1, true)); // connect adjacent points
        //     builder.apply_operation(AdjacentSticks::new(StickConstraint::default().clone(), 6, true)); // connect every n points for extra stability during collisions

        //     builder.create_in_particle_vec(particle_vec);

        //     builder.particle_handles.clone()
        // };

        // /*
        // // wheel interior
        // let interior_particle_handles = {
        //     let mask = 0x1;
        //     let divisions = 14;
        //     let circle_radius = cm_to_m(14.0);
        //     let particle_radius = cm_to_m(4.0);
        //     let mut builder = ShapeBuilder::new();
        //     builder.set_mass(particle_mass);
        //     builder.add_circle(origin, circle_radius, particle_radius, divisions)
        //         //.connect_with_stick_chain(2) // stop the air escaping so easily
        //         .create_in_particle_vec(particle_vec, mask);
        //     builder.particle_handles.clone()
            
        //     //vec![]
        // };       */


        // // notes:
        // // the wheel hub needs a constraint to set its position to the centre of the wheel
        // // that is its position should be determined by a few points on the surface wheel.
        // // that said, this might cause issues with the air inside the wheel (YES, this is happening!). If this is the case
        // // we need a way to disable collisions for the hub (set radius to 0 - no we need to disable collision for the hub with the air - could use collision masks?). Set its layer to zero to mean the no collisions layer?
        // // or add a flag to particles to say they are "invisible"?

         
        //  /* todo: port to v4
        // // to optimise this we really only need maybe 4 points to determine the centre of the wheel for the incoming particles
        // // we set all particles as output particles so the axle can be pushed by any sticks
        // let mut weighted_particles = vec![];
        // for particle_handle in surface_particle_handles.iter() {
        //     weighted_particles.push(WeightedParticle::new(particle_handle.clone(), 1.0));
        // }

        // // todo: reenable outgoing_particles
        // particle_vec.create_attachment_constraint(weighted_particles.clone(), weighted_particles.clone(), hub_particle_handle.clone());
        // */

        // // instead of the above, which moved the huib to the centre, lets try just connecting the hub to the rim
        // // then we can just change the stiffness!
        // {
        //     let mut constraint_container = particle_vec.constraint_container.as_ref().write().unwrap();
            
        //     for (idx, surface_particle_handle) in surface_particle_handles.iter().enumerate() {
        //         let length = (particle_vec.get_particle(hub_particle_handle).pos - particle_vec.get_particle(surface_particle_handle.clone()).pos).length(); 
            
        //         constraint_container.add(StickConstraint::default()
        //             .set_stiffness_factor(20.0)
        //             .set_length(length)
        //             .set_particle_handles([hub_particle_handle, surface_particle_handle.clone()]).box_clone()
        //         );
        //     }
        // }

        // Self {
        //     hub_particle_handle,
        //     surface_particle_handles,
        //     //interior_particle_handles
        // }

        Self {}
    }

    fn rotate(&mut self, direction: f32, particle_vec: &mut ParticleVec) {
        
        // let hub_particle = particle_vec.get_particle(self.hub_particle_handle);
        // let centre = hub_particle.pos;
        // let force_magnitude = 60.0;

        // let particle_manipulator = ParticleManipulator::new();

        // // todo: instead of rotating the points to the wheel tangent. Try moving points towards the next point in the wheel (or where it would be in a perfect wheel).
        // // this will stop the wheels expanding outwards as you accelerate
        // particle_manipulator.add_rotational_force_around_point(particle_vec, &self.surface_particle_handles, centre, force_magnitude * direction);
        // //particle_manipulator.add_rotational_force_around_point(particle_vec, &self.interior_particle_handles, centre, force_magnitude * direction);
    }
}

const NUM_WHEELS: usize = 2;

pub struct CarEntity {
    pub wheels: [CarWheel; NUM_WHEELS],
}

impl CarEntity {
    pub fn new(particle_vec: &mut ParticleVec, origin: Vec2) -> Self {
        let wheel_spacing = 1.0 * 0.5; // metres

        let wheel_1 = CarWheel::new(origin + Vec2::new(wheel_spacing, 0.0), particle_vec);
        let wheel_2 = CarWheel::new(origin - Vec2::new(wheel_spacing, 0.0), particle_vec);

        // // axle stick to connect the two wheel hubs
        // {
        //     let length = (particle_vec.get_particle(wheel_1.hub_particle_handle).pos - particle_vec.get_particle(wheel_2.hub_particle_handle).pos).length(); 
        //     //particle_vec.create_stick([&wheel_1.hub_particle_handle, &wheel_2.hub_particle_handle], length, 0.0);

        //     let mut constraint_container = particle_vec.constraint_container.as_ref().write().unwrap();
        //     constraint_container.add(StickConstraint::default()
        //         .set_length(length)
        //         .set_particle_handles([wheel_1.hub_particle_handle, wheel_2.hub_particle_handle]).box_clone()
        //     );
        // }

        Self {
            wheels: [wheel_1, wheel_2],
        }
    }

    // fn rotate_wheels(&mut self, direction: f32, particle_vec: &mut ParticleVec) {
    //     for wheel in self.wheels.iter_mut() { 
    //         wheel.rotate(direction, particle_vec);
    //     }
    // }

    // pub fn update(&mut self, particle_vec: &mut ParticleVec, keys: Res<ButtonInput<KeyCode>>) {
    //     if keys.pressed(KeyCode::KeyZ) {
    //         self.rotate_wheels(1.0, particle_vec); // ccw
    //     }
    //     if keys.pressed(KeyCode::KeyX) {
    //         self.rotate_wheels(-1.0, particle_vec); // clockwise
    //     }
    // }

    // pub fn get_camera_look_at_position(&self, particle_vec: &ParticleVec) -> Vec2 {
    //     let mut pos = Vec2::new(0.0, 0.0);
        
    //     for wheel in self.wheels.iter() {
    //         pos += particle_vec.get_particle(wheel.hub_particle_handle).pos;
    //     }
    //     pos /= NUM_WHEELS as f32;
    //     //pos.extend(1.0); // homogeneous coordinate
        
    //     pos
    // }
}

impl Entity for CarEntity {
    fn update(&mut self, context: &mut UpdateContext) {
        // todo:
    }
}