mod init;
mod graphics;
mod renderer;
mod data;
mod engine;
mod global_state;
mod util;

use graphics::model::Model;
use renderer::{light::LightingRenderer, model::ModelRenderer};
use std::env;
use crate::init::{gl_init, glfw_init, game_init};


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let use_old_ver = false;

    let (width, height) = (800, 600);

    let mut glfw_state = glfw_init::init(width, height, use_old_ver);
    let mut game_state = game_init::init();

    let model = unsafe { Model::new("assets/models/survival_backpack", "backpack.obj") };
    //let mut renderer = unsafe { LightingRenderer::new(Vec::from(data::CUBE_POSITIONS)) };
    let mut renderer =  ModelRenderer::new(model); 
    let mut graphics_state = unsafe { gl_init::init(&mut renderer) };
    
 
    engine::run(
        &mut renderer, 
        &mut graphics_state, 
        &mut glfw_state, 
        &mut game_state
    );
}