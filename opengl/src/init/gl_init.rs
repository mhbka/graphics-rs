use crate::{
    VertexAttr,
    VAO,
    data,
    Texture,
    Shader,
    Uniform,
    UniformType,
    types::GraphicsState
};

// Initializes OpenGL context.
pub unsafe fn init() -> GraphicsState {
    // load data
    let vertex_data: Vec<f32> = Vec::from(data::VERTEX_DATA);

    // Initialize VAO
    let vertex_attrs = vec![
        VertexAttr::new("Position".to_owned(), 3), 
        VertexAttr::new("Color".to_owned(), 0), // lol
        VertexAttr::new("Texture Coords".to_owned(), 2)
    ];
    let vao = VAO::new(vertex_data, None, vertex_attrs);
    gl::Enable(gl::DEPTH_TEST);
    vao.check_binding();

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

    // Check for error before returning
    let mut err = gl::GetError();
    if err != 0 { 
        panic!("error during gl initialization: {err} ");
    } else { 
        println!("note: initialization success"); 
    }

    GraphicsState::new(vao, shader_program)
}