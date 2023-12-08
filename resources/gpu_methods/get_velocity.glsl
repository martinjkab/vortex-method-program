// vec3 get_velocity(vec4 a, Vortex b){
//     vec3 diff = (b.position - a).xyz;
//     float distance = length(diff);
//     if(distance < 0.0001f){
//         return vec3(0.0f, 0.0f, 0.0f);
//     }
//     if(dot(b.vorticity.xyz, b.vorticity.xyz) < 0.0001f){
//         return vec3(0.0f, 0.0f, 0.0f);
//     }
//     vec3 pw = dot(diff.xyz, b.vorticity.xyz) * b.vorticity.xyz;
//     diff = diff - pw / dot(b.vorticity.xyz, b.vorticity.xyz);
//     distance = length(diff);
//     if(distance < 0.0001f){
//         return vec3(0.0f, 0.0f, 0.0f);
//     }
//     return cross(b.vorticity.xyz, diff / distance);
// }

vec3 get_velocity(vec4 a, Vortex b){
    if(length(b.vorticity.xyz) < 0.0001f) return vec3(0.0f, 0.0f, 0.0f);
    vec3 diff = (b.position - a).xyz;
    float distance = length(diff);
    if(distance < 0.0001f) return vec3(0.0f, 0.0f, 0.0f);
    return cross(b.vorticity.xyz, diff / (distance * distance * distance));
}