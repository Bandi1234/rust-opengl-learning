#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex_coords;

out vec2 v_tex_coords;
out float v_z;

uniform mat4 u_mvp;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = u_mvp * vec4(pos.x, pos.y, pos.z, 1.0);
    v_z = (pos.z + 1) / 3;
}
