#version 430 core

#include_once mat5.glsl

layout (location = 0) in vec4 v_pos;
layout (location = 1) in vec3 v_color;

out vec4 f_pos;
out vec3 f_color;

uniform float u_aspect_ratio;
uniform float u_time;
uniform mat5 u_model;
uniform mat5 u_projection;

void main() {
    vec5 pos5;
    pos5.xyzw = v_pos;
    pos5.v = 1.0;

    vec5 world_pos = vec5_mul_mat5(pos5, u_model);
    world_pos.xyzw /= world_pos.v;
    world_pos.v = 1.0;
    
    vec5 view_pos = vec5_mul_mat5(world_pos, u_projection);
    gl_Position = vec4(view_pos.xyzw.xyz, view_pos.v);

    f_pos = world_pos.xyzw / world_pos.v;
    //f_pos = v_pos;
    f_color = v_color;
}