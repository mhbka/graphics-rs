use glfw::{Action, Key};
use crate::{engine::camera::Camera, global_state::{GLFWState, GameState}};

const SPEED: f32 = 2.5;

// Entrypoint to all keyboard event handlers
pub fn handle_keyboard(glfw_state: &mut GLFWState, game_state: &mut GameState) {
    // compute delta time and set a new last_frame_time
    let cur_frame_time = glfw_state.glfw.get_time();
    let delta_time = cur_frame_time - game_state.last_frame_time;
    game_state.last_frame_time = cur_frame_time;

    handle_wasd_movement(glfw_state, &mut game_state.camera, SPEED * delta_time as f32); 
    handle_others(glfw_state);
}

// Handles WASD keys for movement
fn handle_wasd_movement(glfw_state: &mut GLFWState, camera: &mut Camera, camera_speed: f32) {
    if let Action::Press | Action::Repeat = glfw_state.window.get_key(Key::W) {
        camera.position += camera.front * camera_speed;
    }

    if let Action::Press | Action::Repeat = glfw_state.window.get_key(Key::S) {
        camera.position -= camera.front * camera_speed;
    }

    if let Action::Press | Action::Repeat = glfw_state.window.get_key(Key::A) {
        camera.position -= camera.front
                    .cross(camera.up)
                    .normalize() 
                    * camera_speed;
    }

    if let Action::Press | Action::Repeat = glfw_state.window.get_key(Key::D) {
        camera.position += camera.front
                    .cross(camera.up)
                    .normalize() 
                    * camera_speed;
    }
}

// Any other keys
fn handle_others(glfw_state: &mut GLFWState) {
    if let Action::Press = glfw_state.window.get_key(Key::Escape) {
        glfw_state.window.set_should_close(true);
    }
}