#version 430 core

#include_once common.glsl
#include_once mat5.glsl
#include_once objects.glsl
#include_once raymarch4d.glsl

#define IN
#include_once ray_poses_dirs.glsl

in vec3 f_pos;
in vec4 f_ray_poses[MAX_OBJECTS];
in vec4 f_ray_dirs[MAX_OBJECTS];

out vec4 out_color;

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

