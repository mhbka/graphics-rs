mod glfw_init;
mod shader;
mod vao;
mod texture;

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

    // vertex data
    let vertex_data: Vec<f32> = vec![
        -0.5, -0.5, -0.5,  0.0, 0.0,
        0.5, -0.5, -0.5,  1.0, 0.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0, 0.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 1.0,
        0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,

        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,

        0.5,  0.5,  0.5,  1.0, 0.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5,  0.5,  0.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5, -0.5,  1.0, 1.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0, 1.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0
    ];

    // index data
    /* 
    let index_data: Vec<u32> = vec![
        0, 1, 3,
        1, 2, 3
    ];
    */
    
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
    while !window.should_close() {
        window.swap_buffers();

        unsafe { 
            // Set BG color
            gl::ClearColor(0.9, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);

            // Modify and set transform as uniform
            let model = Mat4::from_rotation_x(glfw.get_time() as f32) * Mat4::from_rotation_y(glfw.get_time() as f32);
            let view = Mat4::from_translation(Vec3::new(0.0, 0.0, -5.0));
            let projection = Mat4::perspective_rh(50.0, 800.0/600.0, 0.1, 100.0);
            
            let transform = projection * view * model;

            shader_program.set_uniform(Uniform::new("transform".to_owned(), UniformType::Matrix4(transform)));

            // Draw triangles
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

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
            println!("{:?}", event);
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                WindowEvent::Key(Key::Up, _, Action::Press, _) => {
                    mix_amount += 0.1;
                    unsafe {
                        shader_program.set_uniform(Uniform::new("mix_amount".to_owned(), UniformType::Float1(mix_amount)));
                    }
                },
                WindowEvent::Key(Key::Down, _, Action::Press, _) => {
                    mix_amount -= 0.1;
                    unsafe {
                        shader_program.set_uniform(Uniform::new("mix_amount".to_owned(), UniformType::Float1(mix_amount)));
                    }
                },
                _ => {},
            }
        }
    }
}