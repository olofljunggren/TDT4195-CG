#version 450 core

in layout(location=0) vec4 VertexColour;
in layout(location=1) vec3 VertexNormal;
out vec4 FragColour;

uniform layout(location=3) ivec2 WindowSize;

void main()
{
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
    
    
    float light_factor = max( dot(VertexNormal, -lightDirection) ,0); //GL CLEARCOLOR?
    FragColour = vec4(light_factor*VertexColour[0], 
                        light_factor*VertexColour[1], 
                        light_factor*VertexColour[2], 
                        VertexColour[3]);
                        
    
    // Windowsizes
    float x_center = WindowSize[0]/2;
    float y_center = WindowSize[1]/2;

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