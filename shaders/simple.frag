#version 450 core

in layout(location=0) vec4 VertexColour;
in layout(location=1) vec3 VertexNormal;
in layout(location=2) vec3 VertexPosition;
out vec4 FragColour;

uniform layout(location=3) ivec2 WindowSize;
uniform layout(location=7) vec3 view_position;

void main()
{

    // Phong lighting model
    vec3 light_color = vec3(1.0f, 1.0f, 1.0f);
    vec3 light_position = vec3(50.0f, 50.0f, -14.0f);

    // Ambient
    float ambient_strength = 0.08;
    vec3 ambient = ambient_strength * light_color;

    // Diffuse
    vec3 normal = normalize(VertexNormal);
    vec3 light_direction = normalize(light_position - VertexPosition);
    float diffuse_strength = 0.9*max(dot(normal,light_direction), 0.0);
    vec3 diffuse = diffuse_strength * light_color;

    // Specular
    float shininess = 16;
    float specular_strength = 2;
    vec3 view_direction = normalize(VertexPosition - view_position);
    vec3 reflect_direction = reflect(-light_direction, normal);
    float specular_compensation = pow(max(dot(-view_direction, reflect_direction), 0.0), shininess);
    vec3 specular = specular_strength * specular_compensation * light_color;

    //Combined
    vec3 result = (ambient + diffuse + specular) * vec3(VertexColour);
    FragColour = vec4(result, 1.0);
    
    // ---------------------------------------------------------------------------------------- //

    // Standard lightning model
    // vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
    // float light_factor = max( dot(VertexNormal, -lightDirection) ,0); //GL CLEARCOLOR?
    // FragColour  = vec4(light_factor*VertexColour[0], 
    //                     light_factor*VertexColour[1], 
    //                     light_factor*VertexColour[2], 
    //                     VertexColour[3]);
                        
    
    
    
    // Windowsizes
    // float x_center = WindowSize[0]/2;
    // float y_center = WindowSize[1]/2;

    // // Floored rasterized pixel values
    // float floorx = floor(gl_FragCoord.x/10);
    // float floory = floor(gl_FragCoord.y/10);
    
    // // Checkboard
    // if(mod(floorx + floory,2)  == 1) {
    //     FragColour = vec4(1.0, 1.0, 1.0, 1.0);
    // }
    // else {
    //     FragColour = vec4(0.0, 0.0, 0.0, 1.0);
    // }

    // // Green centre circle
    // if(pow((gl_FragCoord.x-x_center),2) + pow((gl_FragCoord.y-y_center),2)< 200){
    //     FragColour = vec4(0.0, 1.0, 0.0, 1.0);
    // }

    // // Sin wave y = sin(x), horizontal
    // if(abs(30*sin(0.1*(gl_FragCoord.x-x_center))-(gl_FragCoord.y-y_center)) < 2){
    //     FragColour = vec4(1.0, 0.0, 0.0, 1.0);
    // }
    // if(abs(30*sin(0.1*(gl_FragCoord.x-x_center-1))-(gl_FragCoord.y-y_center)) < 2){
    //     FragColour = vec4(1.0, 0.0, 0.0, 1.0);
    // }
    // if(abs(30*sin(0.1*(gl_FragCoord.x-x_center+1))-(gl_FragCoord.y-y_center)) < 2){
    //     FragColour = vec4(1.0, 0.0, 0.0, 1.0);
    // }

    // // Sin wave x = sin(y), vertical
    // if(abs(30*sin(0.1*(gl_FragCoord.y-y_center))-(gl_FragCoord.x-x_center)) < 2){
    //     FragColour = vec4(0.3, 1.0, 1.0, 1.0);
    // }
    // if(abs(30*sin(0.1*(gl_FragCoord.y-y_center-1))-(gl_FragCoord.x-x_center)) < 2){
    //     FragColour = vec4(0.3, 1.0, 1.0, 1.0);
    // }
    // if(abs(30*sin(0.1*(gl_FragCoord.y-y_center+1))-(gl_FragCoord.x-x_center)) < 2){
    //     FragColour = vec4(0.3, 1.0, 1.0, 1.0);
    // }

}