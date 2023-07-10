#version 330 core

layout (location = 0) in vec3 v_pos;

out vec3 f_pos;

void main() {
    gl_Position = vec4(v_pos, 1.0);
    f_pos = v_pos;
}