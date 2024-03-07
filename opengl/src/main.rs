mod glfw_init;
mod shader;
mod vao;
mod texture;

use glfw::{Action, Context, Key};
use shader::Shader;
use vao::{VAO, VertexAttr};
use texture::Texture;
use std::env;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let use_old_ver = true;

    // Initialize GLFW window and make current + load functions into `gl`
    let (mut glfw, mut window, mut events) = glfw_init::init(use_old_ver);

    // vertex data
    let vertex_data: Vec<f32> = vec![
        // positions      // colors        // texture coords
        0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // top right
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // top left 
    ];
    
    // Initialize VAO
    let vertex_attrs = vec![
        VertexAttr::new("Position".to_owned(), 3), 
        VertexAttr::new("Color".to_owned(), 3),
        VertexAttr::new("Texture Coords".to_owned(), 2)
        ];
    unsafe { VAO::new(vertex_data, None, vertex_attrs) };

    // Initialize and use shader
    let shader_program = unsafe { Shader::new("test") };
    unsafe { shader_program.use_program(); }

    // Initialize texture
    let texture = unsafe { Texture::new("wall.jpg") };

    // Check for error before main loop (also using this for checking error during loop)
    let mut cur_error = unsafe { gl::GetError() };
    if cur_error != 0 { panic!("error during init: {cur_error} ");} 
    else { println!("note: initialized safely"); }

    //
    // MAIN LOOP - until window is closed
    // 
    while !window.should_close() {
        window.swap_buffers();

        unsafe { 
            // Set BG color
            gl::ClearColor(0.9, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT); 

            // Draw triangles
            gl::DrawArrays(gl::TRIANGLES, 0, 4);

            // Check for any new errors
            let error = gl::GetError();
            if error != cur_error {
                println!("error: {error}");
                cur_error = error;
            }
        };

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}