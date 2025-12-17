use std::env;

use cgmath::Rotation3;
use iced_wgpu::graphics::{Shell, Viewport};
use iced_wgpu::{Engine, Renderer};
use iced_winit::clipboard::Clipboard;
use iced_winit::core::mouse;
use iced_winit::core::renderer as core_renderer;
// use iced_winit::core::time::Instant; // Not used yet
use iced_winit::core::{Event as IcedEvent, Font, Pixels, Size, Theme};
use iced_winit::runtime::user_interface::{self, UserInterface};
use iced_winit::winit;

use crate::{core::math::vec2::Vec2, engine::{app::{app::App, camera::{Camera, CameraController}, plugin::Plugin, state::State}, renderer::{instance_renderer::{Instance, InstanceRaw, InstanceRenderer, QUAD_INDICES, QUAD_VERTICES, Vertex}, model::{Material, Mesh}, shader::{Shader, ShaderBuilder}}}, game::{entity::{entities::car_entity::CarEntity, entity_system::EntitySystem}, event::event_system::{EventSystem, GameEvent, KeyCodeType}, level::level_builder::LevelBuilder, irc::{IrcManager, IrcEvent}, leaderboard::Leaderboard}, simulation::particles::{particle_vec::ParticleVec, simulation::Simulation, simulation_demos::SimulationDemos}};
use cgmath::One;

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
    last_mouse_pos: Vec2,
    material: Material,
    particle_shader: Shader,
    line_shader: Shader,
    frame_idx: u128,
    entity_system: EntitySystem,
    event_system: EventSystem,
    simulation: Simulation,
    total_time: f32,
    game_state: GameState,
    irc_manager: IrcManager,
    current_nickname: String,
    leaderboard: Leaderboard,
    // UI
    // Iced UI
    ui: crate::game::ui::GameUI,
    ui_cache: user_interface::Cache,
    ui_renderer: Renderer,
    ui_events: Vec<IcedEvent>,
    ui_viewport: Viewport,
    ui_cursor: mouse::Cursor,
    ui_clipboard: Clipboard,
    
    modifiers: winit::event::Modifiers,
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


        let nickname = format!("Player{}", chrono::Utc::now().timestamp_subsec_micros());
        let irc_manager = IrcManager::new(
             "irc.libera.chat".to_owned(),
             nickname.clone(),
             vec!["#planck-global".to_owned(), "#planck-leaderboard".to_owned()]
        );

        let mut __ui = crate::game::ui::GameUI::new();
        
        // Iced Init
        let physical_size = state.window.inner_size();
        let viewport = Viewport::with_physical_size(
            Size::new(physical_size.width, physical_size.height),
            state.window.scale_factor() as f32,
        );
        let clipboard = Clipboard::connect(state.window.clone());

        let renderer = {
            let engine = Engine::new(
                &state.adapter,
                state.device.clone(),
                state.queue.clone(),
                state.config.format,
                None,
                Shell::headless(),
            );

            Renderer::new(engine, Font::default(), Pixels::from(16))
        };
        
        let mut particles = Self {
            camera,
            camera_controller,
            particle_vec,
            particle_instance_renderer,
            quad_mesh,
            material,
            particle_shader,
            line_shader,
            last_mouse_pos: Vec2::new(0.0, 0.0),
            frame_idx: 0,
            entity_system,
            event_system,
            simulation,
            total_time: 0.0,
            game_state: GameState::Playing,
            irc_manager,
            current_nickname: nickname,
            leaderboard: Leaderboard::new(),
            ui: __ui,
            ui_cache: user_interface::Cache::new(),
            ui_renderer: renderer,
            ui_events: Vec::new(),
            ui_viewport: viewport,
            ui_cursor: mouse::Cursor::Unavailable,
            ui_clipboard: clipboard,
            
            modifiers: winit::event::Modifiers::default(),
        };

        particles.update_particle_instances(&state.queue, &state.device);
        particles
    }

    fn resize(&mut self, app: &mut App<Self>, width: u32, height: u32) {
        if width > 0 && height > 0 {
            if let Some(state) = &mut app.state {
                 state.resize(width, height);
                 let scale_factor = state.window.scale_factor();
                 self.ui_viewport = Viewport::with_physical_size(
                      Size::new(width, height),
                      scale_factor as f32,
                 );
            }
            self.camera.aspect = width as f32 / height as f32;
        }
    }

    fn window_event(&mut self, app: &mut App<Game>, event: &winit::event::WindowEvent) {
        // Map window event to iced event
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        if let Some(iced_event) = iced_winit::conversion::window_event(
            event.clone(),
            state.window.scale_factor() as f32,
            Default::default(), // self.modifiers.into() replacement
        ) {
            self.ui_events.push(iced_event);
        }
        
        // Handle Game Events (Legacy/Recording)
        if let Some(game_event) = EventSystem::window_event_to_game_event(event) {
            self.event_system.queue_event(game_event);
        }

        match event {
             winit::event::WindowEvent::CursorMoved { position, .. } => {
                self.last_mouse_pos = Vec2::new(position.x as f32, position.y as f32);
                self.ui_cursor = mouse::Cursor::Available(iced_winit::conversion::cursor_position(
                    *position,
                    state.window.scale_factor() as f32,
                ));
            }
            winit::event::WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = new_modifiers.state().into();
            }
            _ => {}
        }
    }

    fn update(&mut self, app: &mut App<Self>) {
        if let Some(state) = &app.state {
             self.camera_controller.update_camera(&mut self.camera);

             // Simulation Update
             let dt = if app.dt <= 0.0 { 1.0 / 60.0 } else { app.dt };
             
             self.simulation.pre_solve(dt);
             self.simulation.solve(dt, 8, 0); // 8 iterations
             self.simulation.post_solve(dt);

             // Bridge to Renderer
             let instances: Vec<Instance> = self.simulation.particles.iter()
                .map(|p| Instance {
                    position: cgmath::Vector3::new(p.pos.x, p.pos.y, 0.0),
                    rotation: cgmath::Quaternion::one(),
                    colour: p.colour,
                    radius: p.radius,
                })
                .collect();
             
             self.particle_instance_renderer.update_instances(&instances, &state.queue, &state.device);

             // FPS Update
             let fps = (1.0 / dt).round() as i32;
             self.ui.update(crate::game::ui::Message::UpdateFps(fps));

        }

        
        self.frame_idx += 1;
        
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
        
        // Update UI FPS
        // let fps = 60; // TODO: Fix time delta access
        // self.ui_state.queue_message(crate::game::ui::Message::UpdateFps(fps));
        
        // Iced Update
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };
        let viewport_size = state.window.inner_size();
        // let logical_size = viewport_size.to_logical(state.window.scale_factor());
        // /*
        // let _ = self.ui_state.update(
        //     state.viewport().logical_size(),
        //     winit::dpi::PhysicalPoint::new(-1.0, -1.0), // TODO: Cursor position
        //     &mut self.modifiers,
        //     &mut iced::Theme::Dark,
        //     &iced::renderer::Style::default(),
        //     &mut self.ui_clipboard,
        //     &mut self.ui_debug,
        // );
        // */

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
                
                // Post time to leaderboard
                let seed = chrono::Utc::now().format("%Y-%m-%d").to_string();
                let msg = format!("BEST_TIME seed={} time={:.3} user={}", seed, self.total_time, self.current_nickname);
                self.irc_manager.send_message("#planck-leaderboard".to_owned(), msg);
                println!("Posted time to leaderboard: {:.3}", self.total_time);

                // Add my own score to the leaderboard immediately
                self.leaderboard.add_score(seed.clone(), self.current_nickname.clone(), self.total_time);

                // Post Top 10 to global chat
                if let Some(top10) = self.leaderboard.get_top_10(&seed) {
                    self.irc_manager.send_message("#planck-global".to_owned(), top10);
                }
            }
        }

        self.camera.update_camera_uniform(&state.queue);
        //particle_instance_renderer.update_camera_uniform(&camera, &state.queue);



        // Update particles
        self.update_particle_instances(&state.queue, &state.device);

        // Process IRC events
        for event in self.irc_manager.process_events() {
            match event {
                IrcEvent::Connected => println!("IRC: Connected!"),
                IrcEvent::MessageReceived { target, sender, message } => {
                    println!("[{}] <{}> {}", target, sender, message);
                    if target == "#planck-leaderboard" {
                         self.leaderboard.parse_message(&message);
                    }
                },
                IrcEvent::Disconnected => println!("IRC: Disconnected"),
            }
        }
    }

    fn render(&mut self, app: &mut App<Game>) {
        // Get state
        let state = match &mut app.state {
             Some(s) => s,
             None => return,
        };

        // 1. Get Texture
        let output = match state.surface.get_current_texture() {
            Ok(output) => output,
            Err(wgpu::SurfaceError::OutOfMemory) => {
                panic!("Swapchain error: OutOfMemory");
            }
            Err(_) => return,
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 2. Create Encoder
        let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // 3. Render Game Scene
        { // Scope for render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                             r: 0.1, g: 0.2, b: 0.3, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
        // Add depth stencil usage
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &state.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Bind resources
            self.particle_shader.bind(&mut render_pass);
            self.material.bind(&mut render_pass, 0);
            
            self.particle_instance_renderer.render(&mut render_pass);
            self.quad_mesh.render(&mut render_pass, 0..1);
        }
        
        // 4. Submit Game Scene
        state.queue.submit(std::iter::once(encoder.finish()));

        // 5. Iced Rendering
        let mut user_interface = UserInterface::build(
            self.ui.view(),
            self.ui_viewport.logical_size(),
            std::mem::take(&mut self.ui_cache),
            &mut self.ui_renderer,
        );
        
        let (_state, _) = user_interface.update(
            &self.ui_events,
            self.ui_cursor,
            &mut self.ui_renderer,
            &mut self.ui_clipboard,
            &mut Vec::new(),
        );
        
        user_interface.draw(
            &mut self.ui_renderer,
            &Theme::Dark,
            &Default::default(),
            self.ui_cursor,
        );
        
        self.ui_cache = user_interface.into_cache();
        self.ui_events.clear();
        
        self.ui_renderer.present(
            None,
            state.config.format,
            &view,
            &self.ui_viewport,
            // &[],
        );

        // 7. Present Surface
        output.present();
    }
}