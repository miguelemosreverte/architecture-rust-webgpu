use webgpu_rust_architecture::{
    camera::{Camera, CameraUniform},
    camera_controller::CameraController,
    scene::{loader::{load_scene_from_file, scene_to_mesh}, primitives::Vertex},
    shaders::SCENE_SHADER,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent, ElementState, MouseButton, DeviceEvent},
    event_loop::EventLoop,
    keyboard::{PhysicalKey, KeyCode},
    window::Window,
};
use wgpu::util::DeviceExt;
use std::path::Path;

fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: Some("Depth Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    };
    let texture = device.create_texture(&desc);
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

fn main() {
    env_logger::init();
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let scene_file = if args.len() > 1 {
        // Handle different argument formats
        let arg = &args[1];
        if arg.ends_with(".json") {
            // Full path to JSON file
            arg.clone()
        } else if let Ok(num) = arg.parse::<u32>() {
            // Just a number - use example file
            if num == 0 {
                "examples/10_full_house.json".to_string()
            } else if num <= 10 {
                format!("examples/{}_*.json", num)
                    .replace("*", match num {
                        1 => "single_wall",
                        2 => "wall_with_window",
                        3 => "simple_room",
                        4 => "room_with_door_window",
                        5 => "two_rooms",
                        6 => "room_with_furniture",
                        7 => "multi_level",
                        8 => "building_with_columns",
                        9 => "complex_floor_plan",
                        10 => "full_house",
                        _ => "simple_room",
                    })
            } else {
                eprintln!("Example number must be between 1-10 (or 0 for example 10)");
                std::process::exit(1);
            }
        } else {
            // Try to find the file in examples directory
            format!("examples/{}.json", arg)
        }
    } else {
        "examples/3_simple_room.json".to_string()
    };
    
    // Check if file exists
    if !std::path::Path::new(&scene_file).exists() {
        eprintln!("Error: Scene file '{}' not found!", scene_file);
        eprintln!("\nUsage:");
        eprintln!("  {} [scene_file.json]     # Load specific JSON file", args[0]);
        eprintln!("  {} [1-10]                # Load example by number", args[0]);
        eprintln!("  {} 3_simple_room         # Load example by name", args[0]);
        eprintln!("\nExamples:");
        eprintln!("  {} examples/5_two_rooms.json", args[0]);
        eprintln!("  {} 7", args[0]);
        eprintln!("  {} my_custom_scene.json", args[0]);
        std::process::exit(1);
    }
    
    println!("Loading scene: {}", scene_file);
    
    let event_loop = EventLoop::new().unwrap();
    let window = event_loop.create_window(Window::default_attributes()
        .with_title(format!("WebGPU Architecture - {}", scene_file))
        .with_inner_size(PhysicalSize::new(1024, 768)))
        .unwrap();

    let window = std::sync::Arc::new(window);
    let mut state = pollster::block_on(State::new(window.clone(), &scene_file));

    let _ = event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => target.exit(),
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::KeyboardInput {
                    event,
                    ..
                } => {
                    if let PhysicalKey::Code(keycode) = event.physical_key {
                        // Handle scene switching only on press
                        if event.state == ElementState::Pressed {
                            match keycode {
                                KeyCode::Digit1 => state.load_scene("examples/1_single_wall.json"),
                                KeyCode::Digit2 => state.load_scene("examples/2_wall_with_window.json"),
                                KeyCode::Digit3 => state.load_scene("examples/3_simple_room.json"),
                                KeyCode::Digit4 => state.load_scene("examples/4_room_with_door_window.json"),
                                KeyCode::Digit5 => state.load_scene("examples/5_two_rooms.json"),
                                KeyCode::Digit6 => state.load_scene("examples/6_room_with_furniture.json"),
                                KeyCode::Digit7 => state.load_scene("examples/7_multi_level.json"),
                                KeyCode::Digit8 => state.load_scene("examples/8_building_with_columns.json"),
                                KeyCode::Digit9 => state.load_scene("examples/9_complex_floor_plan.json"),
                                KeyCode::Digit0 => state.load_scene("examples/10_full_house.json"),
                                _ => {}
                            }
                        }
                        // Always process camera controls
                        state.camera_controller.process_keyboard(keycode, event.state);
                    }
                }
                WindowEvent::MouseInput { button, state: button_state, .. } => {
                    state.camera_controller.process_mouse(*button, *button_state);
                }
                WindowEvent::RedrawRequested => {
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            state.resize(window.inner_size());
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            target.exit();
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => {}
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                state.camera_controller.process_mouse_motion(delta.0, delta.1);
            }
            Event::AboutToWait => {
                // Update camera
                let now = std::time::Instant::now();
                let dt = (now - state.last_update).as_secs_f32();
                state.last_update = now;
                
                state.camera_controller.update_camera(&mut state.camera, dt);
                state.camera_uniform.update_view_proj(&state.camera);
                state.queue.write_buffer(
                    &state.camera_buffer,
                    0,
                    bytemuck::cast_slice(&[state.camera_uniform]),
                );
                
                window.request_redraw();
            }
            _ => {}
        }
    });
}

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    scene_path: String,
    depth_texture: wgpu::TextureView,
    camera_controller: CameraController,
    last_update: std::time::Instant,
}

impl State {
    async fn new(window: std::sync::Arc<winit::window::Window>, scene_file: &str) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Load scene
        let scene = load_scene_from_file(Path::new(scene_file))
            .expect("Failed to load scene");
        
        let mesh = scene_to_mesh(&scene);
        
        // Create camera
        let aspect = size.width as f32 / size.height as f32;
        let camera = Camera::from_scene(&scene.camera, aspect);
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        // Create camera buffer and bind group
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // Create vertex and index buffers
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = mesh.indices.len() as u32;

        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Scene Shader"),
            source: wgpu::ShaderSource::Wgsl(SCENE_SHADER.into()),
        });

        // Create pipeline layout
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Vertex buffer layout
        let vertex_buffers = [wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }];

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &vertex_buffers,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
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

        // Create depth texture
        let depth_texture = create_depth_texture(&device, &config);

        // Create camera controller
        let mut camera_controller = CameraController::new(10.0, 0.003);
        camera_controller.set_initial_direction(&camera);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            scene_path: scene_file.to_string(),
            depth_texture,
            camera_controller,
            last_update: std::time::Instant::now(),
        }
    }

    fn load_scene(&mut self, scene_file: &str) {
        println!("Loading scene: {}", scene_file);
        
        match load_scene_from_file(Path::new(scene_file)) {
            Ok(scene) => {
                let mesh = scene_to_mesh(&scene);
                
                // Update camera
                self.camera = Camera::from_scene(&scene.camera, self.size.width as f32 / self.size.height as f32);
                self.camera_controller.set_initial_direction(&self.camera);
                self.camera_uniform.update_view_proj(&self.camera);
                self.queue.write_buffer(
                    &self.camera_buffer,
                    0,
                    bytemuck::cast_slice(&[self.camera_uniform]),
                );
                
                // Recreate buffers
                self.vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&mesh.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

                self.index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

                self.num_indices = mesh.indices.len() as u32;
                self.scene_path = scene_file.to_string();
            }
            Err(e) => {
                eprintln!("Failed to load scene {}: {}", scene_file, e);
            }
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            // Recreate depth texture
            self.depth_texture = create_depth_texture(&self.device, &self.config);
            
            // Update camera aspect ratio
            self.camera.aspect = new_size.width as f32 / new_size.height as f32;
            self.camera_uniform.update_view_proj(&self.camera);
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_uniform]),
            );
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.7,
                            b: 0.9,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}