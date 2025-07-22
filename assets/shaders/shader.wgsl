struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) world_pos: vec3<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.world_pos = vertex.position;
    out.normal = vertex.normal;
    out.tex_coords = vertex.tex_coords;
    out.clip_position = vec4<f32>(vertex.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let normal = normalize(in.normal);
    
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = diff * vec3<f32>(0.8, 0.8, 0.8);
    
    let ambient = vec3<f32>(0.2, 0.2, 0.2);
    let color = ambient + diffuse;
    
    // Grass color
    let grass_color = vec3<f32>(0.2, 0.6, 0.2);
    return vec4<f32>(color * grass_color, 1.0);
} 