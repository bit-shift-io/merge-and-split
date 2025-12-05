use std::env;

use cgmath::Rotation3;

use crate::{core::math::vec2::Vec2, engine::{app::{app::App, camera::{Camera, CameraController}, plugin::Plugin}, renderer::{instance_renderer::{Instance, InstanceRaw, InstanceRenderer, QUAD_INDICES, QUAD_VERTICES, Vertex}, model::{Material, Mesh}, shader::{Shader, ShaderBuilder}}}, game::{entity::{entities::car_entity::CarEntity, entity_system::EntitySystem}, event::event_system::{EventSystem, GameEvent, KeyCodeType}, level::level_builder::LevelBuilder}, simulation::particles::{particle_vec::ParticleVec, simulation::Simulation, simulation_demos::SimulationDemos}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Finished,
}

pub struct Game {
    camera: Camera,
    camera_controller: CameraController,
    particle_vec: ParticleVec,
    particle_instance_renderer: InstanceRenderer,
    quad_mesh: Mesh,
    material: Material,
    particle_shader: Shader,
    line_shader: Shader,
    frame_idx: u128,
    entity_system: EntitySystem,
    event_system: EventSystem,
    simulation: Simulation,
    total_time: f32,
    game_state: GameState,
}

impl Game {
    fn update_particle_instances(&mut self, queue: &wgpu::Queue, device: &wgpu::Device) {
        // Add particles into the instance renderer
        let mut instances: Vec<Instance> = vec![]; 

        // Switch between the planck-time-trials particle system (old) and the new unified particle system
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

impl Plugin for Game {
    fn new(state: &crate::engine::app::state::State) -> Self {
        let camera_controller = CameraController::new(0.2);

        let mut entity_system = EntitySystem::new();
        let mut particle_vec = ParticleVec::new();
        
        // Create simulation with deterministic RNG seeded from beginning of day
        // This ensures consistent gameplay for the same day
        let rng = crate::core::math::random::Random::seed_from_beginning_of_day();
        let mut simulation = Simulation::new(rng);

        let particle_instance_renderer = InstanceRenderer::new(&state.device, &state.queue, &state.config);
        
        let quad_mesh = Mesh::from_verticies_and_indicies("Quad".to_owned(), &state.device, QUAD_VERTICES, QUAD_INDICES);
        let material = Material::from_file("marble.png".to_owned(), &state.device, &state.queue);

        let camera = Camera::new(&state.device, state.config.width as f32 / state.config.height as f32);
        
        let diffuse_texture = &material.diffuse_texture;

        let particle_shader = ShaderBuilder::from_file("particle_shader.wgsl".to_owned(), &state.device)
            .camera(&camera)
            .diffuse_texture(diffuse_texture)
            .build(&[Vertex::desc(), InstanceRaw::desc()], state.config.format);
        
        let line_shader = ShaderBuilder::from_file("line_shader.wgsl".to_owned(), &state.device)
            .camera(&camera)
            .build(&[Vertex::desc(), InstanceRaw::desc()], state.config.format);

        // Determine what "scene" to load based on command line argument.
        let args: Vec<String> = env::args().collect();
        let scene = if args.len() >= 2 { args[1].clone() } else { String::from("") };
        
        // Check if we should load a replay
        let replay_file = if args.len() >= 3 && args[1] == "replay" {
            Some(args[2].clone())
        } else {
            None
        };
        
        let is_demo_scene = match scene.as_str() {
            "friction" => { SimulationDemos::init_friction(&mut simulation); true }
            "granular" => { SimulationDemos::init_granular(&mut simulation); true }
            "sdf" => { SimulationDemos::init_sdf(&mut simulation); true }
            "boxes" => { SimulationDemos::init_boxes(&mut simulation); true }
            "wall" => { SimulationDemos::init_wall(&mut simulation); true }
            "pendulum" => { SimulationDemos::init_pendulum(&mut simulation); true }
            "rope" => { SimulationDemos::init_rope(&mut simulation); true }
            "fluid" => { SimulationDemos::init_fluid(&mut simulation); true }
            "fluid_solid" => { SimulationDemos::init_fluid_solid(&mut simulation); true }
            "gas" => { SimulationDemos::init_gas(&mut simulation); true }
            "water_balloon" => { SimulationDemos::init_water_balloon(&mut simulation); true }
            "newtons_cradle" => { SimulationDemos::init_newtons_cradle(&mut simulation); true }
            "smoke_open" => { SimulationDemos::init_smoke_open(&mut simulation); true }
            "smoke_closed" => { SimulationDemos::init_smoke_closed(&mut simulation); true }
            "rope_gas" => { SimulationDemos::init_rope_gas(&mut simulation); true }
            "volcano" => { SimulationDemos::init_volcano(&mut simulation); true }
            "wrecking_ball" => { SimulationDemos::init_wrecking_ball(&mut simulation); true }
            "replay" | _ => {
                // Generate a procedural level for replay
                LevelBuilder::default().generate_level_based_on_date(&mut entity_system, &mut particle_vec, &mut simulation);
                let car = CarEntity::new(&mut particle_vec, &mut simulation, Vec2::new(0.0, 1.0));
                entity_system.car_entity_system.push(car);
                false
            }
        };

        let mut event_system = EventSystem::new();
        
        // Handle replay mode from command line
        if let Some(replay_path) = replay_file {
            if let Err(e) = event_system.load_replay(&replay_path) {
                eprintln!("Failed to load replay file '{}': {}", replay_path, e);
            } else {
                event_system.start_replay();
            }
        } else if !is_demo_scene {
            // Only start recording for non-demo scenes (actual gameplay)
            event_system.start_recording();
        }

        let mut particles = Self {
            camera,
            camera_controller,
            particle_vec,
            particle_instance_renderer,
            quad_mesh,
            material,
            particle_shader,
            line_shader,
            frame_idx: 0,
            entity_system,
            event_system,
            simulation,
            total_time: 0.0,
            game_state: GameState::Playing,
        };

        particles.update_particle_instances(&state.queue, &state.device);
        particles
    }

    fn resize(&mut self, app: &mut App<Game>, width: u32, height: u32) {
        if width > 0 && height > 0 {
            let state = match &mut app.state {
                Some(s) => s,
                None => return,
            };
            self.camera.aspect = state.config.width as f32 / state.config.height as f32;
        }
    }

    fn window_event(&mut self, _app: &mut App<Game>, event: GameEvent) {
        self.event_system.queue_event(event);
    }

    fn update(&mut self, app: &mut App<Game>) {
        // if self.frame_idx > 140 {
        //     thread::sleep(Duration::from_millis(200));
        // }
        
        self.frame_idx += 1;
        //println!("F: {}", self.frame_idx);

        // Update event system with current frame number
        self.event_system.set_frame(self.frame_idx);
        self.event_system.process_events();

        // Process events from the event system
        for event in self.event_system.events.iter() {
            match event {
                GameEvent::KeyboardInput { key_code, state } => {
                    let is_pressed = matches!(state, crate::game::event::event_system::ElementStateType::Pressed);

                    self.camera_controller.handle_key(*key_code, is_pressed);
                    self.entity_system.handle_key(*key_code, is_pressed);
                }
                _ => {} // Handle other events if needed
            }
        }
        
        // Clear events after processing
        self.event_system.clear_events();

        // Frame 151, the particle on the left (p50) gets merged and its not near anything! It seems there is a metaparticle P81 that is apparently nearby, but there should not be.
        // if self.frame_idx >= 151 {
        //     println!("slow frame?")
        // }

        let time_delta: f32 = 0.005;

        self.simulation.pre_solve(time_delta);
        self.entity_system.elevator_entity_system.update_counts(&mut self.simulation);

        for i in 0..3 {
            self.simulation.solve(time_delta, 3, i);
            self.entity_system.elevator_entity_system.solve_constraints(&mut self.simulation, time_delta);
        }

        self.simulation.post_solve(time_delta);
        
        // Update camera, then apply the camera matrix to the particle instance renderer.
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        self.camera_controller.update_camera(&mut self.camera);


        // Apply constraints
        self.total_time += time_delta;
        self.entity_system.update(&mut self.particle_vec, &mut self.simulation, &mut self.camera, time_delta, self.total_time);

        // Check if game has finished (car reached finish line)
        if self.game_state == GameState::Playing {
            // Check if any car has finished
            let game_finished = self.entity_system.car_entity_system.0.iter().any(|car| car.game_ended);
            
            if game_finished {
                self.game_state = GameState::Finished;
                
                // Auto-export recording if we were recording
                if self.event_system.is_recording() {
                    self.event_system.stop_recording();
                    let filename = "recording.json"; //format!("recording_{}.json", chrono::Local::now().format("%Y%m%d_%H%M%S"));
                    if let Err(e) = self.event_system.export_recording(&filename) {
                        eprintln!("Failed to auto-export recording: {}", e);
                    } else {
                        println!("Recording auto-exported to: {}", filename);
                    }
                }
            }
        }

        self.camera.update_camera_uniform(&state.queue);
        //particle_instance_renderer.update_camera_uniform(&camera, &state.queue);


        // Update particles
        self.update_particle_instances(&state.queue, &state.device);
    }

    fn render(&self, app: &mut App<Game>) {
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        let render_context = state.render(|render_pass| {
            self.particle_shader.bind(render_pass);
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