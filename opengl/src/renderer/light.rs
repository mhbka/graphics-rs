use crate::{
    data, 
    engine::transform::{get_transform, get_transform_matrices}, 
    global_state::*, 
    graphics::{
        shader::{Shader, Uniform, UniformType}, texture::Texture, vao::{VertexAttr, VAO}
    }
};
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
        // load vertex data
        let vertex_data: Vec<f32> = Vec::from(data::VERTEX_NORM_TEX_DATA);

        // Initialize VAO
        let vertex_attrs = vec![
            VertexAttr::new("Position".to_owned(), 3),
            VertexAttr::new("Normal".to_owned(), 3),
            VertexAttr::new("Tex Coords".to_owned(), 2),
        ];
        let vao = VAO::new(vertex_data, None, vertex_attrs);
        gl::Enable(gl::DEPTH_TEST);
        vao.check_binding();

        // Shader for lighting
        let mut lighting_shader = unsafe { Shader::new("lighting", "lighting") };
        unsafe {
                lighting_shader.set_uniform(Uniform::new("lightColor".to_owned(), UniformType::Float3(1.0, 1.0, 1.0)));

                // texture
                Texture::new("container2.png", gl::TEXTURE0);
                lighting_shader.set_uniform(Uniform::new("material.diffuse".to_owned(), UniformType::Int1(0)));

                // specular texture + shininess
                Texture::new("container2_specular.png", gl::TEXTURE1);
                lighting_shader.set_uniform(Uniform::new("material.specular".to_owned(), UniformType::Int1(1)));
                lighting_shader.set_uniform(Uniform::new("material.shininess".to_owned(), UniformType::Float1(32.0)));
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

        // Draw light cube using light shader
        /* 
        graphics_state.shaders[1].use_program();
        let transform = get_transform(&game_state.camera, self.light_pos, 0.1 * Vec3::ONE);

        graphics_state
            .shaders[1]
            .set_uniform(Uniform::new("transform".to_owned(), UniformType::Matrix4(transform)));

        gl::DrawArrays(gl::TRIANGLES, 0, 36);
        */

        // Draw cubes using lighting shader
        graphics_state.shaders[0].use_program();
        for &pos in self.pos_data.iter() {
            let (projection, view, model) = get_transform_matrices(&game_state.camera, pos, Vec3::ONE);

            let lighting_shader = &mut graphics_state.shaders[0];
            
            // light 
            let light_pos_uniform = UniformType::Float3(game_state.camera.position.x, game_state.camera.position.y, game_state.camera.position.z);
            lighting_shader.set_uniform(Uniform::new("light.position".to_owned(), light_pos_uniform));

            let light_dir_uniform = UniformType::Float3(game_state.camera.front.x, game_state.camera.front.y, game_state.camera.front.z);
            lighting_shader.set_uniform(Uniform::new("light.direction".to_owned(), light_dir_uniform));

            lighting_shader.set_uniform(Uniform::new("light.cutOffCos".to_owned(), UniformType::Float1(20.0_f32.to_radians().cos())));

            lighting_shader.set_uniform(Uniform::new("light.constant".to_owned(), UniformType::Float1(1.0)));
            lighting_shader.set_uniform(Uniform::new("light.linear".to_owned(), UniformType::Float1(0.0022)));
            lighting_shader.set_uniform(Uniform::new("light.quadratic".to_owned(), UniformType::Float1(0.0019)));

            // transforms
            lighting_shader.set_uniform(Uniform::new("projection".to_owned(), UniformType::Matrix4(projection)));
            lighting_shader.set_uniform(Uniform::new("view".to_owned(), UniformType::Matrix4(view)));
            lighting_shader.set_uniform(Uniform::new("model".to_owned(), UniformType::Matrix4(model)));
            lighting_shader.set_uniform(Uniform::new("normTransform".to_owned(), UniformType::Matrix4((model).inverse().transpose())));

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // Check for any new errors
        let error = gl::GetError();
        if error != 0 { println!("error: {error}"); }
    }
}