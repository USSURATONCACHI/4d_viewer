#version 430 core

in vec3 f_pos;
in vec3 f_color;

out vec4 out_color;





#define CUBE 0
#define SPHERE 1

struct vec5 { float data[5]; };  
struct mat5 { float data[25]; };

struct Object {
    uint obj_type;
    mat5 transform;
};

layout(std430, binding = 0) buffer layoutObjects {
    Object objects[];
};

uniform uint objects_count;
uniform float aspect_ratio;





float vec5_mul_mat5_row(vec5 vec, mat5 mat, int row);
vec5 vec5_mul_mat5(vec5 vec, mat5 mat);

#define CAM_WIDTH 0.01

void main() {
    vec3 norm_f_pos = f_pos / 2.0;

    vec4 ray_pos = vec4(0.0, norm_f_pos.x * CAM_WIDTH, norm_f_pos.y * CAM_WIDTH / aspect_ratio, 0.0);
    
    out_color = vec4(norm_f_pos.xy + 0.5, objects[0].obj_type, 1.0);
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