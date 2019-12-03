#version 330 core
out vec4 FragColor;
in float scale_out;
in float iteration_out;

void main()
{
    float scale = scale_out;
    int iterations = int(iteration_out);
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
