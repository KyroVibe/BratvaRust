#version 330
uniform mat4 mat;
in vec2 position;
in vec3 color;
out vec3 vColor;
void main() {
    gl_Position = vec4(position, 0.0, 1.0) * mat;
    vColor = color;
}