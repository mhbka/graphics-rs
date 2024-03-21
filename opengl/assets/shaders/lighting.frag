#version 330 core

in vec3 fragPos;
in vec3 fragNormal;

uniform vec3 lightPos;
uniform vec3 objectColor;
uniform vec3 lightColor;

out vec4 FragColor;

void main()
{   
    // ambient light
    float ambientStrength = 0.3;
    vec3 ambientLight = ambientStrength * lightColor;

    // diffuse light
    vec3 lightDir = normalize(lightPos - fragPos);
    float diffuseStrength = max(0.0, dot(lightDir, fragNormal));
    vec3 diffuseLight = diffuseStrength * lightColor;

    // WIP: spec light


    // final light
    FragColor = vec4((ambientLight + diffuseLight) * objectColor, 1.0);
}