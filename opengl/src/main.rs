mod shader;

use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use glfw::fail_on_errors;
use gl::types::*;
use std::mem::size_of_val;
use std::ptr::null;



fn main() {
    // init glfw and set window hints as found in tutorial
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();    
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    // Create a windowed mode window and its OpenGL context
    let (width, height) = (640, 480);
    let (mut window, events) = glfw.create_window(width, height, "Hello", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Load window's function pointers into `gl`
    gl::load_with(|s| {window.get_proc_address(s)});

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Set viewport + framebuffer size callback fn
    // SELF-NOTE: the closure is supposed to take a `Window` param, but since it's already tied to the `gl` instance, its not necessary
    window.set_framebuffer_size_callback(|_, w, h| unsafe {gl::Viewport(0, 0, w, h)});

    // Set shaders
    unsafe {
        shader::create_and_link_shaders();
    }

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

       // Setting buffer and passing in vertex data
        unsafe { 
            // Set bg color
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT); 

            // Generate a VBO, then bind it to array buffer
            let mut vbo: u32 = 0;
            gl::GenBuffers(1, &mut vbo as *mut u32);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Pass vertex data into the buffer
            let data = [
                -0.5, -0.5, 0.0,
                0.5, -0.5, 0.0,
                0.0, 0.5, 0.0
            ];
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                size_of_val(&data).try_into().unwrap(), 
                data.as_ptr() as *const GLvoid, // cast to pointer, then pointer type to GLvoid/c_void
                gl::STATIC_DRAW
            );

            // Specify how to read array buffer data
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3*64, null());
            gl::EnableVertexAttribArray(0);
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