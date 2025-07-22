#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 tex_coords;

layout(location = 0) out vec3 out_normal;
layout(location = 1) out vec2 out_tex_coords;
layout(location = 2) out vec3 out_world_pos;

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

void main() {
    out_world_pos = vec3(ubo.model * vec4(position, 1.0));
    out_normal = mat3(ubo.model) * normal;
    out_tex_coords = tex_coords;
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(position, 1.0);
} 