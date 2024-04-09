use crate::engine::transform::get_transform;
use crate::graphics::model::Model;
use crate::global_state::*;
use crate::graphics::shader::Shader;
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

        // Draw light cubes for point lights, using light shader
        for i in 0..5 {
            let time = glfw_state.glfw.get_time() as f32;
            let pos =  i as f32 * match i {
                0 => Vec3::new(time.sin(), time.sin(), 0.0),
                1 => Vec3::new(time.cos(), time.cos(), 0.0),
                2 => Vec3::new(time.sin(), 0.0, time.sin()),
                3 => Vec3::new(time.cos(), 0.0, time.cos()),
                4 => Vec3::new(time.sin(), time.sin(), time.cos()),
                _ => panic!(),
            };
            let color = match i {
                0 => Vec3::new(1.0, 0.0, 0.0),
                1 => Vec3::new(0.0, 1.0, 0.0),
                2 => Vec3::new(0.0, 0.0, 1.0),
                3 => Vec3::new(1.0, 0.0, 1.0),
                4 => Vec3::new(1.0, 1.0, 0.0),
                _ => panic!(),
            };

            let transform = get_transform(&game_state.camera, pos, 0.1 * Vec3::ONE);

            LightingRenderer::set_pointlight_uniforms(graphics_state, transform, pos, color, i); //also does shading for actual cubes

            graphics_state.shaders[1].use_program();
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // Set some uniforms for model itself, then draw
        let model_shader = &mut graphics_state.shaders[0];
        model_shader.use_program();

        LightingRenderer::set_flashlight_uniforms(model_shader, game_state);
        LightingRenderer::set_directional_light_uniforms(model_shader);
        LightingRenderer::set_transform_uniforms(model_shader, game_state, object_pos);

        self.model.draw(model_shader);

        // Check for change in error state
        let err = gl::GetError();
        if err != self.cur_err {
            if err != 0 {
                println!("error: mesh drawing ({err})");
            } else {
                println!("note: previous error resolved");
            }
            self.cur_err = err;
        }
    }
}
