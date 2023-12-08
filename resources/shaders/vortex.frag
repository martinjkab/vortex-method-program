#version 460 core
out vec4 final_color;

in float lifetime;
in float initial_lifetime;

const float PI = 3.1415926535897932384626433832795;

uniform vec4 color;
uniform bool fading_enabled;

void main() {
  float frac = lifetime / initial_lifetime;
  float opacity = 1.0f;
  if(fading_enabled){
    opacity = color.w * cos(PI  * frac-(PI/2.0f));
  }
  final_color = vec4(color.xyz, opacity);
}