use crate::{data, global_state::{GLFWState, GameState, GraphicsState}, graphics::{shader::{Shader, Uniform, UniformType}, vao::{VertexAttr, VAO}}};

use super::Renderer;

pub struct LightingRenderer {

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

       // Initialize and use shader + add textures as uniforms
       let mut shader_program = unsafe { Shader::new("light_cube") };
       unsafe {
           shader_program.set_uniform(Uniform::new("objectColor".to_owned(), UniformType::Float3(1.0, 0.5, 0.31)));
           shader_program.set_uniform(Uniform::new("lightColor".to_owned(), UniformType::Float3(1.0, 1.0, 1.0)));
       };
       
    }


    unsafe fn render(&mut self, graphics_state: &mut GraphicsState, glfw_state: &mut GLFWState, game_state: &mut GameState) {
        todo!()
    }
}