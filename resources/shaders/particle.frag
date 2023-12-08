#version 460 core
out vec4 final_color;

// uniform vec4 color;
uniform sampler2D texture0;

flat in int index;
flat in float opacity;
in vec2 texCoords;

void main() {
  vec4 color = texture(texture0, texCoords);
  final_color = vec4(color.xyz, color.w * opacity * 0.25f);
}