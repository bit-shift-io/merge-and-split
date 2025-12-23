use cgmath::prelude::*;
use wgpu::util::DeviceExt;

use crate::engine::app::event_system::KeyCodeType;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,

    // For now we only have 1 camera, so we can store the uniform and buffer here.
    pub camera_uniform: Option<CameraUniform>,
    pub camera_buffer: Option<wgpu::Buffer>,
}

impl Camera {
    pub fn new(device: &wgpu::Device, aspect: f32) -> Self {
        // We are using the RHS coordinate system.
        // Use your right hand and point the thumb along the x-axis, index fingers up along the Y Axis,
        // so Z is middle finger points towards you.
        let mut camera = Self {
            eye: (0.0, 5.0, 15.0).into(), // the position of the camera
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: aspect, //config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,

            camera_uniform: None,
            camera_buffer: None,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera.build_view_projection_matrix());

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        camera.camera_uniform = Some(camera_uniform);
        camera.camera_buffer = Some(camera_buffer);

        camera
    }
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }

    pub fn update_camera_uniform(&mut self, queue: &wgpu::Queue) {
        let projection_matrix = self.build_view_projection_matrix();

        let camera_uniform = match &mut self.camera_uniform {
            Some(c) => c,
            None => return,
        };

        let camera_buffer = match &self.camera_buffer {
            Some(c) => c,
            None => return,
        };
        
        camera_uniform.update_view_proj(&projection_matrix);
        queue.write_buffer(
            &camera_buffer,
            0,
            bytemuck::cast_slice(&[*camera_uniform]),
        );
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, projection_matrix: &cgmath::Matrix4<f32>) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * projection_matrix).into();
    }
}

pub struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyCodeType, is_pressed: bool) -> bool {
        match key {
            KeyCodeType::Space => {
                self.is_up_pressed = is_pressed;
                true
            }
            KeyCodeType::ShiftLeft => {
                self.is_down_pressed = is_pressed;
                true
            }
            KeyCodeType::KeyW | KeyCodeType::ArrowUp => {
                self.is_forward_pressed = is_pressed;
                true
            }
            KeyCodeType::KeyA | KeyCodeType::ArrowLeft => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCodeType::KeyS | KeyCodeType::ArrowDown => {
                self.is_backward_pressed = is_pressed;
                true
            }
            KeyCodeType::KeyD | KeyCodeType::ArrowRight => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the up/ down is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
