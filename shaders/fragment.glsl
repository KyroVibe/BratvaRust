#version 330
in vec3 vColor;
in vec3 vNormal;
out vec4 f_color;
void main() {

    vec3 lightDirection = normalize(vec3(-1.0, -1.0, 1.0));
    float light = -dot(lightDirection, vNormal);

    if (light < 0.05) {
        light = 0.05;
    } else if (light > 1) {
        light = 1.0;
    }

    f_color = vec4(vColor * light, 1.0);
}
