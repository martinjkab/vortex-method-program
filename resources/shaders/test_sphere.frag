#version 460 core
out vec4 final_color;


in vec4 worldPos;

struct Vortex{
    vec4 position;
    vec4 normal;
    vec4 vorticity;
    vec4 lifetime;
};

layout(std430, binding=2) buffer vorticies_data{
    Vortex vorticies[];
};

layout(std430, binding=10) buffer boundary_vorticies_data{
    Vortex boundary_vorticies[];
};

struct BoundaryInfo{
    uint current_index;
    uint current_count;
    uint count;
};

layout(std430, binding=50) buffer boundary_info_data{
    BoundaryInfo info;
};

vec3 get_velocity(vec4 a, Vortex b);

void main() {
  vec4 pos = worldPos;
  vec3 velocity = vec3(0.0, 0.0, 0.0);
  uint offset = info.current_index * info.current_count;
  for (int i = 0; i < vorticies.length(); i++) {
    velocity += get_velocity(pos, vorticies[i]);
  }
  for (uint i = offset; i < info.current_count; i++) {
    velocity += get_velocity(pos, boundary_vorticies[i]);
  }
  float velLength = length(velocity);
  vec3 color = velLength * vec3(1, 1, 1) * 0.5f;
  final_color = vec4(color, 1);
}

$get_velocity