use glam::*;
use glfw::{Window, WindowEvent, Key, Action, Context};
use crate::{
    data::{self, CUBE_POSITIONS}, types::{GLFWState, GraphicsState}, Uniform, UniformType
};

// The main render/event loop of the program
pub fn run(mut graphics_state: GraphicsState, mut glfw_state: GLFWState) {
    let pos_data = Vec::from(data::CUBE_POSITIONS);
    let mut cameraPos = Vec3::new(0.0, 0.0, -3.0);
    let mut cameraTarget = Vec3::new(0.0, 0.0, 0.0);
    let mut fov = 45.0;
    let mut cur_error = 0;
    while !glfw_state.window.should_close() {
        glfw_state.window.swap_buffers();

        unsafe { 
            // Set BG color
            gl::ClearColor(0.9, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);

            // Modify and set transform as uniform for each cube, then draw
            for (i, pos) in pos_data.iter().enumerate() {
                let angle = glfw_state.glfw.get_time() as f32;
                let model = Mat4::from_rotation_x(angle) * Mat4::from_rotation_y(angle);
                let view = Mat4::from_translation(*pos) * Mat4::from_translation(cameraPos - cameraTarget);
                let projection = Mat4::perspective_rh_gl(f32::to_radians(fov), 800.0/600.0, 0.1, 100.0);

                let transform = projection * view * model;

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

        // Poll for and process events
        glfw_state.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&glfw_state.events) {
            // println!("{:?}", event);
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    glfw_state.window.set_should_close(true)
                },
                WindowEvent::Key(Key::Up, _, Action::Press | Action::Repeat, _) => {
                    fov += 1.0;
                    println!("fov up: {fov}");
                },
                WindowEvent::Key(Key::Down, _, Action::Press | Action::Repeat, _) => {
                    fov -= 1.0;
                    println!("fov down: {fov}");
                },
                WindowEvent::CursorEnter(true) => {
                    println!("wow!");
                },
                WindowEvent::Focus(false) => {
                    println!("bye!");
                },
                _ => {},
            }
        }
    }
}