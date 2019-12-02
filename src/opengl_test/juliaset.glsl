#version 330 core
out vec4 FragColor;
in float scale_out;
in float iteration_out;

void main()
{
    float scale = scale_out;
    int iterations = int(50);
    vec2 z;

    vec2 c = vec2(-0.7, 0.27015);
    c.x = cos(scale / 103);
    c.y = sin(scale / 71);

    z.x = 3.0 * (gl_FragCoord.x / 800.0 - 0.5);
    z.y = 3.0 * (gl_FragCoord.y / 600.0 - 0.5);

    int i;
    for (i = 0; i < iterations; i++)
    {
        float x = (z.x * z.x - z.y * z.y) + c.x;
        float y = (z.x * z.y + z.x * z.y) + c.y;

        if ((x * x + y * y) > 4.0) break;
        z.x = x;
        z.y = y;
    }

    float value = (i == iterations ? 0.0 : float(i)) / float(iterations);
    FragColor = vec4(value, value * value, value * value, 1.0);
}

/*
vec2 multiply(vec2 x,vec2 y){
    return vec2(x.x*y.x-x.y*y.y,x.x*y.y+x.y*y.x);
}
void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec2 z0 = 5.*(fragCoord/iResolution.x-vec2(.5,.27));
    vec2 col;
    vec2 c = cos(iTime)*vec2(cos(iTime/2.),sin(iTime/2.)); 
    for(int i = 0; i < 500;i++){
        vec2 z = multiply(z0,z0)+c;
        float mq = dot(z,z);
        if( mq > 4.){
            col = vec2(float(i)/20.,0.);
            break;
        } else {
            z0 = z;
        }
        col =  vec2(mq/2.,mq/2.);
    }
    fragColor = vec4(col,0.,1.);
}
*/
