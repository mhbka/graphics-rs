use std::mem::swap;
use crate::types::*;
use crate::tgaimage::*;

pub fn triangle<T> (image: &mut Image<T>, vertices: &mut [Vertex; 3], color: T) -> Result<(), String>
where T: ColorSpace + Copy {
    // sort vertices by increasing y coords
    if vertices[0].y > vertices[1].y { vertices.swap(0, 1); }
    if vertices[0].y > vertices[2].y { vertices.swap(0, 2); }
    if vertices[1].y > vertices[2].y { vertices.swap(1, 2); }

    // get total and segment height
    let segment_height = vertices[1].y - vertices[0].y;
    let total_height = vertices[2].y - vertices[0].y;

    // draw both sides of the bottom segment
    for y in vertices[0].y as usize .. vertices[1].y as usize {
        let a = (y as f32 - vertices[0].y) / total_height; // the long side, which is chopped by segment
        let b = (y as f32 - vertices[0].y) / segment_height; // the short side, which is exactly bounded by segment

        let mut a_x = vertices[0].x + a*(vertices[2].x - vertices[0].x); // scale x for long side
        let mut b_x = vertices[0].x + b*(vertices[1].x - vertices[0].x); // same for short side

        if a_x > b_x { swap(&mut a_x, &mut b_x);}
        for x in a_x as usize .. b_x as usize { 
            image.set(x as usize, y as usize, color)?; 
        }
    }

    let segment_height = vertices[2].y - vertices[1].y;

    // draw both sides of the top segment
    for y in vertices[1].y as usize .. vertices[2].y as usize {
        let a = (y as f32 - vertices[0].y) / total_height; // the long side, which is chopped by segment
        let b = (y as f32 - vertices[1].y) / segment_height; // the short side, which is exactly bounded by segment

        let mut a_x = vertices[0].x + a*(vertices[2].x - vertices[0].x); // scale x for long side
        let mut b_x = vertices[1].x + b*(vertices[2].x - vertices[1].x); // same for new short side
        
        if a_x > b_x { swap(&mut a_x, &mut b_x);}
        for x in a_x as usize .. b_x as usize { 
            image.set(x as usize, y as usize, color)?; 
        }
    }
    Ok(())
}
