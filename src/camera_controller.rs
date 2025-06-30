use glam::Vec3;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;

pub struct CameraController {
    speed: f32,
    sensitivity: f32,
    forward_pressed: bool,
    backward_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
    mouse_pressed: bool,
    last_mouse_pos: (f32, f32),
    yaw: f32,
    pitch: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            forward_pressed: false,
            backward_pressed: false,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn process_keyboard(&mut self, key: KeyCode, state: ElementState) {
        let is_pressed = state == ElementState::Pressed;
        match key {
            KeyCode::KeyW => self.forward_pressed = is_pressed,
            KeyCode::KeyS => self.backward_pressed = is_pressed,
            KeyCode::KeyA => self.left_pressed = is_pressed,
            KeyCode::KeyD => self.right_pressed = is_pressed,
            KeyCode::Space => self.up_pressed = is_pressed,
            KeyCode::ShiftLeft => self.down_pressed = is_pressed,
            _ => {}
        }
    }

    pub fn process_mouse(&mut self, button: MouseButton, state: ElementState) {
        if button == MouseButton::Left {
            self.mouse_pressed = state == ElementState::Pressed;
        }
    }

    pub fn process_mouse_motion(&mut self, delta_x: f64, delta_y: f64) {
        // Always process mouse movement (FPS style)
        self.yaw -= delta_x as f32 * self.sensitivity;
        self.pitch -= delta_y as f32 * self.sensitivity;
        self.pitch = self.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
    }

    pub fn update_camera(&self, camera: &mut crate::camera::Camera, dt: f32) {
        // Calculate forward and right vectors based on yaw
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let forward = Vec3::new(sin_yaw, 0.0, cos_yaw).normalize();
        let right = Vec3::new(-cos_yaw, 0.0, sin_yaw).normalize();

        let mut velocity = Vec3::ZERO;

        if self.forward_pressed {
            velocity += forward;
        }
        if self.backward_pressed {
            velocity -= forward;
        }
        if self.right_pressed {
            velocity += right;
        }
        if self.left_pressed {
            velocity -= right;
        }
        if self.up_pressed {
            velocity += Vec3::Y;
        }
        if self.down_pressed {
            velocity -= Vec3::Y;
        }

        if velocity.length_squared() > 0.0 {
            camera.position += velocity.normalize() * self.speed * dt;
        }

        // Update camera target based on yaw and pitch
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        
        let look_dir = Vec3::new(
            cos_pitch * sin_yaw,
            sin_pitch,
            cos_pitch * cos_yaw,
        );
        
        camera.target = camera.position + look_dir;
    }

    pub fn set_initial_direction(&mut self, camera: &crate::camera::Camera) {
        let dir = (camera.target - camera.position).normalize();
        self.yaw = dir.z.atan2(dir.x);
        self.pitch = dir.y.asin();
    }
}