mod keyboard;
mod mouse;
mod scroll;

use keyboard::handle_keyboard;
use mouse::handle_mouse;

use crate::types::{GLFWState, GameState};


/// Polls and processes GLFW events.
pub fn poll_and_handle_events(glfw_state: &mut GLFWState, game_state: &mut GameState) {

    glfw_state.glfw.poll_events();

    handle_keyboard(glfw_state, game_state);
    handle_mouse(glfw_state, game_state);
}