use glam::{Mat4, Vec3};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    camera_pos: [f32; 4], // vec3 position + padding for alignment
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            camera_pos: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        let view = Mat4::look_at_rh(camera.position, camera.target, Vec3::Y);
        let proj = Mat4::perspective_rh(
            camera.fov.to_radians(),
            camera.aspect,
            0.1,
            1000.0,
        );
        self.view_proj = (proj * view).to_cols_array_2d();
        self.camera_pos = [camera.position.x, camera.position.y, camera.position.z, 0.0];
    }
}

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub fov: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, fov: f32, aspect: f32) -> Self {
        Self {
            position,
            target,
            fov,
            aspect,
        }
    }

    pub fn from_scene(scene_camera: &crate::scene::Camera, aspect: f32) -> Self {
        Self {
            position: scene_camera.position,
            target: scene_camera.target,
            fov: scene_camera.fov,
            aspect,
        }
    }
}