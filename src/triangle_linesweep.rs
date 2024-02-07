use std::mem::swap;
use crate::types::*;
use crate::tgaimage::*;

pub fn triangle<T> (image: &mut Image<T>, pts: &mut [Vec2Di; 3], color: T) -> Result<(), String>
where T: ColorSpace + Copy {
    // sort pts by increasing y coords
    if pts[0].y > pts[1].y { pts.swap(0, 1); }
    if pts[0].y > pts[2].y { pts.swap(0, 2); }
    if pts[1].y > pts[2].y { pts.swap(1, 2); }

    // get total and segment height
    let segment_height = pts[1].y - pts[0].y;
    let total_height = pts[2].y - pts[0].y;

    // draw both sides of the bottom segment
    for y in pts[0].y .. pts[1].y {
        let a = (y - pts[0].y) / total_height; // the long side, which is chopped by segment
        let b = (y - pts[0].y) / segment_height; // the short side, which is exactly bounded by segment

        let mut a_x = pts[0].x + a*(pts[2].x - pts[0].x); // scale x for long side
        let mut b_x = pts[0].x + b*(pts[1].x - pts[0].x); // same for short side

        if a_x > b_x { swap(&mut a_x, &mut b_x);}
        for x in a_x as usize .. b_x as usize { 
            image.set(x as usize, y as usize, color)?; 
        }
    }

    let segment_height = pts[2].y - pts[1].y;

    // draw both sides of the top segment
    for y in pts[1].y .. pts[2].y {
        let a = (y - pts[0].y) / total_height; // the long side, which is chopped by segment
        let b = (y - pts[1].y) / segment_height; // the short side, which is exactly bounded by segment

        let mut a_x = pts[0].x + a*(pts[2].x - pts[0].x); // scale x for long side
        let mut b_x = pts[1].x + b*(pts[2].x - pts[1].x); // same for new short side
        
        if a_x > b_x { swap(&mut a_x, &mut b_x);}
        for x in a_x as usize .. b_x as usize { 
            image.set(x as usize, y as usize, color)?; 
        }
    }
    Ok(())
}
