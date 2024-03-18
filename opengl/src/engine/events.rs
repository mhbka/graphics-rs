use glam::*;
use glfw::{Action, Key, WindowEvent};
use crate::types::GLFWState;
use super::{movement::{handle_mouse_movement, handle_wasd_movement}, transform::Camera};

const SPEED: f32 = 2.5;

/// Polls and processes GLFW events.
pub fn handle_events(glfw_state: &mut GLFWState, camera: &mut Camera, last_frame_time: &mut f64, last_mouse_pos: &mut Vec2) {
    // compute delta time and set a new last_frame_time
    let cur_frame_time = glfw_state.glfw.get_time();
    let delta_time = cur_frame_time - *last_frame_time;
    *last_frame_time = cur_frame_time; 

    glfw_state.glfw.poll_events();

    // quit on escape
    if let Action::Press = glfw_state.window.get_key(Key::Escape) {
        glfw_state.window.set_should_close(true);
    }

    // handling WASD movement
    handle_wasd_movement(glfw_state, camera, SPEED * delta_time as f32);

    // handle mouse movement
    let (xpos, ypos) = glfw_state.window.get_cursor_pos();
    let cur_mouse_pos = Vec2::new(xpos as f32, ypos as f32);
    handle_mouse_movement(camera, cur_mouse_pos, last_mouse_pos);
}