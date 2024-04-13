use std::cell::RefCell;
use std::rc::Rc;

use gl::types::*;
use glfw::{Context, CursorMode, OpenGlProfileHint, WindowHint};
use glfw::fail_on_errors;
use crate::global_state::GLFWState;
use crate::util::error::{GLErrorState, gl_debug_callback};


/// Initializes GLFW context (including loading functions into `gl`),
/// and returns necessary objects.
pub fn init(width: u32, height: u32, use_old_ver: bool) -> GLFWState {

    // init glfw and set window hints as found in tutorial
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();    

    // if use_old_ver, switch to OpenGL 3.3; else 4.3
    if use_old_ver {
        glfw.window_hint(WindowHint::ContextVersionMajor(3));
        glfw.window_hint(WindowHint::ContextVersionMinor(3));
    }
    else {
        glfw.window_hint(WindowHint::ContextVersionMajor(4));
        glfw.window_hint(WindowHint::ContextVersionMinor(3));
        glfw.window_hint(WindowHint::OpenGlDebugContext(true));
    }
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(width, height, "Hello", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current and poll for everything + disable cursor
    window.make_current();
    window.set_all_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);
    
    // Load opengl function pointers
    gl::load_with(|s| {window.get_proc_address(s)});
    println!("note: successfully loaded gl");

    // Set viewport + framebuffer size callback
    unsafe { gl::Viewport(0, 0, width as i32, height as i32); }
    window.set_framebuffer_size_callback(|_, w, h| 
        unsafe { gl::Viewport(0, 0, w, h) }
    );

    // Set debug output callback if !use_old_ver
    if !use_old_ver {
        let error_state = Rc::new(RefCell::new(GLErrorState::new()));
        unsafe {
        gl::DebugMessageCallback(Some(gl_debug_callback), 0 as *const GLvoid
        );
    }
    }   

    GLFWState::new(glfw, window, events)
}