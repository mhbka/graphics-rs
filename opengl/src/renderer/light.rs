use crate::{data, engine::transform::get_transform, global_state::{GLFWState, GameState, GraphicsState}, graphics::{shader::{Shader, Uniform, UniformType}, vao::{VertexAttr, VAO}}};
use glam::*;
use super::Renderer;

pub struct LightingRenderer {
    pos_data: Vec<Vec3>,
    light_pos: Vec3
}

impl LightingRenderer {
    pub fn new(pos_data: Vec<Vec3>, light_pos: Vec3) -> Self {
        LightingRenderer { pos_data, light_pos }
    }
}

impl Renderer for LightingRenderer {
    unsafe fn init(&mut self) -> GraphicsState {
        // load data
        let vertex_data: Vec<f32> = Vec::from(data::VERTEX_DATA);

        // Initialize VAO
        let vertex_attrs = vec![VertexAttr::new("Position".to_owned(), 3)];
        let vao = VAO::new(vertex_data, None, vertex_attrs);
        gl::Enable(gl::DEPTH_TEST);
        vao.check_binding();

        // Shader for lighting
        let mut lighting_shader = unsafe { Shader::new("lighting", "lighting") };
        unsafe {
                lighting_shader.set_uniform(Uniform::new("objectColor".to_owned(), UniformType::Float3(1.0, 0.5, 0.31)));
                lighting_shader.set_uniform(Uniform::new("lightColor".to_owned(), UniformType::Float3(1.0, 1.0, 1.0)));
        };

        // Shader for the light itself
        let light_shader = unsafe { Shader::new("light_source", "light_source") };
       
        // Check for error before returning
        let err = gl::GetError();
        if err != 0 { 
            panic!("error during gl initialization: {err} ");
        } else { 
            println!("note: initialization success"); 
        }

        GraphicsState::new(vao, vec![lighting_shader, light_shader])
    }


    unsafe fn render(&mut self, graphics_state: &mut GraphicsState, glfw_state: &mut GLFWState, game_state: &mut GameState) {
        // Set BG color + clear color and depth buffer(s)
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Clear(gl::DEPTH_BUFFER_BIT);

        // Draw cubes using lighting shader
        graphics_state.shaders[0].use_program();
        for &pos in self.pos_data.iter() {
            let transform = get_transform(&game_state.camera, pos);

            graphics_state
                .shaders[0]
                .set_uniform(Uniform::new("transform".to_owned(), UniformType::Matrix4(transform)));

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // Draw light cube
        graphics_state.shaders[1].use_program();
        let transform = get_transform(&game_state.camera, self.light_pos);

        graphics_state
            .shaders[0]
            .set_uniform(Uniform::new("transform".to_owned(), UniformType::Matrix4(transform)));

        gl::DrawArrays(gl::TRIANGLES, 0, 36);

        
        // Check for any new errors
        let error = gl::GetError();
        if error != 0 {
            println!("error: {error}");
        }
    }
}