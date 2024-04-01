#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform mat4 normTransform;

out vec2 texCoords;

out vec3 fragPos;
out vec3 fragViewPos;
out vec3 fragNormal;


void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);

    texCoords = aTexCoords;

    fragPos = vec3(model * vec4(aPos, 1.0));
    fragViewPos = vec3(view * model * vec4(aPos, 1.0));
    fragNormal = normalize(mat3(normTransform) * aNormal);
}