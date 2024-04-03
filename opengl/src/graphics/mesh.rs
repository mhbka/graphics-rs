use gl::types::GLvoid;

use super::shader::{Shader, Uniform, UniformType};
use super::texture::{Texture, TextureType};
use super::vao::VAO;
use super::vertex::Vertex;

/// Contains all data necessary for rendering an object.
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub textures: Vec<Texture>,
    pub indices: Vec<u32>,
    vao: VAO
}

impl Mesh {
    /// Generate a new mesh including its VAO.
    pub unsafe fn new(vertices: Vec<Vertex>, textures: Vec<Texture>, indices: Vec<u32>) -> Self {
        let vao = Mesh::setup_mesh_vao(&vertices);
        Mesh { vertices, textures, indices, vao }
    }

    /// Activate this mesh's VAO.
    pub unsafe fn activate(&self) {
        self.vao.bind();
    }

    /// Draw this mesh.
    pub unsafe fn draw(&self, shader: &Shader) {
        let mut diffuse_i = 1;
        let mut specular_i = 1;

        for (i, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);

            let index = match texture.variant {
                TextureType::DIFFUSE => {
                    diffuse_i +=1;
                    diffuse_i.to_string()
                },
                TextureType::SPECULAR => {
                    specular_i +=1;
                    specular_i.to_string()
                },
            };

            let material_str = format!("{}{}", texture.variant.to_string(), index);
            shader.set_uniform(Uniform::new(format!("material.{material_str}"), UniformType::Int1(i as i32)));

            gl::BindTexture(gl::TEXTURE_2D, texture.id); 
        }
        gl::ActiveTexture(gl::TEXTURE0); // TODO: is this necessary, or just a precaution?

        self.vao.bind();
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, 0 as *const GLvoid);
        gl::BindVertexArray(0); // just a precaution?
    }
}

/// Private impls
impl Mesh {
    unsafe fn setup_mesh_vao(vertices: &Vec<Vertex>) -> VAO {
        let vertex_attrs = Vertex::get_vertex_attrs();
        VAO::new(Vertex::flatten(&vertices), None, vertex_attrs)
    }
}