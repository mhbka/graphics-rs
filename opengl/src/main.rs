mod glfw_init;
mod gl_vao_init;
mod shader;

use glfw::{Action, Context, Key};
use shader::Shader;
use std::env;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let use_old_ver = true;

    // Initialize GLFW window and make current + load functions into `gl`
    let (mut glfw, mut window, mut events) = glfw_init::init(true);

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

    // Initialize and bind VAO, VBO, EBO
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ebo: u32 = 0;
    unsafe { gl_vao_init::init(vertices.as_slice(), None, &mut vao, &mut vbo, &mut ebo) };

    let shader_program = unsafe { Shader::new("test") };

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

            // Use shader + bind VAO
            shader_program.use_program();
            gl::BindVertexArray(vao);

            // Draw triangles
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

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