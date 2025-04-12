#version 330 core
flat in uint vid;

out uvec3 FragColor;

void main()
{
    FragColor = uvec3(vid);
}