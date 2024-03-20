use crate::global_state::{GLFWState, GameState, GraphicsState};

use super::Renderer;

pub struct LightingRenderer {

}

impl Renderer for LightingRenderer {
    unsafe fn init(&mut self) -> GraphicsState {
        todo!()
    }

    unsafe fn render(&mut self, graphics_state: &mut GraphicsState, glfw_state: &mut GLFWState, game_state: &mut GameState) {
        todo!()
    }
}