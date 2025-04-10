#version 330 core
layout (location = 0) in vec3 vertices;
layout (location = 1) in uint vertex_id;

uniform mat4 MVP;

out uint vid;

void main()
{
    gl_Position = MVP * vec4(vertices, 1.0);
    vid = vertex_id;
}
