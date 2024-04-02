#version 450 core

in layout(location = 0) vec3 position;
in layout(location = 1) vec4 colour;
in layout(location = 2) vec3 normal;
uniform layout(location=3) ivec2 WindowSize;
uniform layout(location=4) float time;
uniform layout(location=5) mat4 model_view_projection;
uniform layout(location=6) mat4 model_matrix;
out layout(location=0) vec4 VertexColour;
out layout(location=1) vec3 VertexNormal;
out layout(location=2) vec3 VertexPosition;

void main()
{
    // float aspect = float(WindowSize[0]) / float(WindowSize[1]);
    
    // Phong lighting
    mat3 normal_matrix = mat3(model_matrix);
    vec3 new_normal = inverse(transpose(normal_matrix)) * normal;

    vec4 homogenous_coordinates = vec4(position, 1);
    vec4 transformed = model_view_projection*homogenous_coordinates;
    gl_Position = vec4(transformed); 

    VertexColour = colour;
    VertexNormal = new_normal;
    VertexPosition = position;



    // Standard lighting
    //vec3 new_normal = normalize(normal_matrix*normal);
    //VertexColour = vec4(new_normal,colour[3]);
}

// RENDERDOC for debug