#version 330 core

layout(location=0) out vec4 final_color;

uniform sampler2D samp;

in vec2 v_tex_coords;
in float v_z;

void main() {
    // vec2 uv = gl_FragCoord.xy / screen_size;
    // final_color = vec4(uv.x, uv.y, blue_extra, 1.0);
    vec4 tex_color = texture(samp, v_tex_coords);
    final_color = vec4(tex_color.r, tex_color.g, tex_color.b, tex_color.a);
    final_color = vec4(0.2 * v_z, 0.3 * v_z, 0.7 * v_z, 1.0);
}
