use crate::engine::transform::get_transform;
use crate::graphics::model::Model;
use crate::global_state::*;
use crate::graphics::shader::Shader;
use crate::util::error::check_gl_error;
use super::light::LightingRenderer;
use super::Renderer;
use glam::*;

pub struct ModelRenderer {
    cur_err: u32,
    model: Model
}

impl ModelRenderer {
    pub fn new(model: Model) -> Self {
        ModelRenderer { cur_err: 0, model }
    }
}

 
impl Renderer for ModelRenderer {
    unsafe fn init(&mut self) -> GraphicsState {
        let model_shader = Shader::new("model/model", "model/model");
        let light_shader = Shader::new("light/light_source", "light/light_source");
        GraphicsState::new(vec![model_shader, light_shader])
    }

    unsafe fn render(&mut self, graphics_state: &mut GraphicsState, glfw_state: &mut GLFWState, game_state: &mut GameState) {
        // Set BG color + clear color and depth buffer(s)
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Clear(gl::DEPTH_BUFFER_BIT);
        
        // object pos
        let object_pos = Vec3::new(0.0, 0.0, 0.0);

        // set pointlight uniforms and draw them
        graphics_state.shaders[1].use_program();
        LightingRenderer::draw_pointlights(game_state, glfw_state, graphics_state);

        // Set some uniforms for model itself, then draw
        let model_shader = &mut graphics_state.shaders[0];
        model_shader.use_program();
        LightingRenderer::set_flashlight_uniforms(model_shader, game_state);
        LightingRenderer::set_directional_light_uniforms(model_shader);
        LightingRenderer::set_transform_uniforms(model_shader, game_state, object_pos);

        self.model.draw(model_shader);

        self.cur_err = check_gl_error(self.cur_err, "model drawing");
    }
}
