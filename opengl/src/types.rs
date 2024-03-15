use glfw::{Glfw, PWindow, GlfwReceiver, WindowEvent};
use crate::graphics::{
        vao::VAO, 
        shader::Shader
};



// Encapsulates gl state.
pub struct GraphicsState {
    pub vao: VAO,
    pub shader: Shader
}

impl GraphicsState {
    pub fn new(vao: VAO, shader: Shader) -> Self {
        GraphicsState { vao, shader }
    }
}

// Encapsulates GLFW state.
pub struct GLFWState {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>
}

impl GLFWState {
    pub fn new(glfw: Glfw, window: PWindow, events: GlfwReceiver<(f64, WindowEvent)>) -> Self {
        GLFWState { glfw, window, events }
    }
}