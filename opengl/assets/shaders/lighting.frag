#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct SpotLight {
    vec3 position;
    vec3 direction;
    vec3 color;
    float innerCutOffCos;
    float outerCutOffCos;
    float constant;
    float linear;
    float quadratic;
};

in vec2 texCoords;
in vec3 fragPos;
in vec3 fragNormal;

uniform Material material;
uniform SpotLight spotlight;

out vec4 FragColor;

vec3 calcSpotLight(SpotLight spotlight);

void main()
{
    vec3 outputColor = vec3(0.0);

    outputColor += calcSpotLight(spotlight);

    FragColor = vec4(outputColor, 1.0);
}

vec3 calcSpotLight(SpotLight spotlight) {
    // calculate light intensity using attenuation
    float dist = length(fragPos - spotlight.position);
    float attenuation = 1.0 / (spotlight.constant + (spotlight.linear * dist) + (spotlight.quadratic * dist * dist));

    vec3 ambient = attenuation * 0.5 * vec3(texture2D(material.diffuse, texCoords));
    vec3 diffuse = attenuation * 1.0 * vec3(texture2D(material.diffuse, texCoords));
    vec3 specular = attenuation * 2.0 * vec3(texture2D(material.specular, texCoords));

    // calculate cosine angle between light->frag and light direction
    vec3 fragToLight = fragPos - spotlight.position;
    float theta = dot(normalize(fragToLight), normalize(spotlight.direction));

    // if contained within cutoff, calculate lighting normally
    if (theta > spotlight.outerCutOffCos) {

        // fade out lighting after innerCutOff radius, up to outerCutOff
        float epsilon = spotlight.innerCutOffCos - spotlight.outerCutOffCos;   
        float intensity = smoothstep(0.0, 1.0, (theta - spotlight.outerCutOffCos) / epsilon);
        diffuse *= intensity;
        specular *= intensity;     

        // 3 lighting types
        vec3 ambientLight = ambient * spotlight.color;

        vec3 lightDir = normalize(spotlight.position - fragPos);
        float diffuseStrength = max(0.0, dot(lightDir, fragNormal));
        vec3 diffuseLight = diffuseStrength * diffuse * spotlight.color;

        vec3 viewDir = normalize(spotlight.position - fragPos);
        vec3 reflectDir = reflect(-lightDir, fragNormal);
        float specularity = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
        vec3 specularLight = specular * specularity * spotlight.color;

        return ambientLight + diffuseLight + specularLight;
    }

    // else just use ambient
    else {
        return ambient * spotlight.color;
    } 
}