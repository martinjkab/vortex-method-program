#version 460 core
out vec4 final_color;

uniform vec4 color;

void main() {
  vec4 new_color = color;
  float opacity = 1.0f;
  final_color = vec4(new_color.xyz, opacity);
}