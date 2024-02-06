use crate::tgaimage::*;
use std::mem::swap;
use crate::types::Coord;

pub fn line<T>(image: &mut Image<T>, mut start: Coord, mut end: Coord, color: T) -> Result<(), String>
where T: ColorSpace + Copy {
    let mut steep = false;

    // if steep line, transpose coordinates
    if (start.x - end.x).abs() < (start.y - end.y).abs() {
        swap(&mut start.x, &mut start.y);
        swap(&mut end.x, &mut end.y);
        steep = true;
    }

    // if start's x is greater than end's x, swap the coordinates
    if start.x > end.x {
        swap(&mut start.x, &mut end.x);
        swap(&mut start.y, &mut end.y);
    }

    // difference + error
    let dx = (end.x - start.x);
    let dy = (end.y - start.y);

    let derror = (dy as f32 / dx as f32).abs();
    let mut error = 0.0;
    let mut y = start.y;

    // iterate
    for x in start.x .. end.x {
        if steep { image.set(y as usize, x as usize, color)?; }
        else { image.set(x as usize, y as usize, color)?; }

        error += derror;
        if error > 0.5 {
            y += (if end.y>start.y {1} else {-1});
            error -= 1.0;
        }       
    }

    Ok(())
}