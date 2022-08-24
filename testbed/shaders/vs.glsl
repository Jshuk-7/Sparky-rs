#version 460 core

#extension GL_NV_shader_buffer_load : enable

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec4 fColor;

vec3 increment(vec3 pos, float amount);

void main() {
    vec3 pos = increment(aPos, 0.25);
    gl_Position = vec4(pos, 1.0);
    fColor = vec4(aColor, 1.0);
}

vec3 increment(vec3 pos, float amount) {
    return pos - vec3(amount);
}