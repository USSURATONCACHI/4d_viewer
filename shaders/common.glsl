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