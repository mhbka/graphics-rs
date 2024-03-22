#version 330 core

in vec3 fragPos;
in vec3 fragNormal;
in vec3 viewerPos;

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

    // specular light
    float specularStrength = 0.5;
    vec3 viewDir = normalize(viewerPos - fragPos);
    vec3 reflectDir = reflect(-lightDir, fragNormal);
    float specularity = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specularLight = specularStrength * specularity * lightColor;

    // final light
    FragColor = vec4((ambientLight + diffuseLight + specularLight) * objectColor, 1.0);
}