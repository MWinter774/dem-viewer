#version 330 core
flat in uint vid;

uniform uint objectIndex;

out uvec3 FragColor;

void main()
{
    FragColor = uvec3(objectIndex, vid, gl_PrimitiveID);
}