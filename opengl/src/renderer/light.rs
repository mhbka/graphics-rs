use crate::{
    data, 
    engine::transform::{get_transform, get_transform_matrices}, 
    global_state::*, 
    graphics::{
        shader::{Shader, Uniform, UniformType}, 
        vao::{VertexAttr, VAO}
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
        let vertex_data: Vec<f32> = Vec::from(data::VERTEX_AND_NORMAL_DATA);

        // Initialize VAO
        let vertex_attrs = vec![
            VertexAttr::new("Position".to_owned(), 3),
            VertexAttr::new("Normal".to_owned(), 3),
        ];
        let vao = VAO::new(vertex_data, None, vertex_attrs);
        gl::Enable(gl::DEPTH_TEST);
        vao.check_binding();

        // Shader for lighting
        let mut lighting_shader = unsafe { Shader::new("lighting", "lighting") };
        unsafe {
                lighting_shader.set_uniform(Uniform::new("lightColor".to_owned(), UniformType::Float3(1.0, 1.0, 1.0)));

                lighting_shader.set_uniform(Uniform::new("material.ambient".to_owned(), UniformType::Float3(1.0, 0.5, 0.31)));
                lighting_shader.set_uniform(Uniform::new("material.diffuse".to_owned(), UniformType::Float3(1.0, 0.5, 0.31)));
                lighting_shader.set_uniform(Uniform::new("material.specular".to_owned(), UniformType::Float3(0.5, 0.5, 0.5)));
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

        // Light cube orbiting
        const r: f32 = 2.0;
        self.light_pos.x = r * glfw_state.glfw.get_time().cos() as f32;
        self.light_pos.y = r * glfw_state.glfw.get_time().sin() as f32;
        self.light_pos.z = r * glfw_state.glfw.get_time().cos() as f32;

        // Draw light cube using light shader
        graphics_state.shaders[1].use_program();
        let transform = get_transform(&game_state.camera, self.light_pos);

        graphics_state
            .shaders[1]
            .set_uniform(Uniform::new("transform".to_owned(), UniformType::Matrix4(transform)));

        gl::DrawArrays(gl::TRIANGLES, 0, 36);

        // Draw cubes using lighting shader
        graphics_state.shaders[0].use_program();
        for &pos in self.pos_data.iter() {
            let (projection, view, model) = get_transform_matrices(&game_state.camera, pos);

            let lighting_shader = &mut graphics_state.shaders[0];
            
            let light_pos_uniform = UniformType::Float3(self.light_pos.x, self.light_pos.y, self.light_pos.z);
            lighting_shader.set_uniform(Uniform::new("lightPos".to_owned(), light_pos_uniform));

            lighting_shader.set_uniform(Uniform::new("projection".to_owned(), UniformType::Matrix4(projection)));
            lighting_shader.set_uniform(Uniform::new("view".to_owned(), UniformType::Matrix4(view)));
            lighting_shader.set_uniform(Uniform::new("model".to_owned(), UniformType::Matrix4(model)));
            lighting_shader.set_uniform(Uniform::new("normTransform".to_owned(), UniformType::Matrix4((view*model).inverse().transpose())));

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // Check for any new errors
        let error = gl::GetError();
        if error != 0 {
            println!("error: {error}");
        }
    }
}