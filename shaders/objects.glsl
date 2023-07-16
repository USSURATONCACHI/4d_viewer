#include_once mat5.glsl
#include_once raymarch4d.glsl

// Object types
#define CUBE 0
#define SPHERE 1
#define CUBE_FRAME 2

struct Object {
    uint obj_type;
    mat5 transform;
    mat5 inverse;
};

float obj_dist(vec4 pos, uint obj_type) {
    switch (obj_type) {
        case CUBE: return sdBox(pos);
        case SPHERE: return sdSphere(pos);
        case CUBE_FRAME: return sdBoxFrame(pos, 0.1);

        default: return 0.0 / 0.0;
    }
}