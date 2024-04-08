use crate::engine::transform::get_transform;
use crate::graphics::model::Model;
use crate::global_state::*;
use crate::graphics::shader::{Shader, Uniform, UniformType};
use super::Renderer;
use glam::*;

pub struct ModelRenderer {
    model: Model
}

impl ModelRenderer {
    pub fn new(model: Model) -> Self {
        ModelRenderer { model }
    }
}

 
impl Renderer for ModelRenderer {
    unsafe fn init(&mut self) -> GraphicsState {
        let shader = Shader::new("model/model", "model/model");
        GraphicsState::new(vec![shader])
    }

    unsafe fn render(&mut self, graphics_state: &mut GraphicsState, glfw_state: &mut GLFWState, game_state: &mut GameState) {
        // Set BG color + clear color and depth buffer(s)
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Clear(gl::DEPTH_BUFFER_BIT);

        let transform = get_transform(&game_state.camera, Vec3::ONE, Vec3::ONE);
        graphics_state.shaders[0].use_program();
        graphics_state.shaders[0].set_uniform(
            Uniform::new("transform".to_owned(), UniformType::Matrix4(transform))
        );

        self.model.draw(&mut graphics_state.shaders[0]);
    }
}
