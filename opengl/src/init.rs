use gl::types::GLvoid;
use std::mem::{size_of, size_of_val};
use std::ptr::null;

/** Performs initialization for openGL. */
pub unsafe fn gl_init(vertices: &[f32], indices: &[u32], vao: &mut u32, vbo: &mut u32, ebo: &mut u32) {
    // bound check
    if indices.len()%3 != 0 {panic!("indices array length is not modulo 3.")}

    // generate and bind VBO, then copy vertex data into array buffer
    gl::GenBuffers(1, vbo as *mut u32);
    gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER, 
        size_of_val(vertices).try_into().unwrap(), 
        vertices.as_ptr() as *const GLvoid,
        gl::STATIC_DRAW
    );
    println!("{}", size_of_val(&vertices));

    // generate and bind EBO, then copy index data into element buffer
    gl::GenBuffers(1, ebo as *mut u32);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        size_of_val(indices).try_into().unwrap(),
        indices.as_ptr() as *const GLvoid,
        gl::STATIC_DRAW
    );

    // generate and bind VAO
    gl::GenVertexArrays(1, vao as *mut u32);
    gl::BindVertexArray(*vao);

    // set vertex attribute pointers for VAO
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<f32>() as i32, null());
    gl::EnableVertexAttribArray(0);
}