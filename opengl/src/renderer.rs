use crate::global_state::{GLFWState, GameState, GraphicsState};

pub mod light;


/// A Render trait consists of 2 stages: initialization and render.
/// Initialization is called within gl_init, while render is called on each game loop.
/// Thus any kind of rendering can be done through this implementation.
pub trait Renderer {
    unsafe fn init(&mut self) -> GraphicsState;
    unsafe fn render(&mut self, graphics_state: &mut GraphicsState, glfw_state: &mut GLFWState, game_state: &mut GameState);
}
