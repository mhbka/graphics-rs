#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 direction;
    float cutOffCos;
    float constant;
    float linear;
    float quadratic;
};

in vec2 texCoords;
in vec3 fragPos;
in vec3 fragNormal;

uniform Material material;
uniform Light light;
uniform vec3 lightColor;

out vec4 FragColor;

void main()
{
    // calculate light intensity using attenuation
    float dist = length(fragPos - light.position);
    float attenuation = 1.0 / (light.constant + (light.linear * dist) + (light.quadratic * dist * dist));

    // calculate cosine angle between light->frag and light direction
    vec3 fragToLight = fragPos - light.position;
    float theta = dot(normalize(fragToLight), normalize(light.direction));

    // if contained within cutoff, calculate lighting normally
    if (theta > light.cutOffCos) {
        vec3 ambient = attenuation * 0.1 * vec3(texture2D(material.diffuse, texCoords));
        vec3 diffuse = attenuation * 1.0 * vec3(texture2D(material.diffuse, texCoords));
        vec3 specular = attenuation * 2.0 * vec3(texture2D(material.specular, texCoords));

        vec3 ambientLight = ambient * lightColor;

        vec3 lightDir = normalize(light.position - fragPos);
        float diffuseStrength = max(0.0, dot(lightDir, fragNormal));
        vec3 diffuseLight = diffuseStrength * diffuse * lightColor;

        vec3 viewDir = normalize(light.position - fragPos);
        vec3 reflectDir = reflect(-lightDir, fragNormal);
        float specularity = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
        vec3 specularLight = specular * specularity * lightColor;

        FragColor = vec4(ambientLight + diffuseLight + specularLight, 1.0);
    }

    // else just use ambient
    else {
        vec3 ambient = attenuation * 0.1 * vec3(texture2D(material.diffuse, texCoords));
        vec3 ambientLight = ambient * lightColor;
        FragColor = vec4(ambientLight, 1.0);
    } 
}