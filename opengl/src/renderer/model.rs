use crate::graphics::model::Model;
use crate::global_state::*;
use super::Renderer;

pub struct ModelRenderer {
    model: Model
}

impl ModelRenderer {
    pub fn new(model: Model) -> Self {
        ModelRenderer { model }
    }
}

/* 
impl Renderer for ModelRenderer {
    unsafe fn init(&mut self) -> GraphicsState {
        
    }

    unsafe fn render(&mut self, graphics_state: &mut GraphicsState, glfw_state: &mut GLFWState, game_state: &mut GameState) {
        
    }
}
*/