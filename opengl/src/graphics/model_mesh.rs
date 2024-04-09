use std::rc::Rc;
use std::cell::RefCell;
use super::shader::{Shader, Uniform, UniformType};
use super::model_texture::{ModelTexture, ModelTextureType};
use super::vao::VAO;
use super::vertex::Vertex;

/// Contains all data necessary for rendering an object.
/// 
/// Note: named as such due to conflict with russimp's Mesh type.
pub struct ModelMesh {
    pub vertices: Vec<Vertex>,
    pub textures: Vec<Rc<RefCell<ModelTexture>>>,
    pub indices: Vec<u32>,
    vao: VAO
}

impl ModelMesh {
    /// Generate a new ModelMesh including its VAO.
    pub unsafe fn new(vertices: Vec<Vertex>, textures: Vec<Rc<RefCell<ModelTexture>>>, indices: Vec<u32>) -> Self {
        let vertex_attrs = Vertex::get_vertex_attrs();
        let vao = VAO::new(Vertex::flatten(&vertices), None, vertex_attrs);
        ModelMesh { vertices, textures, indices, vao }
    }

    /// Activate this ModelMesh's VAO.
    pub unsafe fn activate(&self) {
        self.vao.bind();
    }

    /// Draw this ModelMesh once.
    /// Sets texture uniforms named by the format `material.{TextureType}{index}`.
    pub unsafe fn draw(&self, shader: &mut Shader) {
        let mut diffuse_i = 0;
        let mut specular_i = 0;

        for (i, texture) in self.textures.iter().enumerate() {
            let texture = texture.borrow();
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);

            let index = match texture.variant {
                ModelTextureType::DIFFUSE => {
                    diffuse_i +=1;
                    diffuse_i.to_string()
                },
                ModelTextureType::SPECULAR => {
                    specular_i +=1;
                    specular_i.to_string()
                },
            };

            let material_str = format!("{}{}", texture.variant.to_string(), index);
            shader.set_uniform(Uniform::new(format!("material.{material_str}"), UniformType::Int1(i as i32)));

            gl::BindTexture(gl::TEXTURE_2D, texture.id); 
        }
        gl::ActiveTexture(gl::TEXTURE0); // just a precaution?

        self.activate();
        // gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, 0 as *const GLvoid);
        gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);

        let err = gl::GetError();
        if err != 0 {
            println!("error: {err}");
        }

        gl::BindVertexArray(0); // just a precaution?
    }
}