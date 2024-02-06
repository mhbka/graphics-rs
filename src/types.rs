//! geometric types

pub struct Coord {
    pub x: i32, pub y: i32
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f32, pub y: f32, pub z: f32
}

// Note that in obj files, indexes start from 1
pub struct Face {
    pub vertices: [Vertex; 3]
}