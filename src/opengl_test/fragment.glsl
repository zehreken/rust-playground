#version 330 core
out vec4 FragColor;
in float scale_out;
in float iteration_out;

void main()
{
    int iterations = int(iteration_out);
    float scale = scale_out;
    vec2 z, c;
    vec2 center = vec2(1.5, 0.0);

    c.x = 1.3333 * ((gl_FragCoord.x / 800.0) - 0.5) * scale - center.x;
    c.y = ((gl_FragCoord.y / 600.0) - 0.5) * scale - center.y;

    z = c;
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
    FragColor = vec4(value, value * value, 1.0 - value, 1.0);
    // FragColor = vec4(0.4f, 0.0f, 0.2f, 1.0f);
}
