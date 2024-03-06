mod glfw_init;
mod gl_vao_init;
mod shader;
mod vao;

use glfw::{Action, Context, Key};
use shader::Shader;
use vao::{VAO, VertexAttr};
use std::env;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let use_old_ver = true;

    // Initialize GLFW window and make current + load functions into `gl`
    let (mut glfw, mut window, mut events) = glfw_init::init(use_old_ver);

    // vertex data
    let vertices: Vec<f32> = vec![
        // positions        // colors
        0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,   // bottom left
        0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // top 
    ];
    
    /* 
    // index data
    let indices: Vec<u32> = vec![
        0, 1, 3,
        1, 2, 3
    ];
    */

    // Initialize VAO
    unsafe { VAO::new(vertices, None, Vec::new()) };

    // Initialize and use shader
    let shader_program = unsafe { Shader::new("test") };
    unsafe { shader_program.use_program(); }

    let mut cur_error = unsafe { gl::GetError() };
    if cur_error != 0 { panic!("error during init: {cur_error} ");} 
    else { println!("note: initialized safely"); }

    // MAIN LOOP - until window is closed
    while !window.should_close() {
        window.swap_buffers();

        unsafe { 
            // Set bg color
            gl::ClearColor(0.9, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT); 

            // Draw triangles
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

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