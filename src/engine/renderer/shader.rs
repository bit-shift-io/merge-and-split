use wgpu::{BindGroup, BindGroupLayout, ShaderModule};
use std::collections::HashMap;

use crate::engine::{app::camera::Camera, renderer::texture::{self, Texture}};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BindGroupType {
    Camera,
    Diffuse,
}

const BIND_GROUP_MAPPINGS: &[(&str, BindGroupType)] = &[
    ("camera", BindGroupType::Camera),
    ("cam", BindGroupType::Camera),
    ("diffuse", BindGroupType::Diffuse),
];

pub struct ShaderBuilder<'a> {
    shader_module: ShaderModule,
    device: &'a wgpu::Device,
    mappings: HashMap<BindGroupType, u32>,
    bind_groups_map: HashMap<u32, (BindGroupLayout, BindGroup)>,
}

impl<'a> ShaderBuilder<'a> {
    pub fn from_file(file_name: String, device: &'a wgpu::Device) -> Self {
        use std::fs;

        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("res")
            .join(file_name.clone());

        let shader_source = fs::read_to_string(path)
            .expect("Failed to read shader file");
        
        let mappings = Self::parse_wgsl(&shader_source);

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        Self {
            device,
            shader_module,
            mappings,
            bind_groups_map: HashMap::new(),
        }
    }

    fn parse_wgsl(source: &str) -> HashMap<BindGroupType, u32> {
        let mut mappings = HashMap::new();
        let lines: Vec<&str> = source.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if line.contains("@group(") {
                // simple parsing strategy:
                // 1. find @group(X)
                // 2. look at the next line (or same line) for variable name
                // 3. if variable name contains a known keyword, map it.
                
                let start = line.find("@group(").unwrap() + 7;
                let end = line[start..].find(")").unwrap() + start;
                let group_index: u32 = line[start..end].parse().unwrap_or(0);

                // Look ahead for variable name
                // This is a bit fragile but works for the current shader style
                // We expect something like: var<uniform> camera: CameraUniform;
                // or var t_diffuse: texture_2d<f32>;
                
                let mut context = String::new();
                context.push_str(line);
                if i + 1 < lines.len() {
                    context.push_str(lines[i+1]);
                }

                for (key, variant) in BIND_GROUP_MAPPINGS {
                    if context.contains(key) {
                        mappings.insert(*variant, group_index);
                        // Once found, we might want to stop searching for this group?
                        // But maybe multiple keys match? 
                        // Let's assume one match per group for now, or last one wins.
                        // Actually, if we have "camera" and "cam", "camera" contains "cam".
                        // So order matters if we break. 
                        // But here we just insert. If multiple match, last one wins.
                        // Ideally we should match the longest key first or exact match.
                        // But context.contains is loose.
                    }
                }
            }
        }
        mappings
    }

    pub fn diffuse_texture(&mut self, diffuse_texture: &Texture) -> &mut Self {
        let group_index = *self.mappings.get(&BindGroupType::Diffuse).expect("Shader does not have a 'diffuse' bind group");

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

        self.bind_groups_map.insert(group_index, (texture_bind_group_layout, diffuse_bind_group));
        
        self
    }

    pub fn camera(&mut self, camera: &Camera) -> &mut Self {
        let group_index = *self.mappings.get(&BindGroupType::Camera).expect("Shader does not have a 'camera' bind group");

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

        self.bind_groups_map.insert(group_index, (camera_bind_group_layout, camera_bind_group));

        self
    }

    pub fn build(&mut self, buffers: &'a [wgpu::VertexBufferLayout<'a>], format: wgpu::TextureFormat) -> Shader {
        let mut bind_groups_map = std::mem::take(&mut self.bind_groups_map);
        let max_index = bind_groups_map.keys().max().copied().unwrap_or(0);
        
        let mut bind_group_layouts = Vec::new();
        let mut bind_groups = Vec::new();

        // Ensure we have all bind groups from 0 to max_index
        for i in 0..=max_index {
            if let Some((layout, group)) = bind_groups_map.remove(&i) {
                bind_group_layouts.push(layout);
                bind_groups.push(group);
            } else {
                // If the map is empty (no bind groups), max_index is 0. 
                // If we didn't insert anything, loop runs once for 0.
                // If map is empty, remove(&0) returns None.
                // But if map is empty, we shouldn't panic if max_index is 0?
                // Wait, if map is empty, max_index is 0. Loop runs for i=0.
                // remove(0) is None. Panic.
                // Correct behavior: if map is empty, we shouldn't loop?
                // Or we should check if map is empty first.
                if bind_groups_map.is_empty() && max_index == 0 {
                    // No bind groups, do nothing
                    break;
                }
                panic!("Missing bind group for index {}", i);
            }
        }

        let bind_group_layout_refs: Vec<&BindGroupLayout> = bind_group_layouts.iter().collect();
        let bind_group_layouts_slice = bind_group_layout_refs.as_slice();

        let render_pipeline_layout =
            self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: bind_group_layouts_slice,
                push_constant_ranges: &[],
            });

        let render_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &self.shader_module,
                entry_point: Some("vs_main"),
                buffers: buffers,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: format,
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
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
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
            multiview: None,
            cache: None,
        });

        Shader {
            render_pipeline,
            bind_groups,
        }
    }
}



pub struct Shader {
    render_pipeline: wgpu::RenderPipeline,
    bind_groups: Vec<BindGroup>,
}

impl Shader {
    pub fn bind(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        for (i, group) in self.bind_groups.iter().enumerate() {
            render_pass.set_bind_group(i as u32, group, &[]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wgsl() {
        let source = r#"
struct CameraUniform {
    view_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;
"#;
        let mappings = ShaderBuilder::parse_wgsl(source);
        
        assert_eq!(mappings.get(&BindGroupType::Camera), Some(&1));
        assert_eq!(mappings.get(&BindGroupType::Diffuse), Some(&0));
    }

    #[test]
    fn test_parse_wgsl_alias() {
        let source = r#"
@group(2) @binding(0)
var<uniform> cam: CameraUniform;
"#;
        let mappings = ShaderBuilder::parse_wgsl(source);
        
        assert_eq!(mappings.get(&BindGroupType::Camera), Some(&2));
    }
}