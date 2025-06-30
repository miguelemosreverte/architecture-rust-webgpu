use super::{Scene, Element, Room, WallSide, WallFeature};
use super::primitives::{Mesh, create_box, create_plane};
use glam::Vec3;
use std::fs;
use std::path::Path;

pub fn load_scene_from_file(path: &Path) -> Result<Scene, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let scene: Scene = serde_json::from_str(&contents)?;
    Ok(scene)
}

pub fn scene_to_mesh(scene: &Scene) -> Mesh {
    let mut combined_mesh = Mesh::new();
    let mut vertex_offset = 0u16;

    // First, add a large base floor plane
    let base_floor_size = 50.0; // 50x50 meter floor
    let base_floor = create_plane(
        Vec3::new(0.0, -0.01, 0.0), // Slightly below ground level to avoid z-fighting
        glam::Vec2::new(base_floor_size, base_floor_size),
        Vec3::Y,
    );
    
    // Add base floor vertices
    for vertex in &base_floor.vertices {
        combined_mesh.vertices.push(*vertex);
    }
    for &index in &base_floor.indices {
        combined_mesh.indices.push(index);
    }
    vertex_offset += base_floor.vertices.len() as u16;

    // Then add all scene elements
    for element in &scene.elements {
        let element_mesh = element_to_mesh(element);
        
        // Add vertices with offset
        for vertex in &element_mesh.vertices {
            combined_mesh.vertices.push(*vertex);
        }

        // Add indices with offset
        for &index in &element_mesh.indices {
            combined_mesh.indices.push(index + vertex_offset);
        }

        vertex_offset += element_mesh.vertices.len() as u16;
    }

    combined_mesh
}

fn element_to_mesh(element: &Element) -> Mesh {
    match element {
        Element::Room(room) => room_to_mesh(room),
        Element::Wall(wall) => {
            create_box(
                Vec3::new(
                    (wall.start.x + wall.end.x) * 0.5,
                    wall.height * 0.5,
                    (wall.start.z + wall.end.z) * 0.5,
                ),
                Vec3::new(
                    (wall.end - wall.start).length(),
                    wall.height,
                    wall.thickness,
                ),
            )
        }
        Element::Floor(floor) => {
            create_plane(
                floor.position,
                floor.dimensions,
                Vec3::Y,
            )
        }
        Element::Ceiling(ceiling) => {
            create_plane(
                Vec3::new(ceiling.position.x, ceiling.height, ceiling.position.z),
                ceiling.dimensions,
                -Vec3::Y,
            )
        }
        Element::Column(column) => {
            use super::ColumnShape;
            match column.shape {
                ColumnShape::Round => {
                    use super::primitives::create_cylinder;
                    create_cylinder(column.position, column.radius, column.height, 16)
                }
                ColumnShape::Square | ColumnShape::Hexagonal => {
                    create_box(
                        column.position + Vec3::new(0.0, column.height * 0.5, 0.0),
                        Vec3::new(column.radius * 2.0, column.height, column.radius * 2.0),
                    )
                }
            }
        }
        Element::Stairs(stairs) => {
            let mut mesh = Mesh::new();
            let step_rise = stairs.step_height;
            let step_run = stairs.step_depth;
            
            for i in 0..stairs.steps {
                let step_mesh = create_box(
                    stairs.position + Vec3::new(
                        0.0,
                        (i as f32 + 0.5) * step_rise,
                        (i as f32 + 0.5) * step_run,
                    ),
                    Vec3::new(stairs.width, step_rise, step_run),
                );
                
                let offset = mesh.vertices.len() as u16;
                mesh.vertices.extend_from_slice(&step_mesh.vertices);
                for &index in &step_mesh.indices {
                    mesh.indices.push(index + offset);
                }
            }
            mesh
        }
        Element::Furniture(furniture) => {
            use super::FurnitureType;
            match &furniture.furniture_type {
                FurnitureType::Table { width, depth, height } => {
                    let mut mesh = Mesh::new();
                    
                    // Table top
                    let top_mesh = create_box(
                        furniture.position + Vec3::new(0.0, *height - 0.05, 0.0),
                        Vec3::new(*width, 0.1, *depth),
                    );
                    mesh.vertices.extend_from_slice(&top_mesh.vertices);
                    mesh.indices.extend_from_slice(&top_mesh.indices);
                    
                    // Table legs
                    let leg_positions = [
                        Vec3::new(-width * 0.4, *height * 0.5, -depth * 0.4),
                        Vec3::new(width * 0.4, *height * 0.5, -depth * 0.4),
                        Vec3::new(-width * 0.4, *height * 0.5, depth * 0.4),
                        Vec3::new(width * 0.4, *height * 0.5, depth * 0.4),
                    ];
                    
                    for leg_offset in &leg_positions {
                        let offset = mesh.vertices.len() as u16;
                        let leg_mesh = create_box(
                            furniture.position + leg_offset,
                            Vec3::new(0.05, *height - 0.1, 0.05),
                        );
                        mesh.vertices.extend_from_slice(&leg_mesh.vertices);
                        for &index in &leg_mesh.indices {
                            mesh.indices.push(index + offset);
                        }
                    }
                    
                    mesh
                }
                _ => create_box(furniture.position, Vec3::new(1.0, 1.0, 1.0)),
            }
        }
    }
}

fn room_to_mesh(room: &Room) -> Mesh {
    let mut mesh = Mesh::new();
    
    // Skip creating individual room floors - we have a base floor now
    
    // Create walls
    for wall_spec in &room.walls {
        let wall_mesh = create_room_wall(room, &wall_spec.side, &wall_spec.features);
        let offset = mesh.vertices.len() as u16;
        mesh.vertices.extend_from_slice(&wall_mesh.vertices);
        for &index in &wall_mesh.indices {
            mesh.indices.push(index + offset);
        }
    }
    
    mesh
}

fn create_room_wall(room: &Room, side: &WallSide, _features: &[WallFeature]) -> Mesh {
    let wall_thickness = 0.2;
    let (start, end) = match side {
        WallSide::North => (
            room.position + Vec3::new(-room.dimensions.width * 0.5, 0.0, room.dimensions.depth * 0.5),
            room.position + Vec3::new(room.dimensions.width * 0.5, 0.0, room.dimensions.depth * 0.5),
        ),
        WallSide::South => (
            room.position + Vec3::new(room.dimensions.width * 0.5, 0.0, -room.dimensions.depth * 0.5),
            room.position + Vec3::new(-room.dimensions.width * 0.5, 0.0, -room.dimensions.depth * 0.5),
        ),
        WallSide::East => (
            room.position + Vec3::new(room.dimensions.width * 0.5, 0.0, room.dimensions.depth * 0.5),
            room.position + Vec3::new(room.dimensions.width * 0.5, 0.0, -room.dimensions.depth * 0.5),
        ),
        WallSide::West => (
            room.position + Vec3::new(-room.dimensions.width * 0.5, 0.0, -room.dimensions.depth * 0.5),
            room.position + Vec3::new(-room.dimensions.width * 0.5, 0.0, room.dimensions.depth * 0.5),
        ),
    };
    
    // Calculate wall center and dimensions
    let wall_center = (start + end) * 0.5 + Vec3::new(0.0, room.dimensions.height * 0.5, 0.0);
    let wall_length = (end - start).length();
    
    // Create wall with proper orientation
    match side {
        WallSide::North | WallSide::South => {
            // Walls running east-west
            create_box(
                wall_center,
                Vec3::new(wall_length, room.dimensions.height, wall_thickness),
            )
        }
        WallSide::East | WallSide::West => {
            // Walls running north-south (need different orientation)
            create_box(
                wall_center,
                Vec3::new(wall_thickness, room.dimensions.height, wall_length),
            )
        }
    }
}