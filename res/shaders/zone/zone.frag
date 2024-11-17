#version 330 core

in vec2 St;
in vec3 Color;
in float Ltrb;

out vec4 fragColor;

uniform vec2 u_resolution;

float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}

bool check_bits(int in_bits, int check_bits) {
    return (in_bits & check_bits) == check_bits;
}

void main(void)
{
    vec4 color = vec4(Color, 1.0);
    float border_size = 0.04;
    vec2 st = St;
    float periodicity = 3.14*4.0;
    int ltrb = int(Ltrb);
    
    float rand_cof = rand(st.xy)*2.0;
    vec4 col =  color * 
        vec4(smoothstep(0.8, 0.9, cos(st.x*periodicity - st.y*periodicity+rand_cof/5.0)));
       
    if (st.x < border_size + rand_cof/1000.0 && !check_bits(ltrb, 1)) {
        col = color;
    }

    if (st.y > 1.0 - border_size + rand_cof/1000.0 && !check_bits(ltrb, 2)) {
        col = color;
    }
    
    if (st.x > 1.0 - border_size + rand_cof/1000.0 && !check_bits(ltrb, 4)) {
        col = color;
    }
    
    if (st.y < border_size + rand_cof/1000.0 && !check_bits(ltrb, 8)) {
        col = color;
    }
    
    
    
        
    fragColor = col;
}