# WebGPU Rust Architecture Renderer

A 3D architectural visualization system built with Rust and WebGPU that can render buildings, rooms, and architectural elements from JSON scene descriptions.

## Features

- 🏗️ **Composable Primitives**: Build complex scenes using simple primitives (walls, doors, windows, furniture, etc.)
- 📄 **JSON Scene Format**: Define scenes declaratively in JSON
- 🎮 **Native & Web Support**: Runs both as a native application and in the browser via WASM
- 🎨 **3D Rendering**: Full 3D geometry with proper normals and texture coordinates
- 📐 **Architectural Elements**: Walls, windows, doors, floors, ceilings, stairs, columns, and furniture

## Quick Start

### Native Build
```bash
# Clone the repository
git clone https://github.com/miguelemosreverte/architecture-rust-webgpu
cd architecture-rust-webgpu

# Build and run
cargo build --release

# Run with default scene (simple room)
cargo run

# Run with specific example by number
cargo run -- 7              # Loads example 7 (multi-level building)
cargo run -- 10             # Loads example 10 (full house)

# Run with specific JSON file
cargo run -- examples/5_two_rooms.json
cargo run -- my_custom_scene.json

# Run with example name (without number prefix)
cargo run -- room_with_furniture
```

### Interactive Controls

While the application is running:

**Camera Movement:**
- `W` - Move forward
- `S` - Move backward
- `A` - Move left
- `D` - Move right
- `Space` - Move up
- `Shift` - Move down
- Mouse movement - Look around (FPS style)
- `P` - Take screenshot (saves to screenshots/ folder with timestamp)

**Scene Selection:**
- Press `1` through `9` for examples 1-9
- Press `0` for example 10

### Examples

The `examples/` directory contains 10 JSON scene files demonstrating progressively complex architectural scenes:

1. `1_single_wall.json` - A simple wall
2. `2_wall_with_window.json` - Wall with a window
3. `3_simple_room.json` - Basic room with four walls
4. `4_room_with_door_window.json` - Room with door and window
5. `5_two_rooms.json` - Two connected rooms
6. `6_room_with_furniture.json` - Furnished living room
7. `7_multi_level.json` - Two-story building with stairs
8. `8_building_with_columns.json` - Building with architectural columns
9. `9_complex_floor_plan.json` - Complex floor plan with multiple rooms
10. `10_full_house.json` - Complete house with multiple rooms and furniture

## Architecture

See [CLAUDE.md](CLAUDE.md) for detailed architecture documentation.

## JSON Scene Format

```json
{
  "name": "Scene Name",
  "camera": {
    "position": [x, y, z],
    "target": [x, y, z],
    "fov": 45.0
  },
  "elements": [
    {
      "type": "room",
      "position": [0.0, 0.0, 0.0],
      "dimensions": {
        "width": 4.0,
        "height": 3.0,
        "depth": 5.0
      },
      "walls": [...]
    }
  ]
}
```

## Future Development

- [ ] WASM build configuration
- [ ] Interactive camera controls
- [ ] Texture mapping
- [ ] Lighting and shadows
- [ ] Scene editor UI

## License

MIT