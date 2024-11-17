#version 330 core

layout (location = 0) in vec2 pos_vertex;

uniform vec2 u_resolution;
uniform vec2 u_camera_pos;
uniform float u_camera_scale;


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
}