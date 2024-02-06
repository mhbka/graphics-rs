use crate::types::*;
use crate::tgaimage::*;
use std::cmp::{min, max};

fn barycentric(vertices: &[Vertex; 3], point: Coord) -> Vertex { 
    let x_bary = Vertex {
        x: vertices[2].x - vertices[0].x,
        y: vertices[1].x - vertices[0].x,
        z: vertices[0].x - point.x as f32
    };

    let y_bary = Vertex {
        x: vertices[2].y - vertices[0].y,
        y: vertices[1].y - vertices[0].y,
        z: vertices[0].y - point.y as f32
    };

    let u = x_bary.cross_product(&y_bary);
    if u.z.abs()<1.0 {return Vertex {x:1.0, y:1.0, z:-1.0};}
    
    Vertex {
        x: 1.0 - (u.x + u.y)/u.z,
        y: u.y/u.z,
        z: u.x/u.z
    }
}


pub fn triangle<T> (image: &mut Image<T>, vertices: &[Vertex; 3], color: T) -> Result<(), String>
where T: ColorSpace + Copy { 
    // set the bounding box corners
    let mut bboxmin = Coord {x: (image.width-1) as i32, y: (image.height-1) as i32};
    let mut bboxmax = Coord {x:0, y:0};

    // bring bounding box to triangle corners
    for vertex in vertices {
        bboxmin.x = max(0, min(bboxmin.x, vertex.x as i32));
        bboxmin.y = max(0, min(bboxmin.y, vertex.y as i32));

        bboxmax.x = min(image.height as i32, max(bboxmax.x, vertex.x as i32));
        bboxmax.y = min(image.width as i32, max(bboxmax.y, vertex.y as i32));
    }

    for x in bboxmin.x .. bboxmax.x {
        for y in bboxmin.y .. bboxmax.y { 
            let check_bary = barycentric(vertices, Coord{x, y});
            if (check_bary.x<0.0 || check_bary.y<0.0 || check_bary.z<0.0) { continue; }
            image.set(x as usize, y as usize, color)?;
        }
    }
    Ok(())
}