use glam::*;

use crate::{
    data, 
    engine::transform::get_transform, 
    global_state::*, 
    graphics::{
        shader::{Shader, Uniform, UniformType}, 
        texture::Texture, 
        vao::{VertexAttr, VAO}
    }
};

use super::Renderer;

pub struct CubesRenderer {
    pos_data: Vec<Vec3>
}

impl CubesRenderer {
    pub fn new(pos_data: Vec<Vec3>) -> Self {
        CubesRenderer { pos_data }
    }
}

impl Renderer for CubesRenderer {
    unsafe fn init(&mut self) -> GraphicsState {
        // load data
        let vertex_data: Vec<f32> = Vec::from(data::VERTEX_DATA);

        // Initialize VAO
        let vertex_attrs = vec![
            VertexAttr::new("Position".to_owned(), 3), 
            VertexAttr::new("Color".to_owned(), 0), // lol
            VertexAttr::new("Texture Coords".to_owned(), 2)
        ];
        let vao = VAO::new(vertex_data, None, vertex_attrs);
        gl::Enable(gl::DEPTH_TEST);
        vao.check_binding();

        // Initialize textures to texture units + amount to mix them
        let texture1 = unsafe { Texture::new("wall.jpg", gl::TEXTURE1) };
        let texture2 = unsafe { Texture::new("smileyface.png", gl::TEXTURE2) };
        let mut mix_amount = 0.2;

        // Initialize and use shader + add textures as uniforms
        let mut shader_program = unsafe { Shader::new("cubes", "cubes") };
        unsafe {
            shader_program.set_uniform(Uniform::new("texture1".to_owned(), UniformType::Int1(1)));
            shader_program.set_uniform(Uniform::new("texture2".to_owned(), UniformType::Int1(2)));
            shader_program.set_uniform(Uniform::new("mix_amount".to_owned(), UniformType::Float1(mix_amount)));
        }

        // Check for error before returning
        let err = gl::GetError();
        if err != 0 { 
            panic!("error during gl initialization: {err} ");
        } else { 
            println!("note: initialization success"); 
        }

        GraphicsState::new(vao, vec![shader_program])
    }

    
    unsafe fn render(&mut self, graphics_state: &mut GraphicsState, glfw_state: &mut GLFWState, game_state: &mut GameState) {
        // Set BG color + clear color and depth buffer(s)
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Clear(gl::DEPTH_BUFFER_BIT);

        // Modify and set transform as uniform for each cube, then draw
        for &pos in self.pos_data.iter() {
            let transform = get_transform(&game_state.camera, pos);

            graphics_state
                .shaders[0]
                .set_uniform(Uniform::new("transform".to_owned(), UniformType::Matrix4(transform)));

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
        
        // Check for any new errors
        let error = gl::GetError();
        if error != 0 {
            println!("error: {error}");
        }
    }
}