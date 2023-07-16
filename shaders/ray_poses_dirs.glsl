#ifdef IN
    #define IN_OR_OUT in
#endif

#ifdef OUT
    #define IN_OR_OUT 
#endif

#define MAX_ARRAY_SIZE 15

// OpenGL does not allow for vec4 arrays more that 15 in size.
// But allows for however many arrays of size 15.
IN_OR_OUT vec4 ray_poses_0[MAX_ARRAY_SIZE];
//IN_OR_OUT vec4 ray_poses_1[MAX_ARRAY_SIZE];
//IN_OR_OUT vec4 ray_poses_2[MAX_ARRAY_SIZE];
//IN_OR_OUT vec4 ray_poses_3[MAX_ARRAY_SIZE];
 
IN_OR_OUT vec4 ray_dirs_0[MAX_ARRAY_SIZE];
//IN_OR_OUT vec4 ray_dirs_1[MAX_ARRAY_SIZE];
//IN_OR_OUT vec4 ray_dirs_2[MAX_ARRAY_SIZE];
//IN_OR_OUT vec4 ray_dirs_3[MAX_ARRAY_SIZE];


vec4 get_ray_pos(int id) {
    int local_id = id % MAX_ARRAY_SIZE;
    switch (id / MAX_ARRAY_SIZE) {
        case 0: return ray_poses_0[local_id];
        //case 1: return ray_poses_1[local_id];
        //case 2: return ray_poses_2[local_id];
        //case 3: return ray_poses_3[local_id];
    }
}

vec4 get_ray_dir(int id) {
    int local_id = id % MAX_ARRAY_SIZE;
    switch (id / MAX_ARRAY_SIZE) {
        case 0: return ray_dirs_0[local_id];
        //case 1: return ray_dirs_1[local_id];
        //case 2: return ray_dirs_2[local_id];
        //case 3: return ray_dirs_3[local_id];
    }
}

#ifdef OUT
    void set_ray_pos(int id, vec4 val) {
        int local_id = id % MAX_ARRAY_SIZE;
        switch (id / MAX_ARRAY_SIZE) {
            case 0: ray_poses_0[local_id] = val; break;
            //case 1: ray_poses_1[local_id] = val; break;
            //case 2: ray_poses_2[local_id] = val; break;
            //case 3: ray_poses_3[local_id] = val; break;
        }
    }

    void set_ray_dir(int id, vec4 val) {
        int local_id = id % MAX_ARRAY_SIZE;
        switch (id / MAX_ARRAY_SIZE) {
            case 0: ray_dirs_0[local_id] = val; break;
            //case 1: ray_dirs_1[local_id] = val; break;
            //case 2: ray_dirs_2[local_id] = val; break;
            //case 3: ray_dirs_3[local_id] = val; break;
        }
    }
#endif