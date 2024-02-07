use crate::types::*;
use crate::tgaimage::*;
use std::mem::swap;

pub fn rasterize_2D(image: &mut Image<RGB>, start: &mut Vec2Di, end: &mut Vec2Di, color: RGB, ybuffer: &mut Vec<i32>) {
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
            image.set(x as usize, y as usize, color).unwrap();
        }
    }
}