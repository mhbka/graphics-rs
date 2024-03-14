mod glfw_init;
mod shader;
mod vao;
mod texture;
mod data;

use glam::*;
use gl::types::*;
use glfw::{Action, Context, Key, WindowEvent};
use shader::{Shader, Uniform, UniformType};
use vao::{VAO, VertexAttr};
use texture::Texture;
use std::env;
use std::f32::consts::PI;

const RAD: f32 = PI/180.0;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let use_old_ver = true;

    // Initialize GLFW + load functions into `gl`
    let (mut glfw, mut window, events) = glfw_init::init(use_old_ver);

    // data
    let vertex_data: Vec<f32> = Vec::from(data::vertex_data);
    let pos_data = Vec::from(data::cube_positions);
    
    // Initialize VAO
    let vertex_attrs = vec![
        VertexAttr::new("Position".to_owned(), 3), 
        VertexAttr::new("Color".to_owned(), 0), // lol
        VertexAttr::new("Texture Coords".to_owned(), 2)
    ];
    let vao = unsafe { VAO::new(vertex_data, None, vertex_attrs) };
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    unsafe { vao.check_binding() };

    // Initialize textures to texture units + amount to mix them
    let texture1 = unsafe { Texture::new("wall.jpg", gl::TEXTURE1) };
    let texture2 = unsafe { Texture::new("smileyface.png", gl::TEXTURE2) };
    let mut mix_amount = 0.2;

    // Initialize and use shader + add textures as uniforms
    let mut shader_program = unsafe { Shader::new("test") };
    unsafe {
        shader_program.set_uniform(Uniform::new("texture1".to_owned(), UniformType::Int1(1)));
        shader_program.set_uniform(Uniform::new("texture2".to_owned(), UniformType::Int1(2)));
        shader_program.set_uniform(Uniform::new("mix_amount".to_owned(), UniformType::Float1(mix_amount)));
    }

    // Check for error before main loop (also using this for checking error during loop)
    let mut cur_error = unsafe { gl::GetError() };
    if cur_error != 0 { panic!("error during init: {cur_error} ");} 
    else { println!("note: initialization succeeded; going to event loop"); }


    //
    // MAIN LOOP - until window is closed
    //
    let mut fov = 45.0;
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
                let view = Mat4::from_translation(*pos) * Mat4::from_translation(Vec3::new(0.0, 0.0, -10.0));
                let projection = Mat4::perspective_rh_gl(fov * RAD, 800.0/600.0, 0.1, 100.0);
                
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
            //println!("{:?}", event);
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                WindowEvent::Key(Key::Up, _, Action::Press, _) | WindowEvent::Key(Key::Up, _, Action::Repeat, _)=> {
                    fov += 0.05;
                    println!("fov up: {fov}");
                },
                WindowEvent::Key(Key::Down, _, Action::Press, _) | WindowEvent::Key(Key::Down, _, Action::Repeat, _) => {
                    fov -= 0.05;
                    println!("fov down: {fov}");
                },
                _ => {},
            }
        }
    }
}