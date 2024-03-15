use std::mem::{size_of, size_of_val};
use gl::types::*;


/// Represents a vertex attribute; in implementation, assumes that attribute datatype is 32 bits (ie f32, u32, i32).
#[derive(Clone, Debug)]
pub struct VertexAttr {
    name: String, // just for knowledge, not used in openGL
    length: usize
}

impl VertexAttr {
    pub fn new(name: String, length: usize) -> Self {
        VertexAttr {name, length}
    }
}


/// Wrapper struct for a VAO.
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

/// Public fns
impl VAO {
    pub unsafe fn new(buffer_data: Vec<f32>, index_data: Option<Vec<u32>>, vertex_attrs: Vec<VertexAttr>) -> Self {
        let (mut vao, mut vbo, mut ebo) = (0, 0, Some(0));

        VAO::init_vao(&mut vao);

        VAO::init_vbo(buffer_data.as_slice(), &mut vbo);

        // if index_data is present, gen and bind an EBO  + copy index data
        match &index_data {
            Some(index_data) => { VAO::init_ebo(index_data.as_slice(), &mut ebo.as_mut().unwrap()) },
            None => { ebo = None; }
        }

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

    /// Bind the VAO.
    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.vao)
    }

    // Getters
    pub fn vao(&self) -> u32 { self.vao }
    pub fn vbo(&self) -> u32 { self.vbo }
    pub fn ebo(&self) -> Option<u32> { self.ebo } 
}


// Internal implementations
impl VAO {
    // test fn; check if EBO and VBOs are bound
    pub unsafe fn check_binding(&self) {
        gl::BindVertexArray(self.vao);

        let mut bound_ebo = 0;
        if self.ebo == None { 
            println!("ebo not used here");
        } else {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.ebo.unwrap());
            gl::GetVertexArrayiv(self.vao, gl::ELEMENT_ARRAY_BUFFER_BINDING, &mut bound_ebo as *mut i32);
            if bound_ebo == 0 { 
                println!("ebo not bound");
            } else {
                println!("ebo bound");
            }
        }
        
        let mut max_attribs: i32 = 0;
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attribs as *mut i32);
        
        let mut bound_vbo = 0;
        for i in 0..max_attribs {
            gl::GetVertexAttribiv(i as u32, gl::VERTEX_ATTRIB_ARRAY_BUFFER_BINDING, &mut bound_vbo as *mut i32);
            if bound_vbo !=0 {println!("bound vbo (attrib {i})")}
            bound_vbo = 0;
        }
    }
    
    // generate and bind VAO
    unsafe fn init_vao(vao: &mut u32) {
        gl::GenVertexArrays(1, vao as *mut u32);
        gl::BindVertexArray(*vao);    
    }
    
    // generate and bind VBO, then copy vertex data into array buffer
    unsafe fn init_vbo(buffer_data: &[f32], vbo: &mut u32) {
        gl::GenBuffers(1, vbo as *mut u32);
        gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            size_of_val(buffer_data).try_into().unwrap(), 
            buffer_data.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );
    }

    // generate and bind EBO, then copy index data into element buffer
    unsafe fn init_ebo(index_data: &[u32], ebo: &mut u32) {
        // each face = 3 vertices, so length must be mod 3
        if index_data.len()%3 != 0 { panic!("indices array length is not modulo 3.") }

        gl::GenBuffers(1, ebo as *mut u32);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size_of_val(index_data).try_into().unwrap(),
            index_data.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );
    }

    // Set vertex attr pointers (make sure this aligns with buffer_data)
    unsafe fn set_vertex_attrs(vao: u32, vertex_attrs: Vec<VertexAttr>) {
        gl::BindVertexArray(vao);

        // set the stride (total size of a vertex's attributes)
        let vertex_attrs_size = vertex_attrs
            .clone()
            .into_iter()
            .fold(0, |a, b| a+b.length);
        let vertex_stride = vertex_attrs_size as i32 * size_of::<f32>() as i32;
    
        // loop and set vertex attrib pointers for each vertex attribute, then enable it
        let mut cur_stride: i32 = 0;
        for (index, attr) in vertex_attrs.iter().enumerate() {
            if attr.length <= 0 { continue; }
            gl::VertexAttribPointer(
                index as u32, 
                attr.length as i32, 
                gl::FLOAT, 
                gl::FALSE, 
                vertex_stride, 
                cur_stride as *const GLvoid
            );
            gl::EnableVertexAttribArray(index as u32);
            cur_stride += (attr.length * size_of::<f32>()) as i32;

            let err = gl::GetError();
            if err != 0 { println!("error during '{}' vertex attr assignment: {}", attr.name, err); }
        }
    }
}