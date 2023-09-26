#version 450 core

in layout(location = 0) vec3 position;
in layout(location = 1) vec4 colour;
uniform layout(location=2) ivec2 WindowSize;
uniform layout(location=3) float time;
uniform layout(location=4) mat4 transformation;
out layout(location=0) vec4 VertexColour;

void main()
{
    // float aspect = float(WindowSize[0]) / float(WindowSize[1]);
    // vec4 col1 = vec4(cos(time), aspect* sin(time), 0.0, 0.0);
    // vec4 col2 = vec4( -sin(time), aspect* cos(time), 0.0, 0.0); // Scales Y-axis since window is stretched 800/600
    // vec4 col3 = vec4(0.0, 0.0, 1.0, 0.0);
    // vec4 col4 = vec4(0.0, 0.0, 0.0, 1.0);
    // mat4 transformation = mat4(col1, col2, col3, col4); // sets columns of matrix n

    vec4 homogenous_coordinates = vec4(position, 1);
    vec4 transformed = transformation*homogenous_coordinates;
    
    gl_Position = vec4(transformed); 

    VertexColour = colour;
}

// RENDERDOC for debug