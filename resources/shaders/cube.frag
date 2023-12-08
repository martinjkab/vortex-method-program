#version 460 core
out vec4 final_color;

uniform mat4 mvp;
uniform bool hover;

struct Cube{
    vec4 position;
    vec4 velocity;
    vec4 color;
};

layout(std430, binding=3) buffer cube_data{
    Cube cube;
};


void main() {
  if(hover){
    final_color = vec4(1,0,0,1);
    return;
  }
  final_color = vec4(0,1,0,1);
}