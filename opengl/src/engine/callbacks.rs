use std::cell::RefCell;
use glam::*;
use glfw::{Action, Key, Modifiers, Scancode, Window};
use crate::types::{GLFWState, GraphicsState, GameState};


/// Sets keyboard, mouse and scroll callbacks.
pub fn set_callbacks(graphics_cell: RefCell<GraphicsState>, glfw_cell: RefCell<GLFWState>, game_cell: RefCell<GameState>) {
    

    // Mouse callback; modifies Camera state based on change in mouse position.
    glfw_cell
        .borrow_mut()
        .window
        .set_cursor_pos_callback(
            move |window: &mut Window, xpos: f64, ypos: f64| {
                const SENS: f64 = 0.1;
                
                let mut game_state = game_cell.borrow_mut();
                let (last_xpos, last_ypos) = game_state.last_mouse_pos;

                let x_offset = (xpos - last_xpos) * SENS;
                let y_offset = (last_ypos - ypos) * SENS; // reversed since y-coords range from bottom to top
                let offset = Vec2::new(x_offset as f32, y_offset as f32);

                game_state
                    .camera
                    .update_with_mouse_offset(offset);

                game_state.last_mouse_pos = (xpos, ypos);
        });

    // Keyboard callback
    glfw_cell
        .borrow_mut()
        .window
        .set_key_callback(
            move |window: &mut Window, key: Key, _: Scancode, action: Action, modifiers: Modifiers| {
                let mut game_state = game_cell.borrow_mut();

                match (key, action) {
                    (Key::W, Action::Press | Action::Release) => {
                        
                    },
                    _ => {}
                }
        });
    
    // Scroll callback
    glfw_cell
        .borrow_mut()
        .window
        .set_scroll_callback(move |window: &mut Window, xpos: f64, ypos: f64| {

        });
}