#version 330 core

uniform vec3 highlightColor;

out vec4 FragColor;

void main()
{
    FragColor = vec4(highlightColor.r, highlightColor.g, highlightColor.b, 1.0);
}