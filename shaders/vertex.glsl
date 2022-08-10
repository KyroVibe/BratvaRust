#version 330

uniform mat4 transform_mat;
uniform mat4 camera_mat;

in vec2 position;

out vec3 vColor;

void main() {
    vec4 tmp = vec4(position, 0.0, 1.0) * transform_mat;
    // gl_Position = vec4(tmp.x, tmp.y * aspect_ratio, 0.5, 1.0);
    gl_Position = tmp * camera_mat;

    vColor = vec3(1.0, 0.0, 1.0);
}
