#version 330 core
out uint vid;

out vec4 FragColor;

void main()
{
    int i = int(vid);
    int r = (i & 0x000000FF) >>  0;
    int g = (i & 0x0000FF00) >>  8;
    int b = (i & 0x00FF0000) >> 16;
    FragColor = vec4(vec3(r, g, b), 1.0);
}