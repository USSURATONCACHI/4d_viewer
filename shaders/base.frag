#version 330 core

in vec3 f_pos;
in vec3 f_color;

out vec4 out_color;

void main() {
    vec3 c = f_pos / 2.0 + 0.5;
    out_color = vec4(c.xy, 0.0, 1.0);
}