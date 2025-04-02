#version 330 core
in vec2 TexCoord;
in float heightValue;

uniform sampler2D myTexture;

out vec4 FragColor;

void main()
{
    float grayscale = clamp(1 - heightValue * 0.0004, 0.0, 1.0);
    FragColor = vec4(vec3(sin(grayscale)), 1.0);
}