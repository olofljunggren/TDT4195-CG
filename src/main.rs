// Uncomment these following global attributes to silence most warnings of "low" interest:

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]

extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod mesh;
mod scene_graph;

use scene_graph::SceneNode;
use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  pointer_to_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Creates a vector of vertices needed to construct a circle in z=0 plane
fn create_circle(n: u32) -> Vec<f32> {
    let mut vector: Vec<f32> = vec![];

    vector.push(0.0);
    vector.push(0.0);
    vector.push(0.0);

    for i in 1..n+1 {
        let angle = 2.0*3.14159265*(i as f32)/(n as f32);
        let x = angle.cos();
        let y = angle.sin();
        let z = 0.0;
        vector.push(x);
        vector.push(y);
        vector.push(z);
        
        }

        return vector
}

// Generates indices for the vertices in create_circle
fn genereate_circle_indices(n: u32) -> Vec<u32> {
    let mut center_index: u32 = 0;
    let mut vector: Vec<u32> = vec![];


    for i in 0..(n) {
        vector.push(center_index);
        vector.push(i+1);
        if i == n-1{
            vector.push(1);
        }
        else{
            vector.push(i+2);
        }
        }

        return vector
}

unsafe fn draw_scene(node: &scene_graph::SceneNode,
    view_projection_matrix: &glm::Mat4,
    transformation_so_far: &glm::Mat4,
    shader: &shader::Shader) {
    // Perform any logic needed before drawing the node
    
    let mut local_transform: glm::Mat4 = glm::identity();

    let translaton_to_origin: glm::Mat4 = glm::translation(&-node.reference_point);
    let rotation_matrix_x: glm::Mat4 = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0)); 
    let rotation_matrix_y: glm::Mat4 = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0)); 
    let rotation_matrix_z: glm::Mat4 = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0)); 
    let translaton_back: glm::Mat4 = glm::translation(&node.reference_point);
    local_transform = translaton_back * rotation_matrix_y * translaton_to_origin * local_transform;
    // node.position
    // node.rotation
    // node.scale
    // node.reference_point

    // Check if node is drawable, if so: set uniforms, bind VAO and draw VAO
    if node.index_count != -1 {
        gl::BindVertexArray(node.vao_id);
        let location: i32 = shader.get_uniform_location("transformation");
        let total_transformation = view_projection_matrix*transformation_so_far*local_transform;
        gl::UniformMatrix4fv(location, 1, gl::FALSE, total_transformation.as_ptr());
        gl::DrawElements(gl::TRIANGLES, node.index_count, gl::UNSIGNED_INT, 0 as *const c_void);
    }
    if node.get_n_children() > 0 {
        // Recurse
        for &child in &node.children {
            draw_scene(&*child, view_projection_matrix, &(transformation_so_far*local_transform), shader);
        }
    }
}


//let mut vertices: [f32; 9] = [1.0, 3.0, 2.0, 5.0, 4.0, 3.0, 2.0, 6.0, 3.0];

// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colours: &Vec<f32>, normals: &Vec<f32>) -> u32 { 

    // c++ syntax :
    // void glGenVertexArrays(int count, unsigned int* arrayIDs);  VAO
    // void glBindVertexArray(unsigned int vertexArrayID);         BIND VAO
    // void glGenBuffers(int count, unsigned int* bufferIDs);      Create VBO
    // void glBindBuffer(enum target, unsigned int bufferID);      Bind VBO
    // void glBufferData(enum target, size_t size, void* data, enum usage); Buffer input data
    // let byte_size_of_f32: usize = mem::size_of::<f32>();

    
    

    let mut array_id: u32 = 1;
    unsafe {
            gl::GenVertexArrays(1, &mut array_id);
            gl::BindVertexArray(array_id);

            let mut vertices_buffer_id: u32 = 0;
            gl::GenBuffers(1, &mut vertices_buffer_id);
            assert!(vertices_buffer_id != 0);            
            gl::BindBuffer(gl::ARRAY_BUFFER, vertices_buffer_id);
            gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(vertices), pointer_to_array(vertices), gl::STATIC_DRAW);
            println!("Size of vertices array: {}", byte_size_of_array(vertices));
    
            // * Tells vertex shader where to get input data
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3*size_of::<f32>(), offset::<u32>(0));
            gl::EnableVertexAttribArray(0);

            // * Generate an IBO, bind it and fill it with data
            let mut index_buffer_id: u32 = 0;
            gl::GenBuffers(1, &mut index_buffer_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, byte_size_of_array(indices), pointer_to_array(indices), gl::STATIC_DRAW);
            println!("Size of indices array: {}", byte_size_of_array(indices));

             // * Generate an RGBA, bind it and fill it with data
             let mut colour_id: u32 = 0;
            gl::GenBuffers(1, &mut colour_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, colour_id);
            gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(colours), pointer_to_array(colours), gl::STATIC_DRAW);
            println!("Size of colours array: {}", byte_size_of_array(colours));
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 4*size_of::<f32>(), offset::<u32>(0));
            gl::EnableVertexAttribArray(1);

            // * Generate an normals objecct, bind it and fill it with data
            let mut normal_id: u32 = 0;
            gl::GenBuffers(1, &mut normal_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, normal_id);
            gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(normals), pointer_to_array(normals), gl::STATIC_DRAW);
            println!("Size of normals array: {}", byte_size_of_array(normals));
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 3*size_of::<f32>(), offset::<u32>(0));
            gl::EnableVertexAttribArray(2);


            }
    return array_id;

    // This should:
    // * Generate a VAO and bind it
    // * Generate a VBO and bind it
    // * Fill it with data
    // * Configure a VAP for the data and enable it
    // * Generate a IBO and bind it
    // * Fill it with data
    // * Return the ID of the VAO
}


fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el   = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        //let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO around here

        // ------------Set 1 of vertices and indices---------------
        // let vertices: Vec<f32> = vec![-0.6,-0.6, -10.0, 
        //                                0.6,-0.6, -10.0, 
        //                                0.0, 0.6, -10.0, 
        //                                1.0, 1.0, -10.0, 
        //                                2.0, 1.5, -10.0,
        //                                0.0, 2.0, -10.0,
        //                               -1.0, 1.0, -10.0,
        //                               -1.0, 2.0, -10.0,
        //                               -2.0, 1.0, -10.0,
        //                               -2.0,-2.0, -10.0,
        //                                0.0,-2.0, -10.0,
        //                               -1.0,-1.0, -10.0,
        //                                1.0,-2.0, -10.0,
        //                                3.0, 0.0, -10.0,
        //                                1.0, 0.0, -10.0];
        // let indices: Vec<u32> = vec![0, 1, 2,
        //                             3, 4, 5,
        //                             6, 7, 8,
        //                             9, 10, 11,
        //                             12, 13 ,14];
        // let colours: Vec<f32> = vec![
        //                         1.0,0.0,0.0,1.0,
        //                         1.0,1.0,0.0,1.0,
        //                         1.0,0.0,1.0,1.0,
        //                         1.0,0.0,0.0,1.0,
        //                         1.0,1.0,0.0,1.0,
        //                         0.0,1.0,1.0,1.0,
        //                         1.0,0.0,0.0,1.0,
        //                         1.0,1.0,0.0,1.0,
        //                         0.0,1.0,0.0,1.0,
        //                         1.0,0.0,1.0,1.0,
        //                         0.0,1.0,1.0,1.0,
        //                         0.0,1.0,0.0,1.0,
        //                         0.0,1.0,0.0,1.0,
        //                         1.0,0.0,1.0,1.0,
        //                         0.0,1.0,1.0,1.0,
        //                         ];
                                            



        // ------------Set 2 of vertices and indices---------------
        // let vertices: Vec<f32> = vec![0.6, -0.8, -1.2,
        //                             0.0, 0.4, 0.0,
        //                             -0.8, -0.2, 1.2];
        // let indices: Vec<u32> = vec![0, 1, 2];
        // let colours: Vec<f32> = vec![
        //         1.0,0.0,0.0,1.0,
        //         1.0,0.0,0.0,1.0,
        //         1.0,0.0,0.0,1.0,
        //         ];

        // ------------Set 3 of vertices and indices---------------
        // let vertices: Vec<f32> = create_circle(40);
        // let indices: Vec<u32> = genereate_circle_indices(40);

        // ----------Print elements in vertices and indices--------  
        // for element in indices.iter() {
        //     println!("{}", element);
        // }
        // for element in vertices.iter() {
        //     println!("{}", element);
        // }


        // ------------Set 4 of vertices and indices---------------
        // let vertices: Vec<f32> = vec![  0.0, -0.6, 0.0, 
        //                                 1.2, -0.6, 0.0, 
        //                                 1.6,  0.6, 0.0,
        //                                -0.6, -0.6, 1.0, 
        //                                 0.6, -0.6, 1.0, 
        //                                 1.0,  0.6, 1.0, 
        //                                 0.2,  0.0, 2.0, 
        //                                 1.4,  0.0, 2.0, 
        //                                 1.8,  1.2, 2.0];
        // let indices: Vec<u32> = vec![
        //                             6, 7, 8,
        //                             3, 4, 5,
        //                             0, 1, 2,
        //                                     ];
        // let colours: Vec<f32> = vec![
        //                         1.0,0.0,0.0,0.5,
        //                         1.0,0.0,0.0,0.5,
        //                         1.0,0.0,0.0,0.5,
        //                         0.0,1.0,0.0,0.5,
        //                         0.0,1.0,0.0,0.5,
        //                         0.0,1.0,0.0,0.5,
        //                         0.0,0.0,1.0,0.5,
        //                         0.0,0.0,1.0,0.5,
        //                         0.0,0.0,1.0,0.5];

        // ------------Set 5 of vertices and indices---------------
        // let vertices: Vec<f32> = vec![  0.0,  2.0, 3.0, 
        //                                -2.0,  1.0, 3.0, 
        //                                 1.0, -2.0, 3.0,];
        // let indices: Vec<u32> = vec![
        //                             0, 1, 2,];
        // let colours: Vec<f32> = vec![
        //                         1.0,0.0,0.0,0.5,
        //                         1.0,0.0,0.0,0.5,
        //                         1.0,0.0,0.0,0.5,];

        // ------------Set 6 Cube---------------
        // let vertices: Vec<f32> = vec![
        //     // front
        //     -1.0, -1.0,  -7.0,
        //      1.0, -1.0,  -7.0,
        //      1.0,  1.0,  -7.0,
        //     -1.0,  1.0,  -7.0,
        //     // back
        //     -1.0, -1.0,  -5.0,
        //      1.0, -1.0,  -5.0,
        //      1.0,  1.0,  -5.0,
        //     -1.0,  1.0,  -5.0,
        // ];
        
        // let colours: Vec<f32> = vec![
        //     // front
        //     1.0, 0.0, 0.0, 1.0,
        //     1.0, 0.0, 0.0, 1.0,
        //     1.0, 0.0, 0.0, 1.0,
        //     1.0, 0.0, 0.0, 1.0,
        //     // back  
        //     0.0, 0.0, 1.0, 1.0,
        //     0.0, 0.0, 1.0, 1.0,
        //     0.0, 0.0, 1.0, 1.0,
        //     0.0, 0.0, 1.0, 1.0,
        // ];

        // let indices: Vec<u32> = vec![
        //     7, 6, 5,
        //     5, 4, 7,
        //     0, 1, 2,
        //     2, 3, 0,
        //     1, 5, 6,
        //     2, 1, 6,
        //     4, 0, 3,
        //     3, 7, 4,
        //     1, 4, 5,
        //     4, 1, 0,
        //     3, 2, 6,
        //     6, 7, 3
        // ];

        //let index_count = indices.len() as i32;   


        // Load lunar_surface object
        let terrain_mesh: mesh::Mesh = mesh::Terrain::load("./resources/lunarsurface.obj");
        let lunar_surface_colours: Vec<f32> = terrain_mesh.colors;
        let lunar_surface_indices: Vec<u32> = terrain_mesh.indices;
        let lunar_surface_vertices: Vec<f32> = terrain_mesh.vertices;
        let lunar_surface_index_count: i32 = terrain_mesh.index_count;
        let lunar_surface_normals: Vec<f32> = terrain_mesh.normals;

        let lunar_surface: u32 = unsafe {
            create_vao(&lunar_surface_vertices, &lunar_surface_indices, &lunar_surface_colours, &lunar_surface_normals) 
            };
        
        // Load entire helicopter object
        let helicopter_mesh: mesh::Helicopter = mesh::Helicopter::load("./resources/helicopter.obj");
        let body_mesh: mesh::Mesh = helicopter_mesh.body;
        let door_mesh: mesh::Mesh = helicopter_mesh.door;
        let main_rotor_mesh: mesh::Mesh= helicopter_mesh.main_rotor;
        let tail_rotor_mesh: mesh::Mesh = helicopter_mesh.tail_rotor;

        // Load body mesh
        let body_colours: Vec<f32> = body_mesh.colors;
        let body_indices: Vec<u32> = body_mesh.indices;
        let body_vertices: Vec<f32> = body_mesh.vertices;
        let body_index_count: i32 = body_mesh.index_count;
        let body_normals: Vec<f32> = body_mesh.normals;

        let body: u32 = unsafe {
            create_vao(&body_vertices, &body_indices, &body_colours, &body_normals)
            };

        // Load door mesh
        let door_colours: Vec<f32> = door_mesh.colors;
        let door_indices: Vec<u32> = door_mesh.indices;
        let door_vertices: Vec<f32> = door_mesh.vertices;
        let door_index_count: i32 = door_mesh.index_count;
        let door_normals: Vec<f32> = door_mesh.normals;

        let door: u32 = unsafe {
            create_vao(&door_vertices, &door_indices, &door_colours, &door_normals)
            };
    
        // Load main_rotor mesh
        let main_rotor_colours: Vec<f32> = main_rotor_mesh.colors;
        let main_rotor_indices: Vec<u32> = main_rotor_mesh.indices;
        let main_rotor_vertices: Vec<f32> = main_rotor_mesh.vertices;
        let main_rotor_index_count: i32 = main_rotor_mesh.index_count;
        let main_rotor_normals: Vec<f32> = main_rotor_mesh.normals;

        let main_rotor: u32 = unsafe {
            create_vao(&main_rotor_vertices, &main_rotor_indices, &main_rotor_colours, &main_rotor_normals)
            };

        // Load tail_rotor mesh
        let tail_rotor_colours: Vec<f32> = tail_rotor_mesh.colors;
        let tail_rotor_indices: Vec<u32> = tail_rotor_mesh.indices;
        let tail_rotor_vertices: Vec<f32> = tail_rotor_mesh.vertices;
        let tail_rotor_index_count: i32 = tail_rotor_mesh.index_count;
        let tail_rotor_normals: Vec<f32> = tail_rotor_mesh.normals;

        let tail_rotor: u32 = unsafe {
            create_vao(&tail_rotor_vertices, &tail_rotor_indices, &tail_rotor_colours, &tail_rotor_normals)
            };


        let mut scene_node = SceneNode::new();
        let mut lunar_surface_node = SceneNode::from_vao(lunar_surface, lunar_surface_index_count);
        scene_node.add_child(&lunar_surface_node);
        
        let mut helicopter1_root_node = SceneNode::new();
        lunar_surface_node.add_child(&helicopter1_root_node);
        let mut body_node = SceneNode::from_vao(body, body_index_count);
        helicopter1_root_node.add_child(&body_node);

        let mut door_node = SceneNode::from_vao(door, door_index_count);
        body_node.add_child(&door_node);

        let mut main_rotor_node = SceneNode::from_vao(main_rotor, main_rotor_index_count);
        body_node.add_child(&main_rotor_node);

        let mut tail_rotor_node = SceneNode::from_vao(tail_rotor, tail_rotor_index_count);
        tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
        body_node.add_child(&tail_rotor_node);


        scene_node.print();
        helicopter1_root_node.print();
        body_node.print();


        

        // == // Set up your shaders here
        let shader_object: shader::Shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.frag")
                .attach_file("./shaders/simple.vert")
                .link()
        };
        unsafe { shader_object.activate(); }

        let mut x_rotation: f32 = 0.0;
        let mut y_rotation: f32 = 0.0;
        
        let mut x_position: f32 = 0.0;
        let mut y_position: f32 = 0.0;
        let mut z_position: f32 = 0.0;

        // Used to demonstrate keyboard handling for exercise 2.
        let mut _arbitrary_number = 0.0; // feel free to remove


        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;

        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let _elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    //window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html

                        VirtualKeyCode::Up => {
                            x_rotation += 2.0*delta_time;
                        }
                        VirtualKeyCode::Down => {
                            x_rotation -= 2.0*delta_time;
                        }
                        VirtualKeyCode::Left => {
                            y_rotation -= 2.0*delta_time;
                        }
                        VirtualKeyCode::Right => {
                            y_rotation += 2.0*delta_time; 
                        }
                        VirtualKeyCode::A => {
                            x_position += 5.0*delta_time*y_rotation.cos();
                            z_position += 5.0*delta_time*y_rotation.sin();
                        }
                        VirtualKeyCode::D => {
                            x_position -= 5.0*delta_time*y_rotation.cos();
                            z_position -= 5.0*delta_time*y_rotation.sin();
                        }
                        VirtualKeyCode::W => {
                            x_position -= 5.0*delta_time*x_rotation.sin()*y_rotation.sin();
                            y_position -= 5.0*delta_time*x_rotation.cos();
                            z_position += 5.0*delta_time*x_rotation.sin()*y_rotation.cos();
                        }
                        VirtualKeyCode::S => {
                            x_position += 5.0*delta_time*x_rotation.sin()*y_rotation.sin();
                            y_position += 5.0*delta_time*x_rotation.cos();
                            z_position -= 5.0*delta_time*x_rotation.sin()*y_rotation.cos();
                        }
                        VirtualKeyCode::LShift => {
                            x_position -= 5.0*delta_time*x_rotation.cos()*y_rotation.sin();
                            y_position += 5.0*delta_time*x_rotation.sin();
                            z_position += 5.0*delta_time*x_rotation.cos()*y_rotation.cos();
                        }
                        VirtualKeyCode::Space => {
                            x_position += 5.0*delta_time*x_rotation.cos()*y_rotation.sin();
                            y_position -= 5.0*delta_time*x_rotation.sin();
                            z_position -= 5.0*delta_time*y_rotation.cos()*x_rotation.cos();
                        }


                        // default handler:
                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }
            
            // == // Please compute camera transforms here (exercise 2 & 3)
            if x_rotation > 3.1415927/2.0
            {
                x_rotation = 3.1415927/2.0;
                println!("Stopp!");
            }
            else if x_rotation < -3.1415927/2.0
            {
                x_rotation = -3.1415927/2.0;
                println!("Stopp!");
            }

            tail_rotor_node.rotation.x += 0.1*_elapsed;

            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                let mut aspect: f32 = 1.0;
                if let Ok(size) = window_size.lock() {
                    gl::Uniform2i(3, size.0 as i32, size.1 as i32);
                    aspect = size.0 as f32/size.1 as f32; 
                }
                gl::Uniform1f(4, _elapsed);

                // Transformations
                let identity: glm::Mat4 = glm::identity();
                let scaler: glm::Mat4 = glm::scaling(&glm::vec3(0.5, 0.5, 0.5));
                let perspective: glm::Mat4 =
                    glm::perspective(aspect, 
                        (3.1415927 as f32)/(2.0 as f32),
                        1.0, 
                        1000.0);

                let rotation_matrix_x: glm::Mat4 = glm::rotation(x_rotation, &glm::vec3(1.0, 0.0, 0.0));
                let rotation_matrix_y: glm::Mat4 = glm::rotation(y_rotation, &glm::vec3(0.0, 1.0, 0.0)); 
                let total_rotation: glm::Mat4 = rotation_matrix_x * rotation_matrix_y;
                let translation: glm::Mat4 = glm::translation(&glm::vec3(x_position, y_position, z_position));
                let view_projection: glm::Mat4 =  perspective * total_rotation * translation * scaler * identity;

                gl::FrontFace(gl::CCW); 
                // == // Issue the necessary gl:: commands to draw your scene here
                // gl::BindVertexArray(lunar_surface);
                // gl::DrawElements(gl::TRIANGLES, lunar_surface_index_count, gl::UNSIGNED_INT, 0 as *const c_void);

                // gl::BindVertexArray(body);
                // gl::DrawElements(gl::TRIANGLES, body_index_count, gl::UNSIGNED_INT, 0 as *const c_void);

                // gl::BindVertexArray(door);
                // gl::DrawElements(gl::TRIANGLES, door_index_count, gl::UNSIGNED_INT, 0 as *const c_void);

                // gl::BindVertexArray(main_rotor);
                // gl::DrawElements(gl::TRIANGLES, main_rotor_index_count, gl::UNSIGNED_INT, 0 as *const c_void);

                // gl::BindVertexArray(tail_rotor);
                // gl::DrawElements(gl::TRIANGLES, tail_rotor_index_count, gl::UNSIGNED_INT, 0 as *const c_void);
                
                draw_scene(&scene_node, &view_projection,&identity, &shader_object);
            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });


    // == //
    // == // From here on down there are only internals.
    // == //


    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size received: {}x{}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }
                    _      => { }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => { }
        }
    });
}
