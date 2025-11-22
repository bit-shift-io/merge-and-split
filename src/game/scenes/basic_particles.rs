use std::{env, thread, time::Duration};

use cgmath::Rotation3;
use winit::{event::WindowEvent, keyboard::KeyCode};

use crate::{core::math::vec2::Vec2, engine::{app::{app::App, camera::{Camera, CameraController}, plugin::Plugin}, renderer::{instance_renderer::{Instance, InstanceRaw, InstanceRenderer, QUAD_INDICES, QUAD_VERTICES, Vertex}, model::{Material, Mesh}, shader::Shader}}, game::{entity::{entities::car_entity::CarEntity, entity_system::EntitySystem}, event::event_system::EventSystem, level::level_builder::LevelBuilder}, simulation::particles::{particle_vec::ParticleVec, simulation::Simulation, simulation_demos::SimulationDemos}};

pub struct BasicParticles {
    camera: Camera,
    camera_controller: CameraController,
    particle_vec: ParticleVec,
    //fixed_point_spring_vec: FixedPointSpringVec,
    particle_instance_renderer: InstanceRenderer,
    quad_mesh: Mesh,
    material: Material,
    shader: Shader,
    frame_idx: u128,
    entity_system: EntitySystem,
    event_system: EventSystem,

    simulation: Simulation
}

impl BasicParticles {
    fn update_particle_instances(&mut self, queue: &wgpu::Queue, device: &wgpu::Device) {
        // Add particles into the instance renderer
        let mut instances: Vec<Instance> = vec![]; 

        // Switch between the merge-and-split particle system (old) and the new unified particle system
        //let particles = &self.particle_vec;
        let particles = &self.simulation.particles;

        for i in 0..particles.len() {
            // Skip debug particle so we can see where it is
            // if self.particle_vec[i].debug {
            //     continue;
            // }

            // todo: Clean this up with Instance::new()
            let position = cgmath::Vector3 {
                        x: particles[i].pos[0],
                        y: particles[i].pos[1],
                        z: 0.0,
                    };

            let rotation =
                        // this is needed so an object at (0, 0, 0) won't get scaled to zero
                        // as Quaternions can effect scale if they're not created correctly
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        );

            let colour = particles[i].colour;
            let radius = particles[i].radius;

            instances.push(Instance { position, rotation, colour, radius });
        }
        self.particle_instance_renderer.update_instances(&instances, &queue, &device);
    }
}

impl Plugin for BasicParticles {
    fn new(state: &crate::engine::app::state::State) -> Self {
        let camera_controller = CameraController::new(0.2);

        let mut entity_system = EntitySystem::new();
        let mut particle_vec = ParticleVec::new();
        let mut simulation = Simulation::new();

        let particle_instance_renderer = InstanceRenderer::new(&state.device, &state.queue, &state.config);
        
        let quad_mesh = Mesh::from_verticies_and_indicies("Quad".to_owned(), &state.device, QUAD_VERTICES, QUAD_INDICES);
        let material = Material::from_file("marble.png".to_owned(), &state.device, &state.queue);

        let camera = Camera::new(&state.device, state.config.width as f32 / state.config.height as f32);
        
        let diffuse_texture = &material.diffuse_texture;

        let shader = Shader::new("particle_shader.wgsl".to_owned(), &state.device, 
            &camera,
            diffuse_texture,
            &[Vertex::desc(), InstanceRaw::desc()],
            state.config.format,
        );

        // Determine what "scene" to load based on command line argument.
        let args: Vec<String> = env::args().collect();
        let scene = if args.len() >= 2 { args[1].clone() } else { String::from("") };
        match scene.as_str() {
            "friction" => SimulationDemos::init_friction(&mut simulation),
            "granular" => SimulationDemos::init_granular(&mut simulation),
            "sdf" => SimulationDemos::init_sdf(&mut simulation),
            "boxes" => SimulationDemos::init_boxes(&mut simulation),
            "wall" => SimulationDemos::init_wall(&mut simulation),
            "pendulum" => SimulationDemos::init_pendulum(&mut simulation),
            "rope" => SimulationDemos::init_rope(&mut simulation),
            "fluid" => SimulationDemos::init_fluid(&mut simulation),
            "fluid_solid" => SimulationDemos::init_fluid_solid(&mut simulation),
            "gas" => SimulationDemos::init_gas(&mut simulation),
            "water_balloon" => SimulationDemos::init_water_balloon(&mut simulation),
            "newtons_cradle" => SimulationDemos::init_newtons_cradle(&mut simulation),
            "smoke_open" => SimulationDemos::init_smoke_open(&mut simulation),
            "smoke_closed" => SimulationDemos::init_smoke_closed(&mut simulation),
            "rope_gas" => SimulationDemos::init_rope_gas(&mut simulation),
            "volcano" => SimulationDemos::init_volcano(&mut simulation),
            "wrecking_ball" => SimulationDemos::init_wrecking_ball(&mut simulation),
            _ => {
                // Generate a procedural level.
                LevelBuilder::default().generate_level_based_on_date(&mut entity_system, &mut particle_vec, &mut simulation);

                // Add car to the scene.
                let car = CarEntity::new(&mut particle_vec, &mut simulation, Vec2::new(0.0, 1.0));
                entity_system.car_entity_system.push(car);
                //self.entity_system.push(car);
            }
        }

        let mut particles = Self {
            camera,
            camera_controller,
            particle_vec,
            //fixed_point_spring_vec,
            particle_instance_renderer,
            quad_mesh,
            material,
            shader,
            frame_idx: 0,
            entity_system,
            event_system: EventSystem::new(),

            simulation,
        };

        particles.update_particle_instances(&state.queue, &state.device);
        particles
    }

    fn resize(&mut self, app: &mut App<BasicParticles>, width: u32, height: u32) {
        if width > 0 && height > 0 {
            let state = match &mut app.state {
                Some(s) => s,
                None => return,
            };
            self.camera.aspect = state.config.width as f32 / state.config.height as f32;
        }
    }

    fn window_event(&mut self, app: &mut App<BasicParticles>, event: WindowEvent) {
        self.event_system.queue_window_event(event);
    }

    fn handle_key(&mut self, app: &mut App<BasicParticles>, key: KeyCode, pressed: bool) {
        self.camera_controller.handle_key(key, pressed);

        // todo: this should occur when we handle window events in the event system
        self.entity_system.handle_key(key, pressed);
    }

    fn update(&mut self, app: &mut App<BasicParticles>) {
        // if self.frame_idx > 140 {
        //     thread::sleep(Duration::from_millis(200));
        // }
        
        self.frame_idx += 1;
        println!("F: {}", self.frame_idx);

        // Frame 151, the particle on the left (p50) gets merged and its not near anything! It seems there is a metaparticle P81 that is apparently nearby, but there should not be.
        // if self.frame_idx >= 151 {
        //     println!("slow frame?")
        // }

        let time_delta: f32 = 0.005;

        self.simulation.tick_1(time_delta);
        self.entity_system.elevator_entity_system.update_counts(&mut self.simulation);

        for i in 0..3 {
            self.simulation.tick_2(time_delta, 3, i);
            self.entity_system.elevator_entity_system.solve_constraints(&mut self.simulation, time_delta);
        }

        self.simulation.tick_3(time_delta);

        // // Update particle system
        // // todo: Need a ParticlePipeline to apply any number of Operations.
        // // todo: The paper talks about doing this whole merge and split twice to avoid some problems.
        // // todo: The paper also talks about limiting the depth of recursion on merge and split to avoid the whole thing becoming too ridgid.
        // // todo: The paper mentions time step based such that a particle will not more more than its radius in 1 step due to the simple collision detection.
        // {
        //     // Measure system metrics
        //     //let mut met = Metrics::default();
        //     //met.execute(&mut self.particle_vec);

        //     let mut m = Merge::default();
        //     m.execute_2(&mut self.particle_vec, time_delta);

        //     let mut i = *VerletIntegration::default().set_time_delta(time_delta);//.set_gravity(Vec2::new(0.0, 0.0));
        //     i.execute(&mut self.particle_vec);

        //     // This should split particle.
        //     let mut s = Split::default().set_restitution_coefficient(1.0).clone();
        //     s.execute(&mut self.particle_vec);


        //     // Second merge and split - this fixes some particle penetration
        //     {
        //         let mut m = Merge::default();
        //         m.execute_2(&mut self.particle_vec, time_delta);

        //         let mut s = Split::default().set_restitution_coefficient(1.0).clone();
        //         s.execute(&mut self.particle_vec);
        //     }

        //     // Measure metrics and see if anything has changed
        //     //met.execute(&mut self.particle_vec);
        // }

        // Update camera, then apply the camera matrix to the particle instance renderer.
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        self.camera_controller.update_camera(&mut self.camera);


        // Apply constraints
        self.entity_system.update(&mut self.particle_vec, &mut self.simulation, &mut self.camera, time_delta);


        self.camera.update_camera_uniform(&state.queue);
        //particle_instance_renderer.update_camera_uniform(&camera, &state.queue);


        // Update particles
        self.update_particle_instances(&state.queue, &state.device);
    }

    fn render(&self, app: &mut App<BasicParticles>) {
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        let render_context = state.render(|render_pass| {
            self.shader.bind(render_pass);
            self.material.bind(render_pass, 0);

            {
                self.particle_instance_renderer.render(render_pass);
            }

            // Trying to drawn an axis so we know which way is up and down
            {
                self.quad_mesh.render(render_pass, 0..1);
            }
        });
    }
}