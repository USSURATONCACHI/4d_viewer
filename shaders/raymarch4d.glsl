// Functions
float sdBox(vec4 pos);
float sdSphere(vec4 pos);
float sdBoxFrame(vec4 pos, float thickness);



// Implementations
float sdBox(vec4 pos) {
  vec4 q = abs(pos) - vec4(0.5);
  return length(max(q,0.0)) + min(max4(q.x,q.y,q.z, q.w),0.0);
}

float sdSphere(vec4 pos) {
  return length(pos) - 0.5;
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