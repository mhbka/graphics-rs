use crate::global_state::{GameState, GLFWState};
use glam::*;

const SENS: f32 = 0.1;

// Handles mouse movement
pub fn handle_mouse(glfw_state: &mut GLFWState, game_state: &mut GameState) {
    let cur_mouse_pos = glfw_state.window.get_cursor_pos();

    let offset = Vec2::new(
        (cur_mouse_pos.0 - game_state.last_mouse_pos.0) as f32 * SENS,
        (game_state.last_mouse_pos.1 - cur_mouse_pos.1) as f32 * SENS // something about y-coords being bottom to top??
    );

    game_state.camera.update_with_mouse_offset(offset); // Updates camera and thus view matrix during render

    game_state.last_mouse_pos = cur_mouse_pos;
}