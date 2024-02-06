//! geometric types.


/// a 2D integer coordinate (ie, can represent a single pixel)
pub struct Coord {
    pub x: i32, pub y: i32
}

/// a 3D float coordinate; also works as a 3D vector etc.
#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f32, pub y: f32, pub z: f32
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex {x, y, z}
    }

    pub fn to_coord(&self) -> Coord {
        Coord {x: self.x as i32, y: self.y as i32}
    }

    pub fn cross_product(&self, other_vec: &Self) -> Self {
        Vertex {
            x: self.y*other_vec.z - self.z*other_vec.y,
            y: self.z*other_vec.x - self.x*other_vec.z,
            z: self.x*other_vec.y - self.y*other_vec.z
        }
    }   
}

// A Face, formed by connecting 3 vertices
// Note that in obj files, indexes start from 1
pub struct Face {
    pub vertices: [Vertex; 3]
}