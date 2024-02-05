use std::fs;

pub struct Vertex {
    x: f32, y: f32, z: f32
}

// Note that in obj files indexes start from 1
pub struct Face {
    f1: Vertex, f2: Vertex, f3: Vertex
}

pub fn parse_obj(filepath: String) -> Vec<Vertex> { 
    let contents = fs::read_to_string(filepath)
        .expect(&format!("No filepath: {filepath}")[..]);

    let vertices = Vec::new();
    let faces = Vec::new();

    for line in contents.lines() {
        if line.starts_with("v ") {
            vertices.push(parse_vertex(&line));
        }

        else if line.starts_with("vt ") {
            //ignore
        }

        else if line.starts_with("vn ") {
            //ignore
        }

        else if line.starts_with("f ") {

        }
    }   

}

fn parse_vertex(str: &str) -> Vertex {
    
}