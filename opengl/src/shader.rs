use std::ffi::{CStr, CString};
use std::ptr::{null, null_mut};
use std::fs::read_to_string;
use gl::types;


/// Wrapper struct for a shader program; encapsulates and tracks/sets shader program state.
#[derive(Debug)]
pub struct Shader {
    program: u32,
    vertex_shader: u32,
    fragment_shader: u32,
    uniforms: Vec<Uniform>
}


/// Wrapper struct and enum for different types of shader uniforms.
#[derive(Debug)]
pub struct Uniform {
    name: String,
    uniform_type: UniformType
}

#[derive(Debug, Clone, Copy)]
pub enum UniformType {
    Float4(f32, f32, f32, f32),
    // TODO: Add other types as they become necessary (including any match statements using Uniform)
}


/// Wrapper implementations for OpenGL shaders.
impl Shader {
    /// Generates a new shader program with the specified vertex and fragment shaders.
    pub unsafe fn new(shader_name: &str) -> Self {
        let shader_program = gl::CreateProgram();

        let vertex_shader_source = read_to_string(&format!("shaders/{shader_name}.vert"))
            .expect(&format!("Can't read {shader_name} vertex shader from file."));
        let fragment_shader_source = read_to_string(&format!("shaders/{shader_name}.frag"))
            .expect(&format!("Can't read {shader_name} fragment shader from file."));

        let vertex_shader = Shader::compile_shader(vertex_shader_source.as_str(), gl::VERTEX_SHADER);
        let fragment_shader = Shader::compile_shader(fragment_shader_source.as_str(), gl::FRAGMENT_SHADER);
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
    
        gl::LinkProgram(shader_program);
        let mut success = 1;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success as *mut i32);
        if success != 1 {
            if success <= 0 {
                let mut info_vec = vec![0; 512];
                gl::GetProgramInfoLog(shader_program, 512,null_mut(), info_vec.as_mut_ptr());
                let info_log = CStr::from_ptr(info_vec.as_mut_ptr()).to_str().unwrap();
                panic!("error: shader compilation failed \n {info_log}");
            } else {
                println!("note: shader compilation succeeded");
            }
        }
        
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    
        Shader {
            program: shader_program,
            vertex_shader,
            fragment_shader,
            uniforms: Vec::new()
        }
    }

    /// Use the shader program.
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.program);
    }

    /// Adds a new uniform to the shader program.
    pub unsafe fn add_uniform(&mut self, uniform_name: &str, uniform: Uniform) {
        let uniform_location = gl::GetUniformLocation(
            self.program, 
            CString::new(uniform_name)
                .unwrap()
                .as_ptr() as *const i8
        );

        if uniform_location == -1 { 
            panic!("Uniform {uniform_name} not found for shader program {}", self.program); 
        }

        match uniform.uniform_type {
            UniformType::Float4(x, y, z, w) => gl::Uniform4f(uniform_location, x, y, z, w),
            other => println!("Unaccounted Uniform enum: {other:?}")
        }

        self.uniforms.push(uniform);
    }
}


/// Private implementation fns
impl Shader {
    unsafe fn compile_shader(shader_source: &str, shader_type: types::GLenum) -> u32 {
        let c_str = CString::new(shader_source).unwrap();
        let c_str_ptr: *const *const i8 = &c_str.as_ptr();
        let shader = gl::CreateShader(shader_type);

        gl::ShaderSource(shader, 1, c_str_ptr, null());
        gl::CompileShader(shader);

        Shader::check_compilation_status(shader);
    
        shader
    }

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
}