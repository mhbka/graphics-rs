mod init;
mod shader;

use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use glfw::fail_on_errors;
use gl::types::*;
use std::env;
use std::ffi::CString;
use std::ptr::null;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

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
    unsafe { init::gl_init(vertices.as_slice(), None, &mut vao, &mut vbo, &mut ebo) };

    let shader_program = unsafe { shader::create_and_link_shaders("test") };
    
    // Set viewport + framebuffer size callback fn
    unsafe { gl::Viewport(0, 0, width as i32, height as i32); }
    window.set_framebuffer_size_callback(|_, w, h| unsafe {gl::Viewport(0, 0, w, h)});

    let mut cur_error = unsafe { gl::GetError() };
    if cur_error != 0 { 
        panic!("error during init: {cur_error} ") 
    } else { 
        println!("note: initialized safely")
    }

    // MAIN LOOP - until window is closed
    while !window.should_close() {
        window.swap_buffers();

        unsafe { 
            // Set bg color
            gl::ClearColor(0.9, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT); 

            // Use shader + bind VAO
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);

            // Set the color
            let time = glfw.get_time() as f32;
            let green = (f32::sin(time)/2.0) + 0.5;
            let vertex_color_location = gl::GetUniformLocation(shader_program, CString::new("ourColor").unwrap().as_ptr());
            gl::Uniform4f(vertex_color_location, 0.0, green, 0.0, 1.0);

            // Draw triangles
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());

            let error = gl::GetError();
            if error!=0 && error!=cur_error { 
                println!("error: {error}") ;
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