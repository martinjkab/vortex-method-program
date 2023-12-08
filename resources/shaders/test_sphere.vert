#version 460 core
layout (location = 0) in vec4 pos;

uniform mat4 mvp;
uniform mat4 model;

out vec4 worldPos;

void main() {
	gl_Position = pos * mvp;
	worldPos = pos * model;
}