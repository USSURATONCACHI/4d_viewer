#version 430 core

#include_once mat5.glsl
uniform float u_aspect_ratio;
uniform float u_time;
uniform mat5 u_model;
uniform mat5 u_projection;

out vec4 out_color;

in vec4 f_pos;
in vec3 f_color;

void main() {
    out_color = vec4(f_color, 1.0);
}