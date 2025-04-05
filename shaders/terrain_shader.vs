#version 330 core
layout (location = 0) in vec3 vertices;
layout (location = 1) in vec2 uv;

uniform mat4 MVP;
uniform mat4 modelMatrix;
uniform float maxHeight;

out vec2 TexCoord;
out float heightValue;

void main()
{
    gl_Position = MVP * vec4(vertices, 1.0);
    TexCoord = uv;

    vec4 worldPos = modelMatrix * vec4(vertices, 1.0);
    heightValue = vertices.y / maxHeight + (worldPos.y * 0.000004);
}
