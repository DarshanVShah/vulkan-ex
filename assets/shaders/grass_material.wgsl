#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::utils

struct GrassMaterial {
    color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: GrassMaterial;

@fragment
fn fragment(
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) is_front: u32,
) -> @location(0) vec4<f32> {
    let normal = normalize(world_normal);
    
    // Calculate lighting
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    
    // Ambient lighting
    let ambient = vec3<f32>(0.2, 0.2, 0.2);
    
    // Diffuse lighting
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = diff * vec3<f32>(0.8, 0.8, 0.8);
    
    // Grass color with variation based on UV coordinates
    let grass_variation = sin(uv.x * 50.0) * sin(uv.y * 50.0) * 0.1;
    let grass_color = material.color.rgb + vec3<f32>(grass_variation);
    
    // Final color
    let final_color = (ambient + diffuse) * grass_color;
    
    return vec4<f32>(final_color, 1.0);
} 