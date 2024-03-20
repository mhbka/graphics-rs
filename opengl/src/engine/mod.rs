pub mod camera;
pub mod transform;
mod events;

use glfw::Context;
use crate::{
    data::CUBE_POSITIONS, global_state::{GLFWState, GameState, GraphicsState}, renderer::Renderer, Uniform, UniformType
};
use self::{
    events::poll_and_handle_events, 
    transform::get_transform
};


// The main render/event loop of the program
pub fn run<T: Renderer>(renderer: &mut T, graphics_state: &mut GraphicsState, glfw_state: &mut GLFWState, game_state: &mut GameState) {
    while !glfw_state.window.should_close() {
        glfw_state.window.swap_buffers();

        unsafe { 
            renderer.render(graphics_state, glfw_state, game_state);
        };
        
        poll_and_handle_events(glfw_state, game_state);
    }
}