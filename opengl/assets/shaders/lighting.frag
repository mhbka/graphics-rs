#version 330 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

in vec3 fragPos;
in vec3 fragNormal;
in vec3 fragLightPos;

uniform Material material;
// uniform vec3 objectColor;
uniform vec3 lightColor;

out vec4 FragColor;

void main()
{
    // weight the material
    vec3 ambient = vec3(0.1) * material.ambient;
    vec3 diffuse = vec3(0.4) * material.diffuse;
    vec3 specular = vec3(1.0) * material.specular;

    // ambient light
    vec3 ambientLight = ambient * lightColor;

    // diffuse light
    vec3 lightDir = normalize(fragLightPos - fragPos);
    float diffuseStrength = max(0.0, dot(lightDir, fragNormal));
    vec3 diffuseLight = diffuseStrength * diffuse * lightColor;

    // specular light
    vec3 viewDir = normalize(-fragPos);
    vec3 reflectDir = reflect(-lightDir, fragNormal);
    float specularity = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specularLight = specular * specularity * lightColor;

    // final light
    FragColor = vec4((ambientLight + diffuseLight + specularLight), 1.0);
}