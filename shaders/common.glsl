#include_once mat5.glsl
#include_once objects.glsl

// Common things
#define MAX_OBJECTS 15
#define CAM_WIDTH 0.01

// Buffers and uniforms
layout(std430, binding = 0) buffer buf1 { Object b_objects[]; };
layout(std430, binding = 1) buffer buf2 { mat5 camera_matrix; };

uniform uint u_objects_count;
uniform float u_aspect_ratio;
uniform float u_time;