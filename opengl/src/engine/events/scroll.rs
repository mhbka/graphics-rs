use glfw::WindowEvent;

use crate::types::GameState;

pub fn handle_scroll(game_state: &mut GameState, event: WindowEvent) {
    if let WindowEvent::Scroll(xoffset, yoffset) = event {
        let fov = &mut game_state.camera.fov;

        *fov -= yoffset as f32;

        if *fov < 1.0 { *fov = 1.0; }
        else if *fov > 45.0 { *fov = 45.0; }
    }
}