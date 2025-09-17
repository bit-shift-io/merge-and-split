use cgmath::prelude::*;
use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::Window
};

use crate::platform::{camera::Camera, texture};


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

// Quad on the XY plane. Clockwise. Made up of 2 triangles.
pub const QUAD_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 1.0],
    },
];

// WGPU, CCW is front face by default.
pub const QUAD_INDICES: &[u16] = &[3, 1, 0, 3, 2, 1];

const NUM_INSTANCES_PER_ROW: u32 = 5;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
);

#[derive(Debug, Copy, Clone)]
pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    #[allow(dead_code)]
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We don't have to do this in code though.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}



#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

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

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
    }
}

pub struct InstanceRenderer {
    //render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    //#[allow(dead_code)]
    //diffuse_texture: texture::Texture,
    //diffuse_bind_group: wgpu::BindGroup,
    
    num_instances: usize,
    #[allow(dead_code)]
    instance_buffer: wgpu::Buffer,

    //pub camera_uniform: CameraUniform,
    //pub camera_buffer: wgpu::Buffer,
    //camera_bind_group: wgpu::BindGroup,
}

impl InstanceRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) -> Self {
        // let diffuse_bytes = include_bytes!("../../res/happy-tree.png");
        // let diffuse_texture =
        //     texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

        // let texture_bind_group_layout =
        //     device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //         entries: &[
        //             wgpu::BindGroupLayoutEntry {
        //                 binding: 0,
        //                 visibility: wgpu::ShaderStages::FRAGMENT,
        //                 ty: wgpu::BindingType::Texture {
        //                     multisampled: false,
        //                     view_dimension: wgpu::TextureViewDimension::D2,
        //                     sample_type: wgpu::TextureSampleType::Float { filterable: true },
        //                 },
        //                 count: None,
        //             },
        //             wgpu::BindGroupLayoutEntry {
        //                 binding: 1,
        //                 visibility: wgpu::ShaderStages::FRAGMENT,
        //                 ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        //                 count: None,
        //             },
        //         ],
        //         label: Some("texture_bind_group_layout"),
        //     });

        // let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &texture_bind_group_layout,
        //     entries: &[
        //         wgpu::BindGroupEntry {
        //             binding: 0,
        //             resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
        //         },
        //         wgpu::BindGroupEntry {
        //             binding: 1,
        //             resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
        //         },
        //     ],
        //     label: Some("diffuse_bind_group"),
        // });

        // let camera_uniform = CameraUniform::new();
        // //camera_uniform.update_view_proj(&camera); // TODO: fixme?

        // let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Camera Buffer"),
        //     contents: bytemuck::cast_slice(&[camera_uniform]),
        //     usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        // });

        let instances = vec![];
        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        // let camera_bind_group_layout =
        //     device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //         entries: &[wgpu::BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: wgpu::ShaderStages::VERTEX,
        //             ty: wgpu::BindingType::Buffer {
        //                 ty: wgpu::BufferBindingType::Uniform,
        //                 has_dynamic_offset: false,
        //                 min_binding_size: None,
        //             },
        //             count: None,
        //         }],
        //         label: Some("camera_bind_group_layout"),
        //     });

        // let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &camera_bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: camera_buffer.as_entire_binding(),
        //     }],
        //     label: Some("camera_bind_group"),
        // });

        // let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        //     label: Some("Shader"),
        //     source: wgpu::ShaderSource::Wgsl(include_str!("particle_shader.wgsl").into()),
        // });

        // // let depth_texture =
        // //     texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        // let render_pipeline_layout =
        //     device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        //         label: Some("Render Pipeline Layout"),
        //         bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
        //         push_constant_ranges: &[],
        //     });

        // let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        //     label: Some("Render Pipeline"),
        //     layout: Some(&render_pipeline_layout),
        //     vertex: wgpu::VertexState {
        //         module: &shader,
        //         entry_point: Some("vs_main"),
        //         buffers: &[Vertex::desc(), InstanceRaw::desc()],
        //         compilation_options: Default::default(),
        //     },
        //     fragment: Some(wgpu::FragmentState {
        //         module: &shader,
        //         entry_point: Some("fs_main"),
        //         targets: &[Some(wgpu::ColorTargetState {
        //             format: config.format,
        //             blend: Some(wgpu::BlendState {
        //                 color: wgpu::BlendComponent::REPLACE,
        //                 alpha: wgpu::BlendComponent::REPLACE,
        //             }),
        //             write_mask: wgpu::ColorWrites::ALL,
        //         })],
        //         compilation_options: Default::default(),
        //     }),
        //     primitive: wgpu::PrimitiveState {
        //         topology: wgpu::PrimitiveTopology::TriangleList,
        //         strip_index_format: None,
        //         front_face: wgpu::FrontFace::Ccw,
        //         cull_mode: Some(wgpu::Face::Back),
        //         // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
        //         // or Features::POLYGON_MODE_POINT
        //         polygon_mode: wgpu::PolygonMode::Fill,
        //         // Requires Features::DEPTH_CLIP_CONTROL
        //         unclipped_depth: false,
        //         // Requires Features::CONSERVATIVE_RASTERIZATION
        //         conservative: false,
        //     },
        //     depth_stencil: Some(wgpu::DepthStencilState {
        //         format: texture::Texture::DEPTH_FORMAT,
        //         depth_write_enabled: true,
        //         depth_compare: wgpu::CompareFunction::Less,
        //         stencil: wgpu::StencilState::default(),
        //         bias: wgpu::DepthBiasState::default(),
        //     }),
        //     multisample: wgpu::MultisampleState {
        //         count: 1,
        //         mask: !0,
        //         alpha_to_coverage_enabled: false,
        //     },
        //     // If the pipeline will be used with a multiview render pass, this
        //     // indicates how many array layers the attachments will have.
        //     multiview: None,
        //     // Useful for optimizing shader compilation on Android
        //     cache: None,
        // });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = QUAD_INDICES.len() as u32;

        Self {
            //render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            //diffuse_texture,
            //diffuse_bind_group,
            
            num_instances: instances.len(),
            instance_buffer,

            //camera_uniform,
            //camera_buffer,
            //camera_bind_group,
        }
    }

    pub fn update_instances(&mut self, instances: &Vec<Instance>, queue: &wgpu::Queue, device: &wgpu::Device) {
        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        
        let instance_data_size = instance_data.len() * std::mem::size_of::<InstanceRaw>();
        let instance_buffer_size = self.instance_buffer.size() as usize;

        if instance_data_size <= instance_buffer_size {
            // Write directly to the existing buffer
            queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instance_data));
        } else {
            // We need a new instance buffer
            self.instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        }

        self.num_instances = instances.len();
    }

    // pub fn update_camera_uniform(&mut self, camera: &Camera, queue: &wgpu::Queue) {
    //     self.camera_uniform.update_view_proj(camera);
    //     queue.write_buffer(
    //         &self.camera_buffer,
    //         0,
    //         bytemuck::cast_slice(&[self.camera_uniform]),
    //     );
    // }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        //render_pass.set_pipeline(&self.render_pipeline);
        //render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        //render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..self.num_instances as u32);
    }
}