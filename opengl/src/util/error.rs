use gl::types::*;
use once_cell::unsync::Lazy;

// i will modify this in unsafe like a bastard
// this is necessary because the callback must be extern "system" fn. i can't use this signature in a closure, and thus can't move in such state
// however it should be ok as this is only used in the callback + its just singlethreaded toy program
static mut ERROR_STATE: Lazy<GLErrorState> = Lazy::new(|| GLErrorState::new());


// a small wrapper for calling gl::GetError anywhere
pub unsafe fn check_gl_error(cur_error: u32, debug_str: &str) -> u32 {
    let err = gl::GetError();
        if err != cur_error {
            if err != 0 {
                println!("error: {err} ({debug_str})");
            } else {
                println!("note: previous error resolved");
            }
        }
    err
}

// encapsulates debug state
pub struct GLErrorState {
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    message: String
}

impl GLErrorState {
    pub fn new() -> Self {
        GLErrorState {
            source: 0,
            type_: 0, 
            id: 0,
            severity: 0,
            message: String::new()
        }
    }
}

// callback for gl debug state
pub extern "system" fn gl_debug_callback(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    user_param: *mut GLvoid,
) {
    // Ignore non-significant error/warning codes
    if id == 131169 || id == 131185 || id == 131218 || id == 131204 {
        return;
    }

    // check if current error is same as previous error state (unsafely)
    unsafe {
        if ERROR_STATE.source == source
        || ERROR_STATE.type_ == type_
        || ERROR_STATE.id == id { 
            return;
        } 
        else {
            ERROR_STATE.source = source;
            ERROR_STATE.type_ = type_;
            ERROR_STATE.id = id;
        }
    }

    println!("---------------");
    println!("Debug message ({id}): {}", unsafe {
        std::ffi::CStr::from_ptr(message)
            .to_string_lossy()
            .to_string()
    });

    match source {
        gl::DEBUG_SOURCE_API => println!("Source: API"),
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => println!("Source: Window System"),
        gl::DEBUG_SOURCE_SHADER_COMPILER => println!("Source: Shader Compiler"),
        gl::DEBUG_SOURCE_THIRD_PARTY => println!("Source: Third Party"),
        gl::DEBUG_SOURCE_APPLICATION => println!("Source: Application"),
        gl::DEBUG_SOURCE_OTHER => println!("Source: Other"),
        _ => {}
    }

    match type_ {
        gl::DEBUG_TYPE_ERROR => println!("Type: Error"),
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => println!("Type: Deprecated Behaviour"),
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => println!("Type: Undefined Behaviour"),
        gl::DEBUG_TYPE_PORTABILITY => println!("Type: Portability"),
        gl::DEBUG_TYPE_PERFORMANCE => println!("Type: Performance"),
        gl::DEBUG_TYPE_MARKER => println!("Type: Marker"),
        gl::DEBUG_TYPE_PUSH_GROUP => println!("Type: Push Group"),
        gl::DEBUG_TYPE_POP_GROUP => println!("Type: Pop Group"),
        gl::DEBUG_TYPE_OTHER => println!("Type: Other"),
        _ => {}
    }

    match severity {
        gl::DEBUG_SEVERITY_HIGH => println!("Severity: high"),
        gl::DEBUG_SEVERITY_MEDIUM => println!("Severity: medium"),
        gl::DEBUG_SEVERITY_LOW => println!("Severity: low"),
        gl::DEBUG_SEVERITY_NOTIFICATION => println!("Severity: notification"),
        _ => {}
    }
}