use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Self {
            position: position.to_array(),
            normal: normal.to_array(),
            tex_coords: tex_coords.to_array(),
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> u16 {
        let index = self.vertices.len() as u16;
        self.vertices.push(vertex);
        index
    }

    pub fn add_triangle(&mut self, i0: u16, i1: u16, i2: u16) {
        self.indices.extend_from_slice(&[i0, i1, i2]);
    }

    pub fn add_quad(&mut self, i0: u16, i1: u16, i2: u16, i3: u16) {
        self.indices.extend_from_slice(&[i0, i1, i2, i0, i2, i3]);
    }
}

pub fn create_box(center: Vec3, size: Vec3) -> Mesh {
    let mut mesh = Mesh::new();
    let half = size * 0.5;

    // Front face (positive Z)
    let v0 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, -half.y, half.z),
        Vec3::Z,
        Vec2::new(0.0, 1.0),
    ));
    let v1 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, -half.y, half.z),
        Vec3::Z,
        Vec2::new(1.0, 1.0),
    ));
    let v2 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, half.y, half.z),
        Vec3::Z,
        Vec2::new(1.0, 0.0),
    ));
    let v3 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, half.y, half.z),
        Vec3::Z,
        Vec2::new(0.0, 0.0),
    ));
    mesh.add_quad(v0, v1, v2, v3);

    // Back face (negative Z)
    let v4 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, -half.y, -half.z),
        -Vec3::Z,
        Vec2::new(0.0, 1.0),
    ));
    let v5 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, -half.y, -half.z),
        -Vec3::Z,
        Vec2::new(1.0, 1.0),
    ));
    let v6 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, half.y, -half.z),
        -Vec3::Z,
        Vec2::new(1.0, 0.0),
    ));
    let v7 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, half.y, -half.z),
        -Vec3::Z,
        Vec2::new(0.0, 0.0),
    ));
    mesh.add_quad(v4, v5, v6, v7);

    // Right face (positive X)
    let v8 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, -half.y, half.z),
        Vec3::X,
        Vec2::new(0.0, 1.0),
    ));
    let v9 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, -half.y, -half.z),
        Vec3::X,
        Vec2::new(1.0, 1.0),
    ));
    let v10 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, half.y, -half.z),
        Vec3::X,
        Vec2::new(1.0, 0.0),
    ));
    let v11 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, half.y, half.z),
        Vec3::X,
        Vec2::new(0.0, 0.0),
    ));
    mesh.add_quad(v8, v9, v10, v11);

    // Left face (negative X)
    let v12 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, -half.y, -half.z),
        -Vec3::X,
        Vec2::new(0.0, 1.0),
    ));
    let v13 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, -half.y, half.z),
        -Vec3::X,
        Vec2::new(1.0, 1.0),
    ));
    let v14 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, half.y, half.z),
        -Vec3::X,
        Vec2::new(1.0, 0.0),
    ));
    let v15 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, half.y, -half.z),
        -Vec3::X,
        Vec2::new(0.0, 0.0),
    ));
    mesh.add_quad(v12, v13, v14, v15);

    // Top face (positive Y)
    let v16 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, half.y, half.z),
        Vec3::Y,
        Vec2::new(0.0, 0.0),
    ));
    let v17 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, half.y, half.z),
        Vec3::Y,
        Vec2::new(1.0, 0.0),
    ));
    let v18 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, half.y, -half.z),
        Vec3::Y,
        Vec2::new(1.0, 1.0),
    ));
    let v19 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, half.y, -half.z),
        Vec3::Y,
        Vec2::new(0.0, 1.0),
    ));
    mesh.add_quad(v16, v17, v18, v19);

    // Bottom face (negative Y)
    let v20 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, -half.y, -half.z),
        -Vec3::Y,
        Vec2::new(0.0, 0.0),
    ));
    let v21 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, -half.y, -half.z),
        -Vec3::Y,
        Vec2::new(1.0, 0.0),
    ));
    let v22 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(half.x, -half.y, half.z),
        -Vec3::Y,
        Vec2::new(1.0, 1.0),
    ));
    let v23 = mesh.add_vertex(Vertex::new(
        center + Vec3::new(-half.x, -half.y, half.z),
        -Vec3::Y,
        Vec2::new(0.0, 1.0),
    ));
    mesh.add_quad(v20, v21, v22, v23);

    mesh
}

pub fn create_plane(center: Vec3, size: Vec2, normal: Vec3) -> Mesh {
    let mut mesh = Mesh::new();
    
    // Calculate basis vectors for the plane
    let up = if normal.abs_diff_eq(Vec3::Y, 0.01) {
        Vec3::Z
    } else {
        Vec3::Y
    };
    let right = normal.cross(up).normalize();
    let forward = right.cross(normal).normalize();

    let half_width = size.x * 0.5;
    let half_height = size.y * 0.5;

    let v0 = mesh.add_vertex(Vertex::new(
        center - right * half_width - forward * half_height,
        normal,
        Vec2::new(0.0, 0.0),
    ));
    let v1 = mesh.add_vertex(Vertex::new(
        center + right * half_width - forward * half_height,
        normal,
        Vec2::new(1.0, 0.0),
    ));
    let v2 = mesh.add_vertex(Vertex::new(
        center + right * half_width + forward * half_height,
        normal,
        Vec2::new(1.0, 1.0),
    ));
    let v3 = mesh.add_vertex(Vertex::new(
        center - right * half_width + forward * half_height,
        normal,
        Vec2::new(0.0, 1.0),
    ));

    // Reverse winding order for upward-facing planes so they're visible from above
    if normal.y > 0.5 {
        mesh.add_quad(v0, v3, v2, v1);
    } else {
        mesh.add_quad(v0, v1, v2, v3);
    }
    mesh
}

pub fn create_cylinder(center: Vec3, radius: f32, height: f32, segments: u32) -> Mesh {
    let mut mesh = Mesh::new();
    let half_height = height * 0.5;

    // Create vertices for top and bottom circles
    for i in 0..segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        // Bottom vertex
        mesh.add_vertex(Vertex::new(
            center + Vec3::new(x, -half_height, z),
            Vec3::new(x, 0.0, z).normalize(),
            Vec2::new(i as f32 / segments as f32, 0.0),
        ));

        // Top vertex
        mesh.add_vertex(Vertex::new(
            center + Vec3::new(x, half_height, z),
            Vec3::new(x, 0.0, z).normalize(),
            Vec2::new(i as f32 / segments as f32, 1.0),
        ));
    }

    // Create side faces
    for i in 0..segments {
        let next = (i + 1) % segments;
        let bottom_current = i * 2;
        let top_current = bottom_current + 1;
        let bottom_next = next * 2;
        let top_next = bottom_next + 1;

        mesh.add_quad(
            bottom_current as u16,
            bottom_next as u16,
            top_next as u16,
            top_current as u16,
        );
    }

    // Add center vertices for caps
    let bottom_center = mesh.add_vertex(Vertex::new(
        center + Vec3::new(0.0, -half_height, 0.0),
        -Vec3::Y,
        Vec2::new(0.5, 0.5),
    ));
    let top_center = mesh.add_vertex(Vertex::new(
        center + Vec3::new(0.0, half_height, 0.0),
        Vec3::Y,
        Vec2::new(0.5, 0.5),
    ));

    // Create cap faces
    for i in 0..segments {
        let next = (i + 1) % segments;
        let bottom_current = i * 2;
        let top_current = bottom_current + 1;
        let bottom_next = next * 2;
        let top_next = bottom_next + 1;

        // Bottom cap
        mesh.add_triangle(
            bottom_center,
            bottom_next as u16,
            bottom_current as u16,
        );

        // Top cap
        mesh.add_triangle(
            top_center,
            top_current as u16,
            top_next as u16,
        );
    }

    mesh
}