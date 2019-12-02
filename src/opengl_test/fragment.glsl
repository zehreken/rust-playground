#version 330 core
out vec4 FragColor;
in float scale_out;

void main()
{
    int iterations = 10;
    float scale = scale_out;
    vec2 z, c;
    vec2 center = vec2(0.5, 0.0);

    c.x = 1.3333 * ((gl_FragCoord.x / 800.0) - 0.5) * scale - center.x;
    c.y = ((gl_FragCoord.y / 600.0) - 0.5) * scale - center.y;

    z = c;
    int i;
    for (i = 0; i < iterations; i++)
    {
        float x = (z.x * z.x - z.y * z.y) + c.x;
        float y = (z.y * z.x + z.x * z.y) + c.y;

        if ((x * x + y * y) > 4.0) break;
        z.x = x;
        z.y = y;
    }
    
    FragColor = vec4((i == iterations ? 0.0 : float(i)) / float(iterations), 0.0, 0.0, 1.0);
    // FragColor = vec4(0.4f, 0.0f, 0.2f, 1.0f);
}
