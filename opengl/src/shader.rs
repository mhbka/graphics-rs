use std::ffi::{CStr, CString};
use std::ptr::{null, null_mut};

use gl::types;

const VERTEX_SHADER_SOURCE: &str = "
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main()
    {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
";

const FRAGMENT_SHADER_SOURCE: &str = "
    #version 330 core
    out vec4 FragColor;

    void main()
    {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    } 
";


// main shader entrypoint
// compiles shaders and links them to shader program object
pub unsafe fn create_and_link_shaders() -> u32 {
    let shader_program = gl::CreateProgram();
    let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);

    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);
    
    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    shader_program
}


// compiles a shader, given its source code and a shader type
// if it fails to compile, info will be printed out but the shader will still be returned
unsafe fn compile_shader(shader_source: &str, shader_type: types::GLenum) -> u32 {
    let c_str = CString::new(shader_source).unwrap();
    let c_str_ptr: *const *const i8 = &c_str.as_ptr();
    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(shader, 1, c_str_ptr, null());
    gl::CompileShader(shader);

    check_compilation_status(shader);

    shader
}


// checks status of compilation
// if failed, write info to a CString then print it out
unsafe fn check_compilation_status(shader: u32) -> bool {
    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success as *mut i32);   

    if success <= 0 {
        let mut info_vec = vec![0; 512];
        gl::GetShaderInfoLog(shader, 512,null_mut(), info_vec.as_mut_ptr());
        let info_log = CStr::from_ptr(info_vec.as_mut_ptr()).to_str().unwrap();
        println!("error: shader compilation failed \n {info_log}");
        false
    } else {
        println!("note: shader compilation succeeded");
        true
    }
}