mod init;
mod graphics;
mod data;
mod main_loop;

use glam::*;
use std::env;
use std::f32::consts::PI;
use glfw::{Action, Context, Key, WindowEvent};

use init::{gl_init, glfw_init};
use graphics::{
    shader::{Shader, Uniform, UniformType},
    vao::{VAO, VertexAttr},
    texture::Texture
};


const RAD: f32 = PI/180.0;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let use_old_ver = true;

    // Initialize GLFW + load functions into gl
    let (width, height) = (800, 600);
    let (mut glfw, mut window, events) = glfw_init::init(width, height, use_old_ver);

    // Initialize gl
    let (vao, mut shader_program) = unsafe { gl_init::init() };

    // Position data
    let pos_data = Vec::from(data::CUBE_POSITIONS);

    //
    // MAIN LOOP - until window is closed
    //
    let mut cameraPos = Vec3::new(0.0, 0.0, -3.0);
    let mut cameraTarget = Vec3::new(0.0, 0.0, 0.0);
    let mut fov = 45.0;
    let mut cur_error = 0;
    while !window.should_close() {
        window.swap_buffers();

        unsafe { 
            // Set BG color
            gl::ClearColor(0.9, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);

            // Modify and set transform as uniform for each cube, then draw
            for (i, pos) in pos_data.iter().enumerate() {
                let angle = glfw.get_time() as f32;
                let model = Mat4::from_rotation_x(angle) * Mat4::from_rotation_y(angle);
                let view = Mat4::from_translation(*pos) * Mat4::from_translation(cameraPos - cameraTarget);
                let projection = Mat4::perspective_rh_gl(fov * RAD, 800.0/600.0, 0.1, 100.0);

                let lookat = Mat4::look_at_rh(cameraPos, cameraTarget, )
                
                let transform = projection * view * model;

                shader_program.set_uniform(Uniform::new("transform".to_owned(), UniformType::Matrix4(transform)));

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
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            // println!("{:?}", event);
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
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