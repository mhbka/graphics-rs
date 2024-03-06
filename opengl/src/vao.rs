use std::{mem::{size_of, size_of_val}, ptr::null};
use gl::types::*;


/// Wrapper struct for a VAO; encapsulates and tracks/sets VAO state.
pub struct VAO {
    // VAO + vertex attributes
    vao: u32,
    vertex_attrs: Vec<VertexAttr>,

    // VBO + vertice data
    vbo: u32,
    buffer_data: Vec<f32>,

    // EBO + index data (optional)
    ebo: Option<u32>,
    index_data: Option<Vec<u32>>
}


// Represents a vertex attribute; in implementation, assumes that attribute datatype is 32 bits (ie f32, u32, i32)
#[derive(Clone)]
pub struct VertexAttr {
    name: String,
    length: usize
}


/// Wrapper implementations for OpenGL shaders.
impl VAO {
    pub unsafe fn new(buffer_data: Vec<f32>, index_data: Option<Vec<u32>>, vertex_attrs: Vec<VertexAttr>) -> Self {
        let (mut vao, mut vbo, mut ebo) = (0, 0, Some(0));

        // only gen and bind an EBO if index_data is present + copy index data
        match &index_data {
            Some(index_data) => { VAO::init_ebo(index_data.as_slice(), &mut ebo.unwrap()) },
            None => { ebo = None; }
        }

        // gen and bind an EBO + copy buffer data
        VAO::init_vbo(buffer_data.as_slice(), &mut vbo);

        // gen and bind the VAO
        VAO::init_vao(&mut vao);

        // set vertex attributes
        VAO::set_vertex_attrs(vao, vertex_attrs.clone());

        VAO {
            vao,
            vertex_attrs,
            vbo,
            buffer_data,
            ebo,
            index_data
        }
    }
}


// private implementations
impl VAO {
    unsafe fn init_vao(vao: &mut u32) {
        // generate and bind VAO
        gl::GenVertexArrays(1, vao as *mut u32);
        gl::BindVertexArray(*vao);    
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

    unsafe fn set_vertex_attrs(vao: u32, vertex_attrs: Vec<VertexAttr>) {
        // set the stride (total size of a vertex's attributes)
        let vertex_attrs_size = vertex_attrs
            .clone()
            .into_iter()
            .fold(0, |a, b| a+b.length);
        let vertex_stride = vertex_attrs_size as i32 * size_of::<f32>() as i32;
    
        // loop and set pointers for each vertex attribute, then enable it
        for (index, attr) in vertex_attrs.iter().enumerate() {
            gl::VertexAttribPointer(
                index as u32, 
                attr.length as i32, 
                gl::FLOAT, 
                gl::FALSE, 
                vertex_stride, 
                null()
            );
            gl::EnableVertexAttribArray(index as u32);
        }
    }
}