pub mod camera;
mod events;
mod transform;

use glfw::Context;
use crate::{
    data::CUBE_POSITIONS, 
    types::{GLFWState, GraphicsState, GameState}, 
    Uniform, 
    UniformType
};
use self::{
    events::poll_and_handle_events, 
    transform::get_transform
};


// The main render/event loop of the program
pub fn run(mut graphics_state: GraphicsState, mut glfw_state: GLFWState, mut game_state: GameState) {
    let pos_data = Vec::from(CUBE_POSITIONS);

    let mut cur_error = 0;

    while !glfw_state.window.should_close() {
        glfw_state.window.swap_buffers();

        unsafe { 
            // Set BG color + clear color and depth buffer(s)
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);

            // Modify and set transform as uniform for each cube, then draw
            for &pos in pos_data.iter() {
                let transform = get_transform(&game_state.camera, pos);

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
        poll_and_handle_events(&mut glfw_state, &mut game_state);
    }
}