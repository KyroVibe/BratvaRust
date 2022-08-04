#version 330

uniform mat4 mat;
uniform mat4 projection_mat;
uniform float aspect_ratio;

in vec3 position;
in vec3 color;
in vec3 normal;

out vec3 vColor;
out vec3 vNormal;

mat4 get_rotation(mat4 m) {
    return mat4(
        m[0][0], m[0][1], m[0][2], 0.0,
        m[1][0], m[1][1], m[1][2], 0.0,
        m[2][0], m[2][1], m[2][2], 0.0,
            0.0,     0.0,     0.0, 1.0
    );
}

void main() {
    vec4 tmp = vec4(position, 1.0) * mat;
    // gl_Position = vec4(tmp.x, tmp.y * aspect_ratio, 0.5, 1.0);
    gl_Position = tmp * projection_mat;

    mat4 rot = get_rotation(mat);

    // if (position.y > 0) {
    //     vColor = vec3(1.0, 0.0, 0.0);
    // } else {
    //     vColor = vec3(1.0, 1.0, 1.0);
    // }

    vColor = vec3(1.0, 0.0, 1.0);
    vNormal = (vec4(normal, 1.0) * rot).xyz;

    // vColor = color;
}
