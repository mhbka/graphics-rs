use super::texture::Texture;
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
    pub fn new(vertices: Vec<Vertex>, textures: Vec<Texture>, indices: Vec<u32>) -> Self {
        
    }
}