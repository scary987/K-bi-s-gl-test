#version 400 core

in vec2 position;

out vec3 fColour;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    fColour = vec3(position / 2 + vec2(1), 1.0);
}
