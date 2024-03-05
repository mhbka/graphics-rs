mod shader;

use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use glfw::fail_on_errors;
use gl::types::*;
use std::mem::{size_of, size_of_val};
use std::ptr::null;
use std::env;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // init glfw and set window hints as found in tutorial
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();    
    /* 
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    */

    // Create a windowed mode window and its OpenGL context
    let (width, height) = (640, 480);
    let (mut window, events) = glfw.create_window(width, height, "Hello", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Load window's function pointers into `gl`
    gl::load_with(|s| {window.get_proc_address(s)});

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Initialize and bind a VAO and its VBO
    let data: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo as *mut u32);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            size_of_val(&data).try_into().unwrap(), 
            data.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );

        gl::GenVertexArrays(1, &mut vao as *mut u32);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<f32>() as i32, null());
        gl::EnableVertexAttribArray(0);
    }

    let shader_program = unsafe { shader::create_and_link_shaders() };
    
    // Set viewport + framebuffer size callback fn
    unsafe { gl::Viewport(0, 0, width as i32, height as i32); }
    window.set_framebuffer_size_callback(|_, w, h| unsafe {gl::Viewport(0, 0, w, h)});

    // Loop until the user closes the window
    while !window.should_close() {
        window.swap_buffers();

        unsafe { 
            // Set bg color
            gl::ClearColor(0.9, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT); 

            // Use shader + bind VAO
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            //gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, i%50, null());
            
            // Draw triangles
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
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