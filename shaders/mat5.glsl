
// Structs
struct vec5 { 
    vec4 xyzw;
    float v;
};  
struct mat5 { 
    float data[25];
};



// Functions
float vec5_mul_mat5_row(vec5 vec, mat5 mat, int row);
vec5 vec5_mul_mat5(vec5 vec, mat5 mat);

float max4(float a, float b, float c, float d);
float min4(float a, float b, float c, float d);



// Implementation
float max4(float a, float b, float c, float d) {
    return max(a, max(b, max(c, d)));
}

float min4(float a, float b, float c, float d) {
    return min(a, min(b, min(c, d)));
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