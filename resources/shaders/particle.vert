#version 460 core
layout (location = 0) in vec4 pos;

uniform mat4 viewProjectionMatrix;
uniform bool fading_enabled;

struct Particle{
    vec4 position;
    vec4 lifetime;
    vec4 velocity;
};

layout(std430, binding=1) buffer particles_data{
    Particle particles[];
};

flat out int index;
flat out float opacity;
out vec2 texCoords;

const float PI = 3.14159265359f;

void main() {
    index = gl_InstanceID;
    Particle particle = particles[gl_InstanceID];
    vec4 worldPos = particle.position;
    vec4 ndcPos = worldPos * viewProjectionMatrix;
    //Rotate based on veloctity
    vec4 velocity = viewProjectionMatrix * particle.velocity;
    float angle = atan(velocity.y, velocity.x);
    vec2 newPos = vec2(cos(angle) * pos.x - sin(angle) * pos.y, sin(angle) * pos.x + cos(angle) * pos.y);
	ndcPos.xy += newPos.xy * 0.1f * 0.25f;
	gl_Position = ndcPos;
    opacity = 1.0f;
    if(fading_enabled){
        float lifetime = particle.lifetime.x;
        float initial_lifetime = particle.lifetime.y;
        float frac = lifetime / initial_lifetime;
        opacity = cos(PI  * frac-(PI/2.0f));
    }

    texCoords = pos.xy * 0.5f + 0.5f;
}