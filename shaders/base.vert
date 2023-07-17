#version 430 core

#include_once common.glsl
#include_once mat5.glsl
#include_once objects.glsl

layout (location = 0) in vec3 v_pos;

#define OUT
out vec4 f_ray_pos;
out vec4 f_ray_dir;


out vec3 f_pos;

void calculate_objects_ray();

void main() {
    gl_Position = vec4(v_pos, 1.0);
    f_pos = v_pos;
    calculate_objects_ray();
}


#define FOV 90.0
#define FOV_COEF tan(FOV / 2.0)

void calculate_objects_ray() {
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
    vec5 ray_pos;
    ray_pos.xyzw = main_ray_pos;
    ray_pos.v = 1.0;

    ray_pos = vec5_mul_mat5(ray_pos, b_camera_matrix);
    ray_pos = vec5_mul_mat5(ray_pos, b_objects[u_object_id].inverse);
    f_ray_pos = ray_pos.xyzw / ray_pos.v;

    vec5 ray_dir;
    ray_dir.xyzw = main_ray_dir;
    ray_dir.v = 0.0;

    ray_dir = vec5_mul_mat5(ray_dir, b_objects[u_object_id].inverse);
    f_ray_dir = ray_dir.xyzw;
}
