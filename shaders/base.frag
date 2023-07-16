#version 430 core

#define MAX_OBJECTS 15
#define CAM_WIDTH 0.01

in vec3 f_pos;
in vec4 f_ray_poses[MAX_OBJECTS];
in vec4 f_ray_dirs[MAX_OBJECTS];

out vec4 out_color;



#define CUBE 0
#define SPHERE 1
#define CUBE_FRAME 2


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

float max4(float a, float b, float c, float d);
float min4(float a, float b, float c, float d);

// Raymarching functions
float obj_dist(vec4 pos, uint obj_type);
float sdBox(vec4 pos);
float sdSphere(vec4 pos);
float sdBoxFrame(vec4 pos, float thickness);




void main() {
    vec3 norm_f_pos = f_pos / 2.0;
    out_color = vec4(0.0, 0.0, 0.0, 1.0);

    vec4 main_ray_pos = vec4(
        -1.0, 
        norm_f_pos.x * CAM_WIDTH * u_aspect_ratio + 0.0, 
        norm_f_pos.y * CAM_WIDTH + 0.0,
        0.0
    );
    vec4 main_ray_dir = vec4(1.0, norm_f_pos.x * u_aspect_ratio, norm_f_pos.y, 0.0);
    main_ray_dir /= length(main_ray_dir);

    vec4 obj_ray_poses[MAX_OBJECTS];
    vec4 obj_ray_dirs[MAX_OBJECTS];

    for (int i = 0; i < u_objects_count; i++) {
        vec5 ray_pos;
        ray_pos.xyzw = main_ray_pos;
        ray_pos.v = 1.0;

        ray_pos = vec5_mul_mat5(ray_pos, b_objects[i].inverse);
        obj_ray_poses[i] = ray_pos.xyzw / ray_pos.v;

        
        vec5 ray_dir;
        ray_dir.xyzw = main_ray_dir;
        ray_dir.v = 0.0;

        ray_dir = vec5_mul_mat5(ray_dir, b_objects[i].inverse);
        obj_ray_dirs[i] = ray_dir.xyzw / length(ray_dir.xyzw);
    }

    //float total_distance = 0.0;
    for (int i = 0; i < 50; i++) {
        for (int obj = 0; obj < u_objects_count; obj++) {
            float dist = obj_dist(obj_ray_poses[obj], b_objects[obj].obj_type);
            //total_distance += dist;
            
            if (dist <= 0.0) {
                //out_color = vec4(0.0, 1.0 - float(i) / 100.0, 0.0, 1.0);
                //out_color = vec4(float(i) / 2550.0, 2.0 / total_distance * float(u_objects_count) + 0.0, float(i) / 2550.0, 1.0);
                vec4 abspos = abs(obj_ray_poses[obj]);
                float max = max4(
                    abspos.x, 
                    abspos.y, 
                    abspos.z, 
                    abspos.w
                );

                out_color = max == abspos.w ? vec4(1.0, 0.0, 1.0, 1.0) :
                vec4(
                    max == abspos.x ? 1.0 : 0.0,
                    max == abspos.y ? 1.0 : 0.0,
                    max == abspos.z ? 1.0 : 0.0,
                    1.0
                );
                
                break;
            }

            obj_ray_poses[obj] += obj_ray_dirs[obj] * (dist + 0.0005);
        }
    }
}



// Util

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


float obj_dist(vec4 pos, uint obj_type) {
    switch (obj_type) {
        case CUBE: return sdBox(pos);
        case SPHERE: return sdSphere(pos);
        case CUBE_FRAME: return sdBoxFrame(pos, 0.1);

        default: return 0.0 / 0.0;
    }
}

float sdBox(vec4 pos) {
  vec4 q = abs(pos) - vec4(0.5);
  return length(max(q,0.0)) + min(max4(q.x,q.y,q.z, q.w),0.0);
}

float sdSphere(vec4 pos) {
  return length(pos) - 0.5;
}

float max4(float a, float b, float c, float d) {
    return  
    max(a, max(b, max(c, d)));
}

float min4(float a, float b, float c, float d) {
    return  
    min(a, min(b, min(c, d)));
}

float sdBoxFrame(vec4 p, float e)
{
    p = abs(p) - 0.5;
    vec4 q = abs(p+e)-e;
    return min4(
      length(max(vec4(p.x,q.y,q.z,q.w),0.0))+min(max4(p.x,q.y,q.z,q.w),0.0),
      length(max(vec4(q.x,p.y,q.z,q.w),0.0))+min(max4(q.x,p.y,q.z,q.w),0.0),
      length(max(vec4(q.x,q.y,p.z,q.w),0.0))+min(max4(q.x,q.y,p.z,q.w),0.0),
      length(max(vec4(q.x,q.y,q.z,p.w),0.0))+min(max4(q.x,q.y,q.z,p.w),0.0));
}