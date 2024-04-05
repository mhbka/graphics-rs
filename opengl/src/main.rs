mod init;
mod graphics;
mod renderer;
mod data;
mod engine;
mod global_state;

use graphics::model::Model;
use renderer::light::LightingRenderer;
use std::env;
use crate::init::{gl_init, glfw_init, game_init};

const ASSETS_LOCATION: &str = "C:/Users/khaimh/source/vsc/graphics-rs/opengl/assets";


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let use_old_ver = true;

    let (width, height) = (800, 600);

    let model = unsafe { Model::new(&format!("{ASSETS_LOCATION}/models/survival_backpack/Survival_BackPack_2.fbx")) };

    let mut renderer = unsafe { LightingRenderer::new(Vec::from(data::CUBE_POSITIONS)) };

    let mut glfw_state = glfw_init::init(width, height, use_old_ver);
    let mut graphics_state = unsafe { gl_init::init(&mut renderer) };
    let mut game_state = game_init::init();
 
    engine::run(
        &mut renderer, 
        &mut graphics_state, 
        &mut glfw_state, 
        &mut game_state
    );
}