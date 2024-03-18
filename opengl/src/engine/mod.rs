mod events;
mod transform;

use glam::*;
use glfw::Context;
use crate::{
    data::CUBE_POSITIONS, 
    types::{GLFWState, GraphicsState}, 
    Uniform, 
    UniformType
};
use self::{events::handle_events, transform::{get_transform, Camera}};



// The main render/event loop of the program
pub fn run(mut graphics_state: GraphicsState, mut glfw_state: GLFWState) {
    let pos_data = Vec::from(CUBE_POSITIONS);

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, -3.0), 
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let mut cur_error = 0;

    let mut last_frame_time = glfw_state.glfw.get_time();

    let mut dist = 0.0;

    while !glfw_state.window.should_close() {
        glfw_state
            .window
            .swap_buffers();

        unsafe { 
            // Set BG color + clear color and depth buffer(s)
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);

            // Modify and set transform as uniform for each cube, then draw
            for &pos in pos_data.iter() {
                let transform = get_transform(&camera, 45.0, pos);

                graphics_state
                    .shader
                    .set_uniform(Uniform::new("transform".to_owned(), UniformType::Matrix4(transform)));

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            
            // Check for any new errors
            let error = gl::GetError();
            if error != cur_error {
                println!("error: {error}");
                cur_error = error;
            }
        };

        // Poll and handle events
        handle_events(&mut glfw_state, &mut camera, &mut last_frame_time, &mut dist);
    }
}