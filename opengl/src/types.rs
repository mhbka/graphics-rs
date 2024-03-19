use glam::*;
use glfw::{Glfw, PWindow, GlfwReceiver, WindowEvent};
use crate::{engine::camera::Camera, graphics::{
        shader::Shader, vao::VAO
}};



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

// Encapsulates game state.
pub struct GameState {
    pub camera: Camera,
    pub last_frame_time: f64,
    pub last_mouse_pos: (f64, f64)
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            camera: Camera::new(
                Vec3::new(0.0, 0.0, -3.0), 
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 1.0, 0.0)
            ),
            last_frame_time: 0.0,
            last_mouse_pos: (0.0, 0.0)
        }
    }
}