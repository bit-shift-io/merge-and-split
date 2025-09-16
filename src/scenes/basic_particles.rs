use cgmath::Rotation3;
use winit::keyboard::KeyCode;

use crate::{math::Vec2, particles::{operations::{merge::Merge, r#move::Move, operation::Operation, split::Split}, particle::Particle, particle_vec::ParticleVec}, platform::{app::App, camera::CameraController, instance_renderer::{Instance, InstanceRenderer, QUAD_INDICES, QUAD_VERTICES}, model::{Material, Mesh}, plugin::Plugin}};


pub struct BasicParticles {
    camera_controller: CameraController,
    particle_vec: ParticleVec,
    particle_instance_renderer: Option<InstanceRenderer>,
    quad_mesh: Option<Mesh>,
    material: Option<Material>,
}

impl BasicParticles {
    pub fn new() -> Self {
        let camera_controller = CameraController::new(0.2);

        let mut particle_vec = ParticleVec::default();
        let p1 = *Particle::default().set_vel(Vec2::new(0.1, 0.0));
        let p2 = *Particle::default().set_pos(Vec2::new(2.0, 0.0));
        particle_vec.push(p1);
        particle_vec.push(p2);

        Self {
            camera_controller,
            particle_vec,
            particle_instance_renderer: None,
            quad_mesh: None,
            material: None,
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

            instances.push(Instance { position, rotation });
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
        self.material = Some(Material::from_file("happy-tree.png".to_owned(), &state.device, &state.queue));
    }

    fn handle_key(&mut self, app: &mut App, key: KeyCode, pressed: bool) {
        self.camera_controller.handle_key(key, pressed);
    }

    fn update(&mut self, app: &mut App) {
        // Update particle system
        // todo: Need a ParticlePipeline to apply any number of Operations.
        {
            let m = Merge::default();
            m.execute(&mut self.particle_vec);

            let o = *Move::default().set_time_delta(0.1);
            o.execute(&mut self.particle_vec);

            // This should split particle.
            let s = Split::default();
            s.execute(&mut self.particle_vec);
        }

        // Update camera, then apply the camera matrix to the particle instance renderer.
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        self.camera_controller.update_camera(&mut state.camera);

        let particle_instance_renderer = match &mut self.particle_instance_renderer {
            Some(p) => p,
            None => return,
        };
        particle_instance_renderer.update_camera_uniform(&state.camera, &state.queue);


        // Update particles
        self.update_particle_instances(&state.queue, &state.device);
    }

    fn render(&self, app: &mut App) {
        let state = match &mut app.state {
            Some(s) => s,
            None => return,
        };

        let render_context = state.render(|render_pass| {
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