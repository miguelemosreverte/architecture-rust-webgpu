use wgpu::{RenderPipeline, Device, ShaderModule};

pub struct RenderPipelineBuilder {
    vertex_shader: Option<ShaderModule>,
    fragment_shader: Option<ShaderModule>,
}

impl RenderPipelineBuilder {
    pub fn new() -> Self {
        Self {
            vertex_shader: None,
            fragment_shader: None,
        }
    }

    pub fn vertex_shader(mut self, shader: ShaderModule) -> Self {
        self.vertex_shader = Some(shader);
        self
    }

    pub fn fragment_shader(mut self, shader: ShaderModule) -> Self {
        self.fragment_shader = Some(shader);
        self
    }

    pub fn build(self, device: &Device, format: wgpu::TextureFormat) -> RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &self.vertex_shader.unwrap(),
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.fragment_shader.unwrap(),
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }
}