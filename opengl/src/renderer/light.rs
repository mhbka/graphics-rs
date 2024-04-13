use crate::{
    data, 
    engine::transform::{get_transform, get_transform_matrices}, 
    global_state::*, 
    graphics::{
        shader::{Shader, Uniform, UniformType},
        model_texture::{ModelTexture, ModelTextureType}, 
        vao::{VertexAttr, VAO}
    }
};
use glam::*;
use super::Renderer;

pub struct LightingRenderer {
    vao: VAO,
    pos_data: Vec<Vec3>
}

impl LightingRenderer {
    pub unsafe fn new(pos_data: Vec<Vec3>) -> Self {
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

        LightingRenderer { vao, pos_data }
    }
}

impl Renderer for LightingRenderer {
    unsafe fn init(&mut self) -> GraphicsState {
        // Shader for lighting
        let mut lighting_shader = unsafe { Shader::new("light/lighting", "light/lighting") };
        unsafe {
                // ModelTexture
                gl::ActiveTexture(gl::TEXTURE0);
                ModelTexture::new("container2.png", ModelTextureType::DIFFUSE);
                lighting_shader.set_uniform(Uniform::new("material.diffuse".to_owned(), UniformType::Int1(0)));

                // specular ModelTexture + shininess
                gl::ActiveTexture(gl::TEXTURE1);
                ModelTexture::new("container2_specular.png", ModelTextureType::SPECULAR);
                lighting_shader.set_uniform(Uniform::new("material.specular".to_owned(), UniformType::Int1(1)));
                lighting_shader.set_uniform(Uniform::new("material.shininess".to_owned(), UniformType::Float1(32.0)));
        };

        // Shader for the light itself
        let light_shader = unsafe { Shader::new("light/light_source", "light/light_source") };
       
        // Check for error before returning
        let err = gl::GetError();
        if err != 0 { 
            panic!("error during gl initialization: {err} ");
        } else { 
            println!("note: initialization success"); 
        }

        GraphicsState::new(vec![lighting_shader, light_shader])
    }


    unsafe fn render(&mut self, graphics_state: &mut GraphicsState, glfw_state: &mut GLFWState, game_state: &mut GameState) {
        // Set BG color + clear color and depth buffer(s)
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Clear(gl::DEPTH_BUFFER_BIT);

        // Draw and set uniforms for pointlights 
        graphics_state.shaders[1].use_program();
        LightingRenderer::draw_pointlights(game_state, glfw_state, graphics_state);

        // Draw and shade actual cubes using lighting shader
        graphics_state.shaders[0].use_program();
        for &pos in self.pos_data.iter() {
            let lighting_shader = &mut graphics_state.shaders[0];
            
            // light 
            LightingRenderer::set_flashlight_uniforms(lighting_shader, game_state);
            LightingRenderer::set_directional_light_uniforms(lighting_shader);

            // transforms
            LightingRenderer::set_transform_uniforms(lighting_shader, game_state, pos);

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // Check for any new errors
        let error = gl::GetError();
        if error != 0 { println!("error: {error}"); }
    }
}


// For setting light uniforms - you can use them in other renderers
impl LightingRenderer {
    pub unsafe fn draw_pointlights(game_state: &GameState, glfw_state: &GLFWState, graphics_state: &mut GraphicsState) {
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
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }

    pub unsafe fn set_pointlight_uniforms(graphics_state: &mut GraphicsState, transform: Mat4, pos: Vec3, color: Vec3, index: u32) {
        let pointlight_str = format!("pointlights[{index}]");

        let light_shader = &mut graphics_state.shaders[1];
        light_shader.use_program();
        light_shader.set_uniform(Uniform::new("transform".to_owned(), UniformType::Matrix4(transform)));
        light_shader.set_uniform(Uniform::new("fragColor".to_owned(), UniformType::Float3(color.x, color.y, color.z)));

        let lighting_shader = &mut graphics_state.shaders[0];
        lighting_shader.use_program();
        lighting_shader.set_uniform(Uniform::new(pointlight_str.clone() + ".position", UniformType::Float3(pos.x, pos.y, pos.z)));
        lighting_shader.set_uniform(Uniform::new(pointlight_str + ".color", UniformType::Float3(color.x, color.y, color.z)));
    }
    

    pub unsafe fn set_flashlight_uniforms(lighting_shader: &mut Shader, game_state: &GameState) { 
        let light_pos_uniform = UniformType::Float3(game_state.camera.position.x, game_state.camera.position.y, game_state.camera.position.z);
        lighting_shader.set_uniform(Uniform::new("spotlight.position".to_owned(), light_pos_uniform));

        let light_dir_uniform = UniformType::Float3(game_state.camera.front.x, game_state.camera.front.y, game_state.camera.front.z);
        lighting_shader.set_uniform(Uniform::new("spotlight.direction".to_owned(), light_dir_uniform));
        
        lighting_shader.set_uniform(Uniform::new("spotlight.color".to_owned(), UniformType::Float3(1.0, 1.0, 1.0)));

        lighting_shader.set_uniform(Uniform::new("spotlight.innerCutOffCos".to_owned(), UniformType::Float1(1.0_f32.to_radians().cos())));
        lighting_shader.set_uniform(Uniform::new("spotlight.outerCutOffCos".to_owned(), UniformType::Float1(12.5_f32.to_radians().cos())));

        lighting_shader.set_uniform(Uniform::new("spotlight.constant".to_owned(), UniformType::Float1(1.0)));
        lighting_shader.set_uniform(Uniform::new("spotlight.linear".to_owned(), UniformType::Float1(0.0022)));
        lighting_shader.set_uniform(Uniform::new("spotlight.quadratic".to_owned(), UniformType::Float1(0.0019)));
    }

    pub unsafe fn set_directional_light_uniforms(lighting_shader: &mut Shader) {
        lighting_shader.set_uniform(Uniform::new("dirlight.direction".to_owned(), UniformType::Float3(1.0, 1.0, 1.0)));
        lighting_shader.set_uniform(Uniform::new("dirlight.color".to_owned(), UniformType::Float3(1.0, 1.0, 1.0)));
    }

    pub unsafe fn set_transform_uniforms(lighting_shader: &mut Shader, game_state: &GameState, object_position: Vec3) { 
        let (projection, view, model) = get_transform_matrices(&game_state.camera, object_position, Vec3::ONE);
        
        lighting_shader.set_uniform(Uniform::new("projection".to_owned(), UniformType::Matrix4(projection)));
        lighting_shader.set_uniform(Uniform::new("view".to_owned(), UniformType::Matrix4(view)));
        lighting_shader.set_uniform(Uniform::new("model".to_owned(), UniformType::Matrix4(model)));
        lighting_shader.set_uniform(Uniform::new("normTransform".to_owned(), UniformType::Matrix4((model).inverse().transpose())));
    }
}