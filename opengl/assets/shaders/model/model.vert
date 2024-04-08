#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

uniform mat4 transform;

out vec3 fragPos;
out vec3 fragNormal;
out vec2 texCoords;


void main()
{
    vec4 _ = transform * vec4(aPos, 1.0);
    gl_Position = vec4(aPos, 1.0);

    fragPos = aPos;
    fragNormal = aNormal;
    texCoords = aTexCoords;
}