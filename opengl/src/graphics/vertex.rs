use glam::*;

use super::vao::VertexAttr;

/// A graphical vertex.
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texture_coords: Vec2 
}

impl Vertex {
    /// Returns the vertex attributes for a Vertex, for generating a VAO.
    pub fn get_vertex_attrs() -> Vec<VertexAttr> {
        vec![
            VertexAttr::new("Position".to_owned(), 3),
            VertexAttr::new("Normal".to_owned(), 3),
            VertexAttr::new("Tex Coords".to_owned(), 2),
        ]
    }

    /// Flattens a vector of vertices into a vector of f32s.
    pub fn flatten(vertices: &Vec<Self>) -> Vec<f32> {
        let mut flattened_vec = Vec::with_capacity(3*3*2*vertices.len());

        for vertex in vertices {
            flattened_vec.extend_from_slice(&vertex.position.to_array());
            flattened_vec.extend_from_slice(&vertex.normal.to_array());
            flattened_vec.extend_from_slice(&vertex.texture_coords.to_array());
        }

        flattened_vec
    }
}