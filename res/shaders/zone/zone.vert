#version 430 core

layout (location = 0) in vec2 pos_vertex;
layout (location = 1) in vec2 st;
layout (location = 2) in vec3 zone_color;
layout (location = 3) in float borders;

layout (location = 0) uniform vec2 u_resolution;
layout (location = 1) uniform vec2 u_camera_pos;
layout (location = 2) uniform float u_camera_scale;

out vec2 St;
out vec3 Color;
out float Ltrb;

void main(void) {
    vec4 n_cam_pos = vec4(u_camera_pos.xy / u_resolution.xy, 0.0, 1.0);
    vec4 uv = vec4(pos_vertex.xy / u_resolution.xy, 0.0, 1.0);

    mat4 transform_matrix = mat4(
        1.0,            0.0,            0.0, 0.0,
        0.0,            1.0,            0.0, 0.0,
        0.0,            0.0,            1.0, 0.0,
        -n_cam_pos.x,    -n_cam_pos.y,   0.0, 1.0
    );

    mat4 scale_matrix = mat4(
        u_camera_scale,   0.0,            0.0, 0.0,
        0.0,            u_camera_scale,  0.0, 0.0,
        0.0,            0.0,            1.0, 0.0,
        0.0,            0.0,            0.0, 1.0
    );

    vec4 pos =  scale_matrix * transform_matrix * uv;

    gl_Position = vec4(pos);
    St = st;
    Color = zone_color;
    Ltrb = borders;
}