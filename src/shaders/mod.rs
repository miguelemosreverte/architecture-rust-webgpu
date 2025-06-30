pub const SCENE_SHADER: &str = r#"
struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) world_position: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
    out.world_normal = in.normal;
    out.tex_coords = in.tex_coords;
    out.world_position = in.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple lighting
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let ambient = 0.3;
    let diffuse = max(dot(normalize(in.world_normal), light_dir), 0.0) * 0.7;
    let light = ambient + diffuse;
    
    // Check if this is a floor (normal pointing up AND at ground level)
    let is_floor = in.world_normal.y > 0.9 && abs(in.world_position.y) < 0.1;
    
    var base_color = vec3<f32>(0.9, 0.9, 0.9);  // Default wall color
    
    if (is_floor) {
        // Floor base color
        base_color = vec3<f32>(0.85, 0.82, 0.78);
        
        // Grid parameters
        let grid_size = 1.0; // 1 meter grid
        let line_width = 0.02;
        let major_line_frequency = 5.0; // Major lines every 5 meters
        
        // Calculate grid coordinates
        let grid_x = fract(in.world_position.x / grid_size);
        let grid_z = fract(in.world_position.z / grid_size);
        
        // Check for major grid lines (every 5 meters)
        let is_major_x = fract(in.world_position.x / (grid_size * major_line_frequency)) < (line_width * 2.0 / grid_size);
        let is_major_z = fract(in.world_position.z / (grid_size * major_line_frequency)) < (line_width * 2.0 / grid_size);
        
        // Draw grid lines
        let is_line_x = grid_x < line_width || grid_x > (1.0 - line_width);
        let is_line_z = grid_z < line_width || grid_z > (1.0 - line_width);
        
        // Check for axis lines
        let is_x_axis = abs(in.world_position.z) < line_width * 2.0;
        let is_z_axis = abs(in.world_position.x) < line_width * 2.0;
        
        if (is_x_axis) {
            // X axis - red tint
            base_color = vec3<f32>(0.8, 0.2, 0.2);
        } else if (is_z_axis) {
            // Z axis - blue tint
            base_color = vec3<f32>(0.2, 0.2, 0.8);
        } else if (is_line_x || is_line_z) {
            if (is_major_x || is_major_z) {
                // Major grid lines - darker
                base_color = vec3<f32>(0.4, 0.4, 0.4);
            } else {
                // Minor grid lines - lighter
                base_color = vec3<f32>(0.6, 0.6, 0.6);
            }
        }
        
        // Add subtle checkerboard pattern for better visibility
        let checker = (i32(floor(in.world_position.x)) + i32(floor(in.world_position.z))) & 1;
        if (checker == 0 && !is_line_x && !is_line_z) {
            base_color = base_color * 0.97;
        }
    }
    
    return vec4<f32>(base_color * light, 1.0);
}
"#;