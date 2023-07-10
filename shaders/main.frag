#version 330 core

in vec2 f_pos;

out vec4 f_color;

uniform float u_time;
uniform vec2 u_screen_size;

float sdBox(vec3 p, vec3 b);
float sdSphere(vec3 p, float s);

void main() {
    vec3 cam_pos = vec3(cos(u_time) * 2, sin(u_time), 0.0);

    vec3 cube_pos = vec3(0, 0, 2.0);
    vec3 cube = vec3(0.5);

    vec2 frag_pos = (gl_FragCoord.xy - u_screen_size.xy / 2.0) / u_screen_size.y * 2.0;
    vec3 ray_pos = cam_pos +  vec3(frag_pos / 100.0, 0.0);
    vec3 sp = ray_pos;
    vec3 ray_dir = vec3(frag_pos.x, frag_pos.y, 1.0);
    ray_dir /= length(ray_dir);

    int i;
    for (i = 0; i < 100; i++) {
        float dist = sdBox(ray_pos - cube_pos, cube);
        dist = min(dist, sdSphere(ray_pos - vec3(0.0, 0.0, 3.0), 1.0));

        if (dist <= 0.0) {
            f_color = vec4(1.0);
            break;
        }

        ray_pos += ray_dir * (dist + 0.001);
    }
    
    f_color = vec4(vec3(0.0, 1.0 - float(i) / 100.0, 0.0), 1.0);
}

float sdBox(vec3 p, vec3 b) {
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}

float sdSphere(vec3 p, float s) {
  return length(p)-s;
}