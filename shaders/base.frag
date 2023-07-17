#version 430 core

#include_once common.glsl
#include_once mat5.glsl
#include_once objects.glsl
#include_once raymarch4d.glsl

in vec4 f_ray_pos;
in vec4 f_ray_dir;
vec4 ray_pos;

in vec3 f_pos;
in vec4 f_ray_poses[MAX_OBJECTS];
in vec4 f_ray_dirs[MAX_OBJECTS];

out vec4 out_color;

vec4 obj_ray_poses[60];

void copy_ray_poses();

void main() {
    ray_pos = f_ray_pos;
    vec3 norm_f_pos = f_pos / 2.0;

    //out_color = vec4(0.0, 0.0, 0.0, 1.0);

    float total_distance = 0.0;
    int i;
    for (i = 0; i < 100; i++) {
        float dist = obj_dist(ray_pos, b_objects[u_object_id].obj_type);
        vec5 a;
        a.xyzw.x = float(i);
        mat5 b;
        
        
        if (dist <= 0.0) {
            gl_FragDepth = 1.0 / total_distance;
            vec4 abspos = abs(ray_pos);
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
            
            i = -1;
            break;
        }

        ray_pos += f_ray_dir / length(f_ray_dir) * (dist + 0.0005);
        total_distance += (dist + 0.0005) / length(f_ray_dir);
    }

    if (i > 0) {
        gl_FragDepth = 0.0;
        discard;
    }
}