use crate::tgaimage::*;
use std::mem::swap;
use glam::*;

pub fn line<T>(image: &mut Image<T>, mut start: Vec2, mut end: Vec2, color: T) -> Result<(), String>
where T: ColorSpace + Copy {
    let mut steep = false;

    // if steep line, transpose them
    if (start.x - end.x).abs() < (start.y - end.y).abs() {
        swap(&mut start.x, &mut start.y);
        swap(&mut end.x, &mut end.y);
        steep = true;
    }

    // if start's x is greater than end's x, swap them
    if start.x > end.x {
        swap(&mut start.x, &mut end.x);
        swap(&mut start.y, &mut end.y);
    }

    // difference + error
    let dx = end.x - start.x;
    let dy = end.y - start.y;

    let derror = (dy as f32 / dx as f32).abs();
    let mut error = 0.0;
    let mut y = start.y;

    // iterate
    for x in start.x as i32 .. end.x as i32 + 1 {
        if steep { image.set(y as usize, x as usize, color)?; }
        else { image.set(x as usize, y as usize, color)?; }

        error += derror;
        if error > 0.5 {
            y += if end.y > start.y {1.0} else {-1.0};
            error -= 1.0;
        }       
    }

    Ok(())
}

pub fn line_3d<T>(image: &mut Image<T>, start: Vec3, end: Vec3, transform: Affine3A, color: T) -> Result<(), String>
where T: ColorSpace + Copy 
{
    let mut start = transform.transform_point3(start);
    let mut end = transform.transform_point3(end);
    //println!("start: {}; end: {}", start, end);
    let mut steep = false;

    // if steep line, transpose them
    if (start.x - end.x).abs() < (start.y - end.y).abs() {
        swap(&mut start.x, &mut start.y);
        swap(&mut end.x, &mut end.y);
        steep = true;
    }

    // if start's x is greater than end's x, swap them
    if start.x > end.x {
        swap(&mut start.x, &mut end.x);
        swap(&mut start.y, &mut end.y);
    }

    // difference + error
    let dx = end.x - start.x;
    let dy = end.y - start.y;

    let derror = (dy as f32 / dx as f32).abs();
    let mut error = 0.0;
    let mut y = start.y;

    // iterate
    for x in start.x as i32 .. end.x as i32 + 1 {
        if steep { 
            image.set(y as usize, x as usize, color)?; 
        }
        else { 
            image.set(x as usize, y as usize, color)?;
        }

        error += derror;
        if error > 0.5 {
            y += if end.y > start.y {1.0} else {-1.0};
            error -= 1.0;
        }       
    };

    Ok(())
}


/// Adds lines on [-1, 1] for the 3 axes; red for -ve values, blue for +ve.
pub fn add_axis_lines(mut image: &mut Image<RGB>, transform: Affine3A) {
    let mid = Vec3::new(0.0, 0.0, 0.0);
    let x_neg = Vec3::new(-1.0, 0.0, 0.0);
    let x_pos = Vec3::new(1.0, 0.0, 0.0);
    let y_neg = Vec3::new(0.0, -1.0, 0.0);
    let y_pos = Vec3::new(0.0, 1.0, 0.0);
    let z_neg = Vec3::new(0.0, 0.0, -1.0);
    let z_pos = Vec3::new(0.0, 0.0, 1.0);

    let red = RGB {r: 255, g: 0, b: 0}; //negative
    let blue = RGB {r:0, g:0, b: 255}; //positive

    line_3d(&mut image, x_neg, mid, transform.clone(), red.clone()).unwrap();
    line_3d(&mut image, mid, x_pos, transform.clone(), blue.clone()).unwrap();
    line_3d(&mut image, y_neg, mid, transform.clone(), red.clone()).unwrap();
    line_3d(&mut image, mid, y_pos, transform.clone(), blue.clone()).unwrap();
    line_3d(&mut image, z_neg, mid, transform.clone(), red.clone()).unwrap();
    line_3d(&mut image, mid, z_pos, transform.clone(), blue.clone()).unwrap();

}