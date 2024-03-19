mod init;
mod graphics;
mod data;
mod engine;
mod types;

use glam::*;
use glfw::CursorMode;
use std::env;
use std::f32::consts::PI;

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
    let mut glfw_state = glfw_init::init(width, height, use_old_ver);
    let mut graphics_state = unsafe { gl_init::init() };
    let mut game_state = game_init::init();

    glfw_state.window.set_all_polling(true);
    glfw_state.window.set_cursor_mode(CursorMode::Disabled);

    engine::run(graphics_state, glfw_state, game_state);
}