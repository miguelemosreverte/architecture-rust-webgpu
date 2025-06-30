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
    
    // Different colors for different elements
    let base_color = select(
        vec3<f32>(0.9, 0.9, 0.9),  // Walls - light grey
        vec3<f32>(0.85, 0.82, 0.78),  // Floor - beige
        in.world_normal.y > 0.9
    );
    
    return vec4<f32>(base_color * light, 1.0);
}
"#;