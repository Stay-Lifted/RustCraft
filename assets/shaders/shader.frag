// shader.frag
#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec3 normal;
layout(location=2) in vec4 applied_color;

layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    vec4 ambient = vec4(0.5, 0.5, 0.5, 1.0);

    f_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords) * ambient * applied_color;
}