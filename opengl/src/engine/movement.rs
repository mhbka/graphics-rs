use glam::*;
use glfw::{Action, Key, Window};
use crate::types::GLFWState;
use super::camera::Camera;


// Handles WASD input on each game loop
pub fn handle_wasd_movement(glfw_state: &mut GLFWState, camera: &mut Camera, camera_speed: f32) {
    if let (Action::Press | Action::Repeat) = glfw_state.window.get_key(Key::W) {
        camera.position += camera.front * camera_speed;
    }

    if let (Action::Press | Action::Repeat) = glfw_state.window.get_key(Key::S) {
        camera.position -= camera.front * camera_speed;
    }

    if let (Action::Press | Action::Repeat) = glfw_state.window.get_key(Key::A) {
        camera.position -= camera.front
                    .cross(camera.up)
                    .normalize() 
                    * camera_speed;
    }

    if let (Action::Press | Action::Repeat) = glfw_state.window.get_key(Key::D) {
        camera.position += camera.front
                    .cross(camera.up)
                    .normalize() 
                    * camera_speed;
    }
}


// Applies mouse movement into camera pitch and yaw.
pub fn handle_mouse_movement(camera: &mut Camera, cur_pos: Vec2, last_pos: &mut Vec2) {
    const SENS: f32 = 0.1;
    let x_offset = (cur_pos.x - last_pos.x) * SENS;
    let y_offset = (last_pos.y - cur_pos.y) * SENS; // reversed since y-coords range from bottom to top
    let offset = Vec2::new(x_offset, y_offset);
    
    camera.update_with_mouse_offset(offset);
    
    *last_pos = cur_pos;
}