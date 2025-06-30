pub mod pipeline;
pub mod buffer;

use crate::core::{state::GpuState, surface::RenderSurface};
use winit::window::Window;

pub struct Renderer<'window> {
    pub gpu_state: GpuState,
    pub surface: RenderSurface<'window>,
}

impl<'window> Renderer<'window> {
    pub async fn new(window: &'window Window) -> Self {
        let gpu_state = GpuState::new(window).await;
        let surface = RenderSurface::new(window, &gpu_state.instance, &gpu_state.adapter, &gpu_state.device);

        Self { gpu_state, surface }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.surface.resize(&self.gpu_state.device, new_size);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.gpu_state.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.gpu_state.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}