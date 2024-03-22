#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform vec3 lightPos;
uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform mat4 normTransform;

out vec3 fragPos;
out vec3 fragNormal;
out vec3 fragLightPos;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    fragPos = vec3(view * model * vec4(aPos, 1.0));
    fragNormal = normalize(mat3(normTransform) * aNormal);
    fragLightPos = vec3(view * vec4(lightPos, 1.0));
}