#version 330 core
    layout (location = 0) in vec3 aPos;
    out vec4 vertexColor;

    void main()
    {
        gl_Position = vec4(aPos, 1.0);
        vertexColor = vec4(0.56, 0.06, 0.53, 1.0);
    }