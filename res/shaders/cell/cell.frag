#version 330 core 

#define MAX_RADIUS_CELL 0.45
#define RADIUS_BALLS 0.08
#define COUNT_BALLS 8.0
#define SPEED 0.5
#define ANGLE_FOR_BALLS 360.0/COUNT_BALLS
#define PI 3.14

in vec2 ST;
in vec3 color_cell;

uniform float u_time;
uniform vec2 u_resolution;

void main(void) {
    float time = u_time * SPEED;
    float radius_cell = MAX_RADIUS_CELL;
    float contrast = 0.006;
    vec4 col = vec4(vec3(0.0), 0.0);
    
    vec2 uv = ST;
    
    vec2 center_cell = vec2(0.5);
    
    float d = distance(uv, center_cell);
    
    float beta = cos(uv.x*12.0 + time)*sin(uv.y*12.0 + time) / 300.0;
    
    radius_cell = MAX_RADIUS_CELL;
    float alpha = smoothstep(radius_cell + beta, radius_cell - contrast + beta, d);
    col += vec4(color_cell*vec3(0.7) * alpha, 1.0) + smoothstep(radius_cell, 0.2, d)/10.0;
    
    radius_cell = MAX_RADIUS_CELL * 0.95;
    alpha = smoothstep(radius_cell - beta, radius_cell - contrast - beta, d);
    col += vec4(color_cell * alpha, 1.0);
    
    radius_cell = MAX_RADIUS_CELL * 0.2;
    alpha = smoothstep(radius_cell, radius_cell - contrast, d);
    col -= vec4(color_cell*vec3(0.9) * alpha, 1.0) - smoothstep(radius_cell, 0.0, d)/5.0;
    
    for (int i = 0; i < int(COUNT_BALLS); i++) {
        float angle_ball = (ANGLE_FOR_BALLS * float(i));
        float distance_core = MAX_RADIUS_CELL * 0.5;
        vec2 pos_ball = vec2(0.0);
        pos_ball.x = center_cell.x + distance_core * cos(angle_ball * PI/180.0 + time);
        pos_ball.y = center_cell.y + distance_core * sin(angle_ball * PI/180.0 + time);
            
        float beta = cos(pos_ball.x*uv.x*20.0+time)*sin(pos_ball.y*uv.y*20.0+time)/100.0;
        float db = distance(uv, pos_ball)+beta;
            
        alpha = smoothstep(RADIUS_BALLS, RADIUS_BALLS - contrast, db);
        col -= vec4(color_cell*vec3(0.5) * alpha, 1.0) - smoothstep(RADIUS_BALLS, 0.03, db)/10.0;
    }

    radius_cell = MAX_RADIUS_CELL;
    alpha = smoothstep(radius_cell + beta, radius_cell - contrast + beta, d);
    col.a = alpha;

    gl_FragColor = col;
}
