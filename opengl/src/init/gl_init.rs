use crate::{
    global_state::GraphicsState, renderer::Renderer
};

/// Initializes OpenGL context.
/// Different renders require different setups, so we simply call render from a generic Render,
/// and let it set up the state itself.
pub unsafe fn init <T: Renderer>(renderer: &mut T) -> GraphicsState {
    gl::Enable(gl::DEPTH_TEST);
    renderer.init()
}