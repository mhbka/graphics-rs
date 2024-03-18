use glam::*;
use glfw::Window;
use crate::types::GLFWState;
use super::transform::Camera;


// Handles WASD input on each game loop
pub fn handle_wasd_movement(glfw_state: GLFWState) {
    
}

// Applies mouse movement into camera pitch and yaw.
pub fn handle_mouse_movement(camera: &mut Camera, cur_pos: Vec2, last_pos: &mut Vec2) {
    const SENS: f32 = 0.1;
    let x_offset = (cur_pos.x - last_pos.x) * SENS;
    let y_offset = (last_pos.y - cur_pos.y) * SENS; // reversed since y-coords range from bottom to top
    
    camera.yaw += x_offset;
    camera.pitch += y_offset;

    if camera.pitch > 89.0 { camera.pitch = 89.0; }
    else if camera.pitch < -89.0 { camera.pitch = -89.0; }

    // read here for explanation: https://learnopengl.com/Getting-started/Camera
    let mut direction = Vec3::new(
        camera.yaw.to_radians().cos() * camera.pitch.to_radians().cos(),
        camera.pitch.to_radians().sin(),
        camera.yaw.to_radians().sin() * camera.pitch.to_radians().cos()
    );

    camera.front = direction.normalize();
    *last_pos = cur_pos;
}