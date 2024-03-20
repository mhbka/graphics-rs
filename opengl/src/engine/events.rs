mod keyboard;
mod mouse;
mod scroll;

use keyboard::handle_keyboard;
use mouse::handle_mouse;

use crate::global_state::{GLFWState, GameState};
use self::scroll::handle_scroll;


/// Polls and processes GLFW events.
pub fn poll_and_handle_events(glfw_state: &mut GLFWState, game_state: &mut GameState) {

    glfw_state.glfw.poll_events();

    // These events can be directly polled for
    handle_keyboard(glfw_state, game_state);
    handle_mouse(glfw_state, game_state);

    // These can't; we need to flush and check all events for them
    for event in glfw::flush_messages(&glfw_state.events) {
        handle_scroll(game_state, event.1);
    }
}