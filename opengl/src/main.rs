mod init;
mod graphics;
mod renderer;
mod data;
mod engine;
mod global_state;

use data::CUBE_POSITIONS;
use glam::Vec3;
use glfw::CursorMode;
use renderer::cubes::CubesRenderer;
use renderer::light::LightingRenderer;
use std::env;
use crate::init::{gl_init, glfw_init, game_init};
use crate::graphics::{
    shader::{Shader, Uniform, UniformType},
    vao::{VAO, VertexAttr},
    texture::Texture
};


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let use_old_ver = true;

    let (width, height) = (800, 600);

    // let mut renderer = CubesRenderer::new(vec![CUBE_POSITIONS]);

    let mut renderer = LightingRenderer::new(
        vec![Vec3::new(1.2, 1.0, 1.0)],
        Vec3::new(1.2, 1.0, 2.0)
    );

    let mut glfw_state = glfw_init::init(width, height, use_old_ver);
    let mut graphics_state = unsafe { gl_init::init(&mut renderer) };
    let mut game_state = game_init::init();

    glfw_state.window.set_all_polling(true);
    glfw_state.window.set_cursor_mode(CursorMode::Disabled);

    engine::run(
        &mut renderer, 
        &mut graphics_state, 
        &mut glfw_state, 
        &mut game_state
    );
}