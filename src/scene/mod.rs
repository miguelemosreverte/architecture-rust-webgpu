pub mod primitives;
pub mod loader;

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub camera: Camera,
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub fov: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Element {
    Room(Room),
    Wall(Wall),
    Floor(Floor),
    Ceiling(Ceiling),
    Stairs(Stairs),
    Column(Column),
    Furniture(Furniture),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub position: Vec3,
    pub dimensions: Dimensions,
    pub walls: Vec<WallSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallSpec {
    pub side: WallSide,
    pub features: Vec<WallFeature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WallSide {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WallFeature {
    Window(WindowFeature),
    Door(DoorFeature),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowFeature {
    pub position: Vec2,
    pub size: Vec2,
    pub sill_height: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoorFeature {
    pub position: Vec2,
    pub size: Vec2,
    pub door_type: DoorType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoorType {
    Single,
    Double,
    Sliding,
    Revolving,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wall {
    pub start: Vec3,
    pub end: Vec3,
    pub height: f32,
    pub thickness: f32,
    pub features: Vec<WallFeature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Floor {
    pub position: Vec3,
    pub dimensions: Vec2,
    pub material: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ceiling {
    pub position: Vec3,
    pub dimensions: Vec2,
    pub height: f32,
    pub material: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stairs {
    pub position: Vec3,
    pub width: f32,
    pub steps: u32,
    pub step_height: f32,
    pub step_depth: f32,
    pub has_railing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub position: Vec3,
    pub radius: f32,
    pub height: f32,
    pub shape: ColumnShape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnShape {
    Round,
    Square,
    Hexagonal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Furniture {
    pub position: Vec3,
    pub rotation: f32,
    pub furniture_type: FurnitureType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FurnitureType {
    Table { width: f32, depth: f32, height: f32 },
    Chair { seat_height: f32 },
    Sofa { width: f32, depth: f32 },
    Bed { width: f32, length: f32 },
    Desk { width: f32, depth: f32 },
    Cabinet { width: f32, depth: f32, height: f32 },
}