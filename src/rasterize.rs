use crate::types::*;
use crate::tgaimage::*;
use std::mem::swap;

// used to render a 2D scene in 1D and account for depth.
// assumes that we are viewing from the "top"; ie, higher y-value = closer.
// so if a pixel is "closer" for an x-value, then it overwrites the previous pixel and sets the new ybuffer value.
pub fn rasterize_2D<T>(image: &mut Image<T>, start: &mut Vec2Di, end: &mut Vec2Di, color: T, ybuffer: &mut Vec<i32>)
where T: ColorSpace + Copy {
    if start.x > end.x { swap(start, end); }
    
    for x in start.x .. end.x {
        let t = (x - start.x) as f32 / (end.x - start.x) as f32;
        let y = (t*start.y as f32) + ((1.0-t)*end.y as f32);
        let y = y as i32;

        // if, for this x, the line's y-value is closer than buffer's y-value,
        // write the pixel to image, and set a new buffer y-value
        // (closer = higher y, since we assume camera from the top of scene)
        if y > ybuffer[x as usize] {
            ybuffer[x as usize] = y;
            image.set(x as usize, 0, color).unwrap();
        }
    }
}


