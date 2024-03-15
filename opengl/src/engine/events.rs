use glfw::{Action, Key, WindowEvent};
use crate::types::GLFWState;


/// Polls and processes GLFW events.
pub fn handle_events(glfw_state: &mut GLFWState) {
    glfw_state.glfw.poll_events();
    for (_, event) in glfw::flush_messages(&glfw_state.events) {
        // println!("{:?}", event);
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                glfw_state.window.set_should_close(true)
            },
            WindowEvent::Key(Key::Up, _, Action::Press | Action::Repeat, _) => {
               
            },
            _ => {},
        }
    }
}