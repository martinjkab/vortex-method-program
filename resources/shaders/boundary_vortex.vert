#version 460 core
layout (location = 0) in vec3 pos;

uniform mat4 viewProjectionMatrix;

struct Vortex{
    vec4 position;
    vec4 normal;
    vec4 vorticity;
    vec4 lifetime;
};

layout(std430, binding=10) buffer vorticies_data{
    Vortex vorticies[];
};

struct BoundaryInfo{
    uint current_index;
    uint current_count;
    uint count;
};

layout(std430, binding=50) buffer boundary_info_data{
    BoundaryInfo info;
};


flat out uint index;

mat4 getRotationMatrix(vec3 a, vec3 b);

void main() {
    uint idx = gl_InstanceID + info.current_index * info.current_count;
    mat4 translationMatrix = mat4(1.0f);
    translationMatrix[3] = vorticies[idx].position;
    mat4 scaleMatrix = mat4(1.0f);
    float vorticityLength = length(vorticies[idx].vorticity.xyz);
    scaleMatrix[0][0] = vorticityLength;
    scaleMatrix[1][1] = vorticityLength;
    scaleMatrix[2][2] = vorticityLength;
    scaleMatrix[0][0] = 0.2f;
    scaleMatrix[1][1] = 0.2f;
    scaleMatrix[2][2] = 0.2f;
    mat4 rotationMatrix = getRotationMatrix(vec3(0.0f, 1.0f, 0.0f), normalize(vorticies[gl_InstanceID].vorticity.xyz));
    mat4 modelMatrix = translationMatrix * scaleMatrix * rotationMatrix;
    vec4 ndcPos = vec4(pos, 1) * transpose(modelMatrix) * viewProjectionMatrix;
	gl_Position = ndcPos;
    index = idx;
}

mat4 getRotationMatrix(vec3 a, vec3 b){
    mat3 I = mat3(1.0f);
    vec3 v = cross(a, b);
    if(length(v) < 0.00001f) return mat4(I);
    float s = length(v);
    float c = dot(a, b);
    mat3 vx = mat3(0.0f, v.z, -v.y, -v.z, 0.0f, v.x, v.y, -v.x, 0.0f);
    mat3 result = I + vx + vx * vx * ((1.0f - c) / (s * s));
    return mat4(result);
}