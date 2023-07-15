#version 430 core

in vec3 f_pos;

out vec4 out_color;





#define CUBE 0
#define SPHERE 1

struct vec5 { float data[5]; };  
struct mat5 { float data[25]; };

struct Object {
    uint obj_type;
    mat5 transform;
    mat5 inverse;
};

layout(std430, binding = 0) buffer layoutObjects {
    Object b_objects[];
};

uniform uint u_objects_count;
uniform float u_aspect_ratio;
uniform float u_time;




float vec5_mul_mat5_row(vec5 vec, mat5 mat, int row);
vec5 vec5_mul_mat5(vec5 vec, mat5 mat);

float max4(float a, float b, float c, float d);

// Raymarching functions
float obj_dist(vec4 pos, uint obj_type);
float sdBox(vec4 pos);
float sdSphere(vec4 pos);

#define MAX_OBJECTS 16
#define CAM_WIDTH 0.01

void main() {
    vec3 norm_f_pos = f_pos / 2.0;
    out_color = vec4(0.0, 0.0, 0.0, 1.0);

    vec4 main_ray_pos = vec4(0.0, norm_f_pos.x * CAM_WIDTH, norm_f_pos.y * CAM_WIDTH / u_aspect_ratio, 0.0);
    vec4 main_ray_dir = vec4(1.0, norm_f_pos.x / 10.0, norm_f_pos.y / 10.0, 0.0);
    main_ray_dir /= length(main_ray_dir);

    vec4 obj_ray_poses[MAX_OBJECTS];
    vec4 obj_ray_dirs[MAX_OBJECTS];

    for (int i = 0; i < u_objects_count; i++) {
        vec5 ray_pos;
        ray_pos.data[0] = main_ray_pos.x;
        ray_pos.data[1] = main_ray_pos.y;
        ray_pos.data[2] = main_ray_pos.z;
        ray_pos.data[3] = main_ray_pos.w;
        ray_pos.data[4] = 1.0;

        ray_pos = vec5_mul_mat5(ray_pos, b_objects[i].inverse);
        obj_ray_poses[i] = vec4(
            ray_pos.data[0],
            ray_pos.data[1],
            ray_pos.data[2],
            ray_pos.data[3]
        );
        obj_ray_poses[i] /= ray_pos.data[4];

        
        vec5 ray_dir;
        ray_dir.data[0] = main_ray_dir.x;
        ray_dir.data[1] = main_ray_dir.y;
        ray_dir.data[2] = main_ray_dir.z;
        ray_dir.data[3] = main_ray_dir.w;
        ray_dir.data[4] = 0.0;

        ray_dir = vec5_mul_mat5(ray_dir, b_objects[i].inverse);
        obj_ray_dirs[i] = vec4(
            ray_dir.data[0],
            ray_dir.data[1],
            ray_dir.data[2],
            ray_dir.data[3]
        );
        
        obj_ray_dirs[i] /= length(obj_ray_dirs[i]);
    }

    for (int i = 0; i < 20; i++) {
        for (int obj = 0; obj < u_objects_count; obj++) {
            float dist = obj_dist(obj_ray_poses[obj], b_objects[obj].obj_type);
            
            if (dist <= 0.0) {
                out_color = vec4(1.0);
                break;
            }

            obj_ray_poses[obj] += obj_ray_dirs[obj] * (dist + 0.00001);
        }
    }
}



// Util

vec5 vec5_mul_mat5(vec5 vec, mat5 mat) {
    vec5 result;

    result.data[0] = vec5_mul_mat5_row(vec, mat, 0);
    result.data[1] = vec5_mul_mat5_row(vec, mat, 1);
    result.data[2] = vec5_mul_mat5_row(vec, mat, 2);
    result.data[3] = vec5_mul_mat5_row(vec, mat, 3);
    result.data[4] = vec5_mul_mat5_row(vec, mat, 4);

    return result;
}

float vec5_mul_mat5_row(vec5 vec, mat5 mat, int row) {
    return
        vec.data[0] * mat.data[row * 5 + 0] + 
        vec.data[1] * mat.data[row * 5 + 1] + 
        vec.data[2] * mat.data[row * 5 + 2] + 
        vec.data[3] * mat.data[row * 5 + 3] + 
        vec.data[4] * mat.data[row * 5 + 4];
}


float obj_dist(vec4 pos, uint obj_type) {
    switch (obj_type) {
        case CUBE: return sdBox(pos);
        case SPHERE: return sdSphere(pos);

        default: return 0.0 / 0.0;
    }
}

float sdBox(vec4 pos) {
  vec4 q = abs(pos) - vec4(1.0);
  return length(max(q,0.0)) + min(max4(q.x,q.y,q.z, q.w),0.0);
}

float sdSphere(vec4 pos) {
  return length(pos) - 1.0;
}

float max4(float a, float b, float c, float d) {
    return  
    max(a, max(b, max(c, d)));
}