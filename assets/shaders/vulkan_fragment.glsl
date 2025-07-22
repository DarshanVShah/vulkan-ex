#version 450

layout(location = 0) in vec3 in_normal;
layout(location = 1) in vec2 in_tex_coords;
layout(location = 2) in vec3 in_world_pos;

layout(location = 0) out vec4 out_color;

void main() {
    vec3 light_dir = normalize(vec3(1.0, 1.0, 1.0));
    vec3 normal = normalize(in_normal);
    
    float diff = max(dot(normal, light_dir), 0.0);
    vec3 diffuse = diff * vec3(0.8, 0.8, 0.8);
    
    vec3 ambient = vec3(0.2, 0.2, 0.2);
    vec3 color = ambient + diffuse;
    
    // Grass color
    vec3 grass_color = vec3(0.2, 0.6, 0.2);
    out_color = vec4(color * grass_color, 1.0);
} 