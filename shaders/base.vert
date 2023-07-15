#version 330 core

layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec3 v_color;

out vec3 f_pos;
out vec3 f_color;

void main() {
    gl_Position = vec4(v_pos, 1.0);
    f_pos = v_pos;
    f_color = v_color;
}