/// A wrapper struct to externally keep track of VAO state.
/// A Vertex Array Object (VAO) tracks the following state:
///     - Bound VBO(s) and their associated buffers
///     - Bound EBO(s) and their associated element buffers (optional)
///     - Vertex attribute pointers, and if they're enabled
pub struct VAOState {
    vao: u32,
    vbo: u32,
    ebo: u32,
    vertices: Vec<f32>,
    indices: Option<Vec<f32>>, // optional; an EBO is only generated and bound if this is not None
    attributes: u32 // an ORDERED list of vertex attributes; assumes that 
}