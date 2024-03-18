pub mod camera; // used in main for initialization
mod callbacks;
mod events;
mod transform;
mod movement;


use std::{cell::RefCell, rc::Rc};

use glam::*;
use glfw::{Context, CursorMode, Window};
use crate::{
    data::CUBE_POSITIONS, 
    types::{GLFWState, GraphicsState, GameState}, 
    Uniform, 
    UniformType
};
use self::{
    events::handle_events, 
    transform::get_transform,
    callbacks::set_callbacks
};



// The main render/event loop of the program
pub fn run(graphics_state: GraphicsState, glfw_state: GLFWState, game_state: GameState) {

    // Wrap in RefCell as callbacks require static references
    // Inshallah it will not blow up at runtime
    let graphics_cell = Rc::new(RefCell::new(graphics_state));
    let glfw_cell = Rc::new(RefCell::new(glfw_state));
    let game_cell = Rc::new(RefCell::new(game_state));
    set_callbacks(graphics_cell, glfw_cell, game_cell);

    let pos_data = Vec::from(CUBE_POSITIONS);

    let mut cur_error = 0;

    while !glfw_state.window.should_close() {
        glfw_state
            .window
            .swap_buffers();

        unsafe { 
            // Set BG color + clear color and depth buffer(s)
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);

            // Modify and set transform as uniform for each cube, then draw
            for &pos in pos_data.iter() {
                let transform = get_transform(&game_state.camera, 45.0, pos);

                graphics_state
                    .shader
                    .set_uniform(Uniform::new("transform".to_owned(), UniformType::Matrix4(transform)));

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            
            // Check for any new errors
            let error = gl::GetError();
            if error != cur_error {
                println!("error: {error}");
                cur_error = error;
            }
        };

        // Poll and handle events
        // handle_events(&mut glfw_state, &mut camera, &mut last_frame_time, &mut last_mouse_pos);
    }
}