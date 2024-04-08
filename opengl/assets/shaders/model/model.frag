#version 330 core

struct Material {
    sampler2D diffuse1;
    sampler2D specular1;
};

/*
struct DirectionalLight {
    vec3 direction;
    vec3 color;
};

struct PointLight {
    vec3 position;
    vec3 color;
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
*/

in vec2 texCoords;
in vec3 fragPos;
in vec3 fragNormal;

uniform Material material;

/*
uniform DirectionalLight dirlight;
uniform SpotLight spotlight;

#define NR_POINTLIGHTS 5
uniform PointLight pointlights[NR_POINTLIGHTS];
*/

out vec4 FragColor;

/*
vec3 calcDirLight(DirectionalLight light);
vec3 calcPointLight(PointLight light);
vec3 calcSpotLight(SpotLight light);
*/

void main()
{
    vec3 outputColor = vec3(texture2D(material.diffuse1, texCoords));

    vec3 nothing = vec3(texture2D(material.specular1, texCoords));

    FragColor = vec4(outputColor, 1.0);
}

/*
vec3 calcDirLight(DirectionalLight light) 
{
    const float AMB_W = 0.0;
    const float DIF_W = 0.5;
    const float SPC_W = 1.0;

    vec3 ambient = AMB_W * vec3(texture2D(material.diffuse, texCoords));
    vec3 diffuse = DIF_W * vec3(texture2D(material.diffuse, texCoords));
    vec3 specular = SPC_W * vec3(texture2D(material.specular, texCoords));

    vec3 lightDir = normalize(light.direction);

    vec3 ambientLight = ambient * light.color;

    float diffuseStrength = max(0.0, dot(lightDir, fragNormal));
    vec3 diffuseLight = diffuseStrength * diffuse * light.color;

    vec3 viewDir = normalize(spotlight.position - fragPos); // just a hack because i didn't pass in any viewPos, assume spotlights[0] is camera
    vec3 reflectDir = reflect(-lightDir, fragNormal);
    float specularity = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specularLight = specular * specularity * light.color;

    return ambientLight + diffuseLight + specularLight;
}

vec3 calcPointLight(PointLight light) {
    const float AMB_W = 0.0;
    const float DIF_W = 0.5;
    const float SPC_W = 1.0;

    vec3 ambient = AMB_W * vec3(texture2D(material.diffuse, texCoords));
    vec3 diffuse = DIF_W * vec3(texture2D(material.diffuse, texCoords));
    vec3 specular = SPC_W * vec3(texture2D(material.specular, texCoords));

    vec3 lightDir = normalize(light.position - fragPos);

    vec3 ambientLight = ambient * light.color;

    float diffuseStrength = max(0.0, dot(lightDir, fragNormal));
    vec3 diffuseLight = diffuseStrength * diffuse * light.color;

    vec3 viewDir = normalize(spotlight.position - fragPos); // hack cuz i didn't pass in any viewPos; assume spotlight is at camera
    vec3 reflectDir = reflect(-lightDir, fragNormal);
    float specularity = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specularLight = specular * specularity * light.color;

    return ambientLight + diffuseLight + specularLight;
}


vec3 calcSpotLight(SpotLight light) 
{
    const float AMB_W = 0.5;
    const float DIF_W = 1.0;
    const float SPC_W = 2.0;

    // calculate light intensity using attenuation
    float dist = length(fragPos - light.position);
    float attenuation = 1.0 / (light.constant + (light.linear * dist) + (light.quadratic * dist * dist));

    vec3 ambient = attenuation * AMB_W * vec3(texture2D(material.diffuse, texCoords));
    vec3 diffuse = attenuation * DIF_W * vec3(texture2D(material.diffuse, texCoords));
    vec3 specular = attenuation * SPC_W * vec3(texture2D(material.specular, texCoords));

    // calculate cosine angle between light->frag and light direction
    vec3 fragToLight = fragPos - light.position;
    float theta = dot(normalize(fragToLight), normalize(light.direction));

    // if contained within cutoff, calculate lighting normally
    if (theta > light.outerCutOffCos) {

        // fade out lighting after innerCutOff radius, up to outerCutOff
        float epsilon = light.innerCutOffCos - light.outerCutOffCos;   
        float intensity = smoothstep(0.0, 1.0, (theta - light.outerCutOffCos) / epsilon);
        diffuse *= intensity;
        specular *= intensity;     

        // 3 lighting types
        vec3 ambientLight = ambient * light.color;

        vec3 lightDir = normalize(light.position - fragPos);
        float diffuseStrength = max(0.0, dot(lightDir, fragNormal));
        vec3 diffuseLight = diffuseStrength * diffuse * light.color;

        vec3 viewDir = normalize(light.position - fragPos);
        vec3 reflectDir = reflect(-lightDir, fragNormal);
        float specularity = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
        vec3 specularLight = specular * specularity * light.color;

        return ambientLight + diffuseLight + specularLight;
    }

    // else just use ambient
    else {
        return ambient * light.color;
    } 
}
*/