use std::ffi::{CStr, CString};
use std::ptr::{null, null_mut};
use std::fs::read_to_string;
use gl::types;


// main shader entrypoint
// compiles vertex and fragment shaders, links them to shader program object, and returns program ID
pub unsafe fn create_and_link_shaders(shader_name: &str) -> u32 {
    // init a shader program
    let shader_program = gl::CreateProgram();

    // fetch vert and fragment shader from files
    let vertex_shader_source = 
        read_to_string(&format!("shaders/{shader_name}.vert"))
        .expect(&format!("Can't read {shader_name} vertex shader from file."));
    let fragment_shader_source = 
        read_to_string(&format!("shaders/{shader_name}.frag"))
        .expect(&format!("Can't read {shader_name} fragment shader from file."));

    // compile shaders and attach to program
    let vertex_shader = compile_shader(vertex_shader_source.as_str(), gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(fragment_shader_source.as_str(), gl::FRAGMENT_SHADER);
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);

    // link program, then check if successful; if not, panic with info log
    gl::LinkProgram(shader_program);
    let mut success = 1;
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success as *mut i32);
    if success !=1 {
        if success <= 0 {
            let mut info_vec = vec![0; 512];
            gl::GetProgramInfoLog(shader_program, 512,null_mut(), info_vec.as_mut_ptr());
            let info_log = CStr::from_ptr(info_vec.as_mut_ptr()).to_str().unwrap();
            panic!("error: shader compilation failed \n {info_log}");
        } else {
            println!("note: shader compilation succeeded");
        }
    }
    
    // delete shaders since we've already linked them
    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    // return the shader program ID
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