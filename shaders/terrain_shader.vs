#version 330 core
layout (location = 0) in vec3 vertices;
layout (location = 1) in vec2 uv;

uniform mat4 MVP;

out vec2 TexCoord;

void main()
{
    gl_Position = MVP * vec4(vertices, 1.0);
    TexCoord = uv;
}
