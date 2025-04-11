#version 330 core
flat in uint vid;

out uvec3 FragColor;

void main()
{
    int i = int(vid);
    int r = (i & 0x000000FF) >>  0;
    int g = (i & 0x0000FF00) >>  8;
    FragColor = uvec3(r, g, gl_PrimitiveID);
}