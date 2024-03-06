use gl::types::GLvoid;
use std::mem::{size_of, size_of_val};
use std::ptr::null;


/// Initializes all OpenGL state.
/* 
pub unsafe fn init() ->  {

}
*/

/// Performs initialization for vertex-related state.
pub unsafe fn init(vertices: &[f32], indices: Option<&[u32]>, vao: &mut u32, vbo: &mut u32, ebo: &mut u32) {
    // only gen and bind EBO if we passed in indices
    match indices {
        Some(indices) => init_ebo(indices, ebo),
        None => {}
    }
    
    // init VBO and VAO
    init_vbo(vertices, vbo);
    init_vao(vao);
}

unsafe fn init_vbo(vertices: &[f32], vbo: &mut u32) {
// generate and bind VBO, then copy vertex data into array buffer
    gl::GenBuffers(1, vbo as *mut u32);
    gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER, 
        size_of_val(vertices).try_into().unwrap(), 
        vertices.as_ptr() as *const GLvoid,
        gl::STATIC_DRAW
    );
}

unsafe fn init_ebo(indices: &[u32], ebo: &mut u32) {
    // each face = 3 vertices, so length must be mod 3
    if indices.len()%3 != 0 { panic!("indices array length is not modulo 3.") }

    // generate and bind EBO, then copy index data into element buffer
    gl::GenBuffers(1, ebo as *mut u32);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        size_of_val(indices).try_into().unwrap(),
        indices.as_ptr() as *const GLvoid,
        gl::STATIC_DRAW
    );
}

unsafe fn init_vao(vao: &mut u32) {
    // generate and bind VAO
    gl::GenVertexArrays(1, vao as *mut u32);
    gl::BindVertexArray(*vao);

    // set the stride (total size of a vertex's attributes)
    let vertex_stride = 6*size_of::<f32>() as i32;

    // set 1st pointer: xyz position data, then enable it
    gl::VertexAttribPointer(
        0, 3, 
        gl::FLOAT, 
        gl::FALSE, 
        vertex_stride, 
        null()
    );
    gl::EnableVertexAttribArray(0);

    // set 2nd pointer: rgb color data, then enable it
    gl::VertexAttribPointer(
        1, 
        3, 
        gl::FLOAT, 
        gl::FALSE, 
        vertex_stride, 
        (3*size_of::<f32>()) as *const GLvoid
    );
    gl::EnableVertexAttribArray(1);
    
}