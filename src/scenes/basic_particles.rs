use std::{thread, time::Duration};

use cgmath::Rotation3;
use winit::{event::WindowEvent, keyboard::KeyCode};

use crate::{constraints::{fixed_point_spring::FixedPointSpringVec, stick::{Stick, StickVec}}, entity::{entities::{car_entity::CarEntity, fixed_point_spring_vec_entity::FixedPointSpringVecEntity, stick_vec_entity::StickVecEntity}, entity_system::EntitySystem}, event::event_system::EventSystem, level::level_builder::LevelBuilder, math::{vec2::Vec2, vec4::Vec4}, particles::{operations::{euler_integration::EulerIntegration, merge::Merge, metrics::Metrics, operation::Operation, split::Split, verlet_integration::VerletIntegration}, particle::Particle, particle_vec::ParticleVec, shape_builder::{adjacent_sticks::AdjacentSticks, circle::Circle, line_segment::LineSegment, rectangle::Rectangle, shape_builder::ShapeBuilder}}, platform::{app::App, camera::{Camera, CameraController}, instance_renderer::{Instance, InstanceRaw, InstanceRenderer, Vertex, QUAD_INDICES, QUAD_VERTICES}, model::{Material, Mesh}, plugin::Plugin, shader::Shader, texture}};


pub struct BasicParticles {
    camera: Option<Camera>,
    camera_controller: CameraController,
    particle_vec: ParticleVec,
    //fixed_point_spring_vec: FixedPointSpringVec,
    particle_instance_renderer: Option<InstanceRenderer>,
    quad_mesh: Option<Mesh>,
    material: Option<Material>,
    shader: Option<Shader>,
    frame_idx: u128,
    entity_system: EntitySystem,
    event_system: EventSystem,
}


fn setup_stick_test(entity_system: &mut EntitySystem, particle_vec: &mut ParticleVec) {
    // the ideal is particle size around diamter 1, radius = 0.5, as the spatial has has a grid size of 1!
    let particle_radius = 0.1;
    let static_large_mass = 100.0; //10000.0;

    // static
    {
        let red = Vec4::new(1.0, 0.0,0.0, 1.0);
        let mut builder = ShapeBuilder::new();
        builder.set_particle_template(Particle::default().set_colour(red)/* .set_static(true)*/.set_mass(static_large_mass).set_radius(particle_radius).clone())
            .apply_operation(LineSegment::new(Vec2::new(-5.0, 0.0), Vec2::new(5.0, 0.0)))
            .create_in_particle_vec(particle_vec);

        let fixed_point_spring_vec = FixedPointSpringVec::from_existing_particle_positions(&builder.particles.as_slice());
        entity_system.push(FixedPointSpringVecEntity::new(fixed_point_spring_vec));
    }

    // stick connecting 2 particles
    {
        let blue = Vec4::new(0.0, 0.0, 1.0, 1.0);

        let mut stick_vec = StickVec::new();

        let p1 = *Particle::default().set_pos(Vec2::new(0.0, 2.0)).set_colour(blue).set_mass(1.0).set_radius(particle_radius);
        let p2 = *Particle::default().set_pos(Vec2::new(0.0, 1.0)).set_colour(blue).set_mass(1.0).set_radius(particle_radius);
        
        let start_index = particle_vec.len();
        particle_vec.push(p1);
        particle_vec.push(p2);

        stick_vec.push(Stick::from_particles(particle_vec, [start_index, start_index + 1]));
        entity_system.push(StickVecEntity::new(stick_vec));
    }
}

fn setup_stick_test_2(entity_system: &mut EntitySystem, particle_vec: &mut ParticleVec) {
    // the ideal is particle size around diamter 1, radius = 0.5, as the spatial has has a grid size of 1!
    let particle_radius = 0.1;
    let static_large_mass = 100.0; //10000.0;

    // static
    {
        let red = Vec4::new(1.0, 0.0,0.0, 1.0);
        let mut builder = ShapeBuilder::new();
        builder.set_particle_template(Particle::default().set_colour(red)/* .set_static(true)*/.set_mass(static_large_mass).set_radius(particle_radius).clone())
            .apply_operation(LineSegment::new(Vec2::new(-5.0, 0.0), Vec2::new(5.0, 0.0)))
            .create_in_particle_vec(particle_vec);

        let fixed_point_spring_vec = FixedPointSpringVec::from_existing_particle_positions(&builder.particles.as_slice());
        entity_system.push(FixedPointSpringVecEntity::new(fixed_point_spring_vec));
    }

    // sticks connecting 3 particles into a triangle
    {
        let blue = Vec4::new(0.0, 0.0, 1.0, 1.0);

        let mut stick_vec = StickVec::new();

        let p1 = *Particle::default().set_pos(Vec2::new(0.0, 2.0)).set_colour(blue).set_mass(1.0).set_radius(particle_radius);
        let p2 = *Particle::default().set_pos(Vec2::new(0.0, 1.0)).set_colour(blue).set_mass(1.0).set_radius(particle_radius);
        let p3 = *Particle::default().set_pos(Vec2::new(0.5, 1.5)).set_colour(blue).set_mass(1.0).set_radius(particle_radius);
        
        let start_index = particle_vec.len();
        particle_vec.push(p1);
        particle_vec.push(p2);
        particle_vec.push(p3);

        stick_vec.push(Stick::from_particles(particle_vec, [start_index + 0, start_index + 1]));
        stick_vec.push(Stick::from_particles(particle_vec, [start_index + 1, start_index + 2]));
        stick_vec.push(Stick::from_particles(particle_vec, [start_index + 0, start_index + 2]));
        entity_system.push(StickVecEntity::new(stick_vec));
    }
}


fn setup_stick_test_3(entity_system: &mut EntitySystem, particle_vec: &mut ParticleVec) {
    // the ideal is particle size around diamter 1, radius = 0.5, as the spatial has has a grid size of 1!
    let particle_radius = 0.1;
    let static_large_mass = 100.0; //10000.0;

    // static
    {
        let red = Vec4::new(1.0, 0.0,0.0, 1.0);
        let mut builder = ShapeBuilder::new();
        builder.set_particle_template(Particle::default().set_colour(red)/* .set_static(true)*/.set_mass(static_large_mass).set_radius(particle_radius).clone())
            .apply_operation(LineSegment::new(Vec2::new(-5.0, 0.0), Vec2::new(5.0, 0.0)))
            .create_in_particle_vec(particle_vec);

        let fixed_point_spring_vec = FixedPointSpringVec::from_existing_particle_positions(&builder.particles.as_slice());
        entity_system.push(FixedPointSpringVecEntity::new(fixed_point_spring_vec));
    }

    // stick of circles
    {
        let blue = Vec4::new(0.0, 0.0, 1.0, 1.0);

        let mut stick_vec = StickVec::new();

        let origin = Vec2::new(0.0, 2.0);
        let circle_radius = 1.0;
        let particle_radius = 0.1;

        let mut builder = ShapeBuilder::new();
        builder.set_particle_template(Particle::default().set_colour(blue).set_radius(particle_radius).clone());
        builder.apply_operation(Circle::new(origin, circle_radius))
            .create_in_particle_vec(particle_vec);

        AdjacentSticks::new(Stick::default().set_stiffness_factor(1.0).clone(), 1, true).apply_to_particle_handles(particle_vec, &builder.particle_handles, &mut stick_vec); // connect adjacent points
        AdjacentSticks::new(Stick::default().set_stiffness_factor(1.0).clone(), 6, true).apply_to_particle_handles(particle_vec, &builder.particle_handles, &mut stick_vec); // connect every n points for extra stability during collisions
 
        entity_system.push(StickVecEntity::new(stick_vec));
    }
}


fn setup_circular_contained_liquid(entity_system: &mut EntitySystem, particle_vec: &mut ParticleVec) {
    // the ideal is particle size around diamter 1, radius = 0.5, as the spatial has has a grid size of 1!
    let particle_radius = 0.1;
    let static_large_mass = 100.0; //10000.0;

    // static
    let red = Vec4::new(1.0, 0.0,0.0, 1.0);
    let mut perimeter = ShapeBuilder::new();
    perimeter.set_particle_template(Particle::default().set_colour(red)/* .set_static(true)*/.set_mass(static_large_mass).set_radius(particle_radius).clone())
        .apply_operation(Circle::new(Vec2::new(0.0, 0.0), 5.0))
        .create_in_particle_vec(particle_vec);

    let fixed_point_spring_vec = FixedPointSpringVec::from_existing_particle_positions(&perimeter.particles.as_slice());
    entity_system.push(FixedPointSpringVecEntity::new(fixed_point_spring_vec));


    println!("Perimiter has particles from 0 to {}", particle_vec.len());

    // some dynamic particles on the inside   
    let blue = Vec4::new(0.0, 0.0, 1.0, 1.0); 
    let mut liquid = ShapeBuilder::new();
    liquid
        .set_particle_template(Particle::default().set_colour(blue).set_mass(1.0).set_radius(particle_radius).set_vel(Vec2::new(2.0, 0.0)).clone()) // .set_color(Color::from(LinearRgba::BLUE))
        .apply_operation(Rectangle::from_center_size(Vec2::new(0.0, 0.0), Vec2::new(5.0, 5.0)))
        .create_in_particle_vec(particle_vec);

    // // Lets debug what happens to this particle (top left of the fluid)
    // particle_vec[50].set_debug(true);

    //fixed_point_spring_vec
}

fn setup_3_particles(particle_vec: &mut ParticleVec) {
    let p1 = *Particle::default().set_pos(Vec2::new(0.0, 0.0)).set_static(true);
    let p2 = *Particle::default().set_pos(Vec2::new(2.0, 0.0)).set_vel(Vec2::new(-0.1, 0.0));
    particle_vec.push(p1);
    particle_vec.push(p2);

    let p3 = *Particle::default().set_pos(Vec2::new(1.0, 2.0)).set_vel(Vec2::new(-0.0, -0.1));
    particle_vec.push(p3);
}

impl BasicParticles {
    pub fn new() -> Self {
        let camera_controller = CameraController::new(0.2);

        let entity_system = EntitySystem::new();
        let particle_vec = ParticleVec::new();
        
        //setup_circular_contained_liquid(&mut entity_system, &mut particle_vec);
        //setup_3_particles(&mut particle_vec);

        Self {
            camera: None,
            camera_controller,
            particle_vec,
            //fixed_point_spring_vec,
            particle_instance_renderer: None,
            quad_mesh: None,
            material: None,
            shader: None,
            frame_idx: 0,
            entity_system,
            event_system: EventSystem::new(),
        }
    }

    fn update_particle_instances(&mut self, queue: &wgpu::Queue, device: &wgpu::Device) {
        let particle_instance_renderer = match &mut self.particle_instance_renderer {
            Some(s) => s,
            None => return,
        };

        // Add particles into the instance renderer
        let mut instances: Vec<Instance> = vec![]; 
        for i in 0..self.particle_vec.len() {
            // Skip debug particle so we can see where it is
            // if self.particle_vec[i].debug {
            //     continue;
            // }

            // todo: Clean this up with Instance::new()
            let position = cgmath::Vector3 {
                        x: self.particle_vec[i].pos[0],
                        y: self.particle_vec[i].pos[1],
                        z: 0.0,
                    };

            let rotation =
                        // this is needed so an object at (0, 0, 0) won't get scaled to zero
                        // as Quaternions can effect scale if they're not created correctly
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        );

            let colour = self.particle_vec[i].colour;
            let radius = self.particle_vec[i].radius;

            instances.push(Instance { position, rotation, colour, radius });
        }
        particle_instance_renderer.update_instances(&instances, &queue, &device);
    }
}

impl Plugin for BasicParticles {
    fn init(&mut self, app: &mut App) {
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        self.particle_instance_renderer = Some(InstanceRenderer::new(&state.device, &state.queue, &state.config));
        self.update_particle_instances(&state.queue, &state.device);

        self.quad_mesh = Some(Mesh::from_verticies_and_indicies("Quad".to_owned(), &state.device, QUAD_VERTICES, QUAD_INDICES));
        self.material = Some(Material::from_file("marble.png".to_owned(), &state.device, &state.queue));

        self.camera = Some(Camera::new(&state.device, state.config.width as f32 / state.config.height as f32));
        
        let camera = match &self.camera {
            Some(c) => c,
            None => return,
        };

        let diffuse_texture = match &self.material {
            Some(m) => &m.diffuse_texture,
            None => return,
        };

        self.shader = Some(Shader::new("particle_shader.wgsl".to_owned(), &state.device, 
            camera,
            diffuse_texture,
            &[Vertex::desc(), InstanceRaw::desc()],
            state.config.format,
        ));

        // Generate a procedural level.
        LevelBuilder::default().generate_level_based_on_date(&mut self.entity_system, &mut self.particle_vec);

        // Add car to the scene.
        let car = CarEntity::new(&mut self.particle_vec, Vec2::new(0.0, 1.0));
        self.entity_system.push(car);


        //setup_circular_contained_liquid(&mut self.entity_system, &mut self.particle_vec);
        //setup_stick_test(&mut self.entity_system, &mut self.particle_vec);
        //setup_stick_test_2(&mut self.entity_system, &mut self.particle_vec);
        //setup_stick_test_3(&mut self.entity_system, &mut self.particle_vec);
    }


    fn resize(&mut self, app: &mut App, width: u32, height: u32) {
        if width > 0 && height > 0 {
            let state = match &mut app.state {
                Some(s) => s,
                None => return,
            };
            let camera = match &mut self.camera {
                Some(c) => c,
                None => return,
            };
            camera.aspect = state.config.width as f32 / state.config.height as f32;
        }
    }

    fn window_event(&mut self, app: &mut App, event: WindowEvent) {
        self.event_system.queue_window_event(event);
    }

    fn handle_key(&mut self, app: &mut App, key: KeyCode, pressed: bool) {
        self.camera_controller.handle_key(key, pressed);

        // todo: this should occur when we handle window events in the event system
        self.entity_system.handle_key(key, pressed);
    }

    fn update(&mut self, app: &mut App) {
        // if self.frame_idx > 140 {
        //     thread::sleep(Duration::from_millis(200));
        // }
        
        self.frame_idx += 1;
        println!("F: {}", self.frame_idx);

        // Frame 151, the particle on the left (p50) gets merged and its not near anything! It seems there is a metaparticle P81 that is apparently nearby, but there should not be.
        // if self.frame_idx >= 151 {
        //     println!("slow frame?")
        // }

        let time_delta: f32 = 0.01;

        // Update particle system
        // todo: Need a ParticlePipeline to apply any number of Operations.
        // todo: The paper talks about doing this whole merge and split twice to avoid some problems.
        // todo: The paper also talks about limiting the depth of recursion on merge and split to avoid the whole thing becoming too ridgid.
        // todo: The paper mentions time step based such that a particle will not more more than its radius in 1 step due to the simple collision detection.
        {
            // Measure system metrics
            //let mut met = Metrics::default();
            //met.execute(&mut self.particle_vec);

            let mut m = Merge::default();
            m.execute(&mut self.particle_vec);

            let mut i = *VerletIntegration::default().set_time_delta(time_delta).set_gravity(Vec2::new(0.0, 0.0));
            i.execute(&mut self.particle_vec);

            // This should split particle.
            let mut s = Split::default().set_restitution_coefficient(1.0).clone();
            s.execute(&mut self.particle_vec);


            // Second merge and split - this fixes some particle penetration
            {
                let mut m = Merge::default();
                m.execute(&mut self.particle_vec);

                let mut s = Split::default().set_restitution_coefficient(1.0).clone();
                s.execute(&mut self.particle_vec);
            }

            // Measure metrics and see if anything has changed
            //met.execute(&mut self.particle_vec);
        }

        // Update camera, then apply the camera matrix to the particle instance renderer.
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        let camera = match &mut self.camera {
            Some(c) => c,
            None => return,
        };
        self.camera_controller.update_camera(camera);


        // Apply constraints
        self.entity_system.update(&mut self.particle_vec, camera, time_delta);


        let particle_instance_renderer = match &mut self.particle_instance_renderer {
            Some(p) => p,
            None => return,
        };

        camera.update_camera_uniform(&state.queue);
        //particle_instance_renderer.update_camera_uniform(&camera, &state.queue);


        // Update particles
        self.update_particle_instances(&state.queue, &state.device);
    }

    fn render(&self, app: &mut App) {
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        let render_context = state.render(|render_pass| {
            let shader = match &self.shader {
                Some(s) => s,
                None => return,
            };
            shader.bind(render_pass);

            let material = match &self.material {
                Some(m) => m,
                None => return,
            };
            material.bind(render_pass, 0);

            {
                let particle_instance_renderer = match &self.particle_instance_renderer {
                    Some(p) => p,
                    None => return,
                };
                particle_instance_renderer.render(render_pass);
            }

            // Trying to drawn an axis so we know which way is up and down
            {
                let quad_mesh = match &self.quad_mesh {
                    Some(m) => m,
                    None => return,
                };
                quad_mesh.render(render_pass, 0..1);
            }
        });
    }
}