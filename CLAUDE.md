# WebGPU Rust Architecture Renderer

## Project Overview

A 3D architectural visualization system built with Rust and WebGPU that:
- Parses JSON scene descriptions
- Renders 3D architectural elements (walls, windows, doors, etc.)
- Supports both native and web (WASM) targets
- Provides a composable primitive system for building complex scenes

## Architecture

### Core Components

1. **Renderer** (`src/renderer/`)
   - `mod.rs`: Main renderer struct managing GPU state and surface
   - `pipeline.rs`: Render pipeline builder for shader programs
   - `buffer.rs`: Vertex and index buffer management

2. **Core** (`src/core/`)
   - `state.rs`: GPU state management (device, queue, adapter)
   - `surface.rs`: Window surface configuration and resizing

3. **Shaders** (`assets/shaders/`)
   - `basic.wgsl`: Simple colored triangle shader
   - `textured.wgsl`: Texture mapping shader with camera uniforms

### Architectural Primitives

The system defines a finite set of composable 3D primitives:

1. **Wall**: Basic rectangular wall with configurable dimensions
2. **Window**: Wall opening with frame and glass
3. **Door**: Wall opening with door geometry
4. **Floor**: Horizontal surface
5. **Ceiling**: Horizontal surface with possible fixtures
6. **Stairs**: Step geometry with railings
7. **Column**: Vertical support structure
8. **Roof**: Angled or flat top structure
9. **Furniture**: Basic furniture primitives (table, chair, etc.)
10. **Light**: Point or directional light sources

### JSON Scene Format

```json
{
  "scene": {
    "name": "Simple Room",
    "camera": {
      "position": [5.0, 5.0, 5.0],
      "target": [0.0, 0.0, 0.0],
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
        "walls": [
          {
            "side": "north",
            "features": [
              {
                "type": "window",
                "position": [2.0, 1.5],
                "size": [1.5, 1.0]
              }
            ]
          },
          {
            "side": "south",
            "features": [
              {
                "type": "door",
                "position": [2.0, 0.0],
                "size": [0.8, 2.0]
              }
            ]
          }
        ]
      }
    ]
  }
}
```

## Building and Running

### Native Build
```bash
cargo build --release
cargo run
```

### WASM Build
```bash
# Install wasm-pack if not already installed
cargo install wasm-pack

# Build for web
wasm-pack build --target web --out-dir pkg

# Serve the web app (requires a web server)
python3 -m http.server 8000
```

## Development Workflow

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Implementation Progress

- [x] Basic WebGPU setup with winit
- [x] Triangle rendering with vertex colors
- [ ] 3D primitive geometry generation
- [ ] JSON scene parser
- [ ] Camera system with controls
- [ ] WASM build configuration
- [ ] Web demo page
- [ ] Example scenes (1.json through 10.json)

## Dependencies

- `wgpu`: WebGPU implementation
- `winit`: Window management
- `glam`: Math library for 3D transformations
- `serde`: JSON serialization/deserialization
- `bytemuck`: Safe type casting for GPU buffers
- `image`: Texture loading
- `env_logger`: Logging
- `pollster`: Async runtime for initialization

## Future Enhancements

1. Texture mapping for realistic materials
2. Shadow mapping
3. Scene graph hierarchy
4. Animation support
5. Physics integration
6. VR/AR support
7. Ray tracing on supported hardware