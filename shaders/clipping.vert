#version 430 core

layout (location = 0) in vec3 v_pos;
layout (location = 0) in vec3 v_color;

out vec3 f_pos;
out vec3 f_color;

uniform mat4 model;
uniform mat4 projection;

void main() {
    vec4 world_pos = model * vec4(v_pos, 1.0);
    gl_Position = projection * model * vec4(v_pos, 1.0);
    f_pos = world_pos / world_pos.w;
    f_color = v_color;
}