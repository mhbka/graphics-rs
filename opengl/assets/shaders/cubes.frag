#version 330 core
    in vec3 vertexColor;
    in vec2 texCoord;

    out vec4 FragColor;

    uniform sampler2D texture1;
    uniform sampler2D texture2;
    uniform float mix_amount;

    void main()
    {
        FragColor = mix(
            texture2D(texture1, texCoord),
            texture2D(texture2, texCoord),
            mix_amount);
    } 