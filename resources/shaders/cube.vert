#version 460 core
layout (location = 0) in vec4 pos;

uniform mat4 mvp;

void main() {
	gl_Position = pos * mvp;
}