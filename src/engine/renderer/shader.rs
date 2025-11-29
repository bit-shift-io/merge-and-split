use wgpu::{BindGroup, BindGroupLayout, ShaderModule};

use crate::engine::{app::camera::Camera, renderer::texture::{self, Texture}};


pub struct ShaderBuilder<'a> {
    shader_module: ShaderModule,
    device: &'a wgpu::Device,
    bind_group_layouts: Vec<BindGroupLayout>,
    bind_groups: Vec<BindGroup>,
}

impl<'a> ShaderBuilder<'a> {
    pub fn from_file(file_name: String, device: &'a wgpu::Device) -> Self {
        use std::fs;

        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("res")
            .join(file_name.clone());

        let shader_source = fs::read_to_string(path)
            .expect("Failed to read shader file");
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        Self {
            device,
            shader_module,
            bind_group_layouts: vec![],
            bind_groups: vec![],
        }
    }

    pub fn diffuse_texture(&mut self, diffuse_texture: &Texture) -> &mut Self {
        let texture_bind_group_layout =
            self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        self.bind_group_layouts.push(texture_bind_group_layout);
        self.bind_groups.push(diffuse_bind_group);

        self
    }

    pub fn camera(&mut self, camera: &Camera) -> &mut Self {
        let camera_bind_group_layout =
            self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_buffer = match &camera.camera_buffer {
            Some(c) => c,
            None => panic!("Camera buffer not initialized"),
        };

        let camera_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        self.bind_group_layouts.push(camera_bind_group_layout);
        self.bind_groups.push(camera_bind_group);

        self
    }

    pub fn build(&mut self, buffers: &'a [wgpu::VertexBufferLayout<'a>], format: wgpu::TextureFormat) -> Shader {
        let bind_group_layout_refs: Vec<&BindGroupLayout> = self.bind_group_layouts.iter().collect();
        let bind_group_layouts = bind_group_layout_refs.as_slice(); //.try_into().expect("Slice length mismatch");

        let render_pipeline_layout =
            self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: bind_group_layouts, //self.bind_group_layouts, //&[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &self.shader_module,
                entry_point: Some("vs_main"),
                buffers: buffers, //&[Vertex::desc(), InstanceRaw::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: format, //config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
            // Useful for optimizing shader compilation on Android
            cache: None,
        });

        Shader {
            render_pipeline,
            camera_bind_group: self.bind_groups[self.bind_groups.len() - 1].clone() // fixme! assumed first binding is the camera. should not use a hard coded number here!
        }
    }
}



pub struct Shader {
    render_pipeline: wgpu::RenderPipeline,
    camera_bind_group: wgpu::BindGroup,
}

impl Shader {
    pub fn new<'a>(file_name: String, device: &wgpu::Device, camera: &Camera, diffuse_texture: &Texture, buffers: &'a [wgpu::VertexBufferLayout<'a>], format: wgpu::TextureFormat) -> Self {
        let s = ShaderBuilder::from_file(file_name, device)
            .diffuse_texture(diffuse_texture)
            .camera(camera)
            .build(buffers, format);
        s
    }

    pub fn bind(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
    }
}