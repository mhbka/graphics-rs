use super::{mesh::Mesh, shader::Shader};

pub struct Model {
    meshes: Vec<Mesh>,
}

impl Model {
    /* 
    pub fn new(filepath: &str) -> Self {

    }
    */

    pub unsafe fn draw(&self, shader: &mut Shader) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }
}

// private impls
impl Model {
    fn load_model(filepath: &str) {

    }
}