use glfw::{Context, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, WindowEvent, WindowHint};
use glfw::fail_on_errors;
use crate::types::GLFWState;


/// Initializes GLFW context (including loading functions into `gl`),
/// and returns necessary objects.
pub fn init(width: u32, height: u32, use_old_ver: bool) -> GLFWState {

    // init glfw and set window hints as found in tutorial
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();    

    // if `use_old_ver`, switch to OpenGL 3.3
    if use_old_ver {
        glfw.window_hint(WindowHint::ContextVersionMajor(3));
        glfw.window_hint(WindowHint::ContextVersionMinor(3));
    }
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(width, height, "Hello", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_all_polling(true);
    
    // Load window's function pointers into `gl`
    gl::load_with(|s| {window.get_proc_address(s)});

    // Set viewport + framebuffer size callback fn
    unsafe { gl::Viewport(0, 0, width as i32, height as i32); }
    window.set_framebuffer_size_callback(|_, w, h| unsafe {gl::Viewport(0, 0, w, h)});

// TODO: If !`use_old_ver`, set a debug callback fn
    // Note: This functionality doesn't exist in OpenGL 3.30, which is used if `use_old_ver` is true.
    if !use_old_ver {

    }

    GLFWState::new(glfw, window, events)
}