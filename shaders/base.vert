#version 430 core

#define MAX_OBJECTS 15
#define CAM_WIDTH 0.01
#define FOV 90

#define CUBE 0
#define SPHERE 1
#define CUBE3D 2
#define CUBE_FRAME 3

#include_once common.glsl

layout (location = 0) in vec3 v_pos;

out vec3 f_pos;
out vec4 f_ray_poses[MAX_OBJECTS];
out vec4 f_ray_poses1[MAX_OBJECTS];
out vec4 f_ray_poses2[MAX_OBJECTS];
out vec4 f_ray_poses3[MAX_OBJECTS];
out vec4 f_ray_dirs[MAX_OBJECTS];


struct vec5 { 
    vec4 xyzw;
    float v;
};  
struct mat5 { 
    float data[25];
};
struct Object {
    uint obj_type;
    mat5 transform;
    mat5 inverse;
};


layout(std430, binding = 0) buffer buf1 { Object b_objects[]; };
layout(std430, binding = 1) buffer buf2 { mat5 camera_matrix; };


uniform uint u_objects_count;
uniform float u_aspect_ratio;
uniform float u_time;



float vec5_mul_mat5_row(vec5 vec, mat5 mat, int row);
vec5 vec5_mul_mat5(vec5 vec, mat5 mat);
void calculate_objects_rays();

void main() {
    gl_Position = vec4(v_pos, 1.0);
    f_pos = v_pos;
    calculate_objects_rays();
}

#define FOV_COEF tan(FOV / 2.0)

void calculate_objects_rays() {
    vec3 normalized_pos = v_pos / 2.0;

    // ==================================== Camera offset |
    vec4 main_ray_pos = vec4(                         //  v
        0.0                                             - 1.0,
        normalized_pos.x * CAM_WIDTH * u_aspect_ratio   + 0.0, 
        normalized_pos.y * CAM_WIDTH                    + 0.0,
        0.0                                             + 0.0
    );
    vec4 main_ray_dir = vec4(
        1.0, 
        normalized_pos.x * FOV_COEF * u_aspect_ratio, 
        normalized_pos.y * FOV_COEF, 
        0.0
    );
    main_ray_dir /= length(main_ray_dir);

    // In order not to multiply mat5 by vec5 for each object each iteration,
    // we instead multiply inverses of objects (matrices) by camera to get
    // virtual cameras for each object. It can be done only once.
    for (int i = 0; i < u_objects_count; i++) {
        vec5 ray_pos;
        ray_pos.xyzw = main_ray_pos;
        ray_pos.v = 1.0;

        ray_pos = vec5_mul_mat5(ray_pos, b_objects[i].inverse);
        f_ray_poses[i] = ray_pos.xyzw / ray_pos.v;

        
        vec5 ray_dir;
        ray_dir.xyzw = main_ray_dir;
        ray_dir.v = 0.0;

        ray_dir = vec5_mul_mat5(ray_dir, b_objects[i].inverse);
        f_ray_dirs[i] = ray_dir.xyzw;   // / length(ray_dir.xyzw);
    }
}


vec5 vec5_mul_mat5(vec5 vec, mat5 mat) {
    vec5 result;

    result.xyzw.x = vec5_mul_mat5_row(vec, mat, 0);
    result.xyzw.y = vec5_mul_mat5_row(vec, mat, 1);
    result.xyzw.z = vec5_mul_mat5_row(vec, mat, 2);
    result.xyzw.w = vec5_mul_mat5_row(vec, mat, 3);
    result.v      = vec5_mul_mat5_row(vec, mat, 4);

    return result;
}

float vec5_mul_mat5_row(vec5 vec, mat5 mat, int row) {
    vec4 row_xyzw = vec4(
        mat.data[row * 5 + 0],
        mat.data[row * 5 + 1],
        mat.data[row * 5 + 2],
        mat.data[row * 5 + 3]
    );

    return
        dot(vec.xyzw, row_xyzw) + 
        vec.v * mat.data[row * 5 + 4];
}