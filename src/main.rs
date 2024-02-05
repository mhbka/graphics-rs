mod tgaimage;
use tgaimage::*;
use std::mem::swap;

struct Coord {
    x: i32, y: i32
}

fn main() {
    let mut image: Image<RGB> = Image::new(100, 100);

    let start = Coord {x: 100, y: 100};
    let end = Coord {x: 50, y: 0};
    let color = RGB {r: 255, g: 0, b: 0};
    
    line(&mut image, start, end, color);
    image.write_tga_file("line.tga", false, true).unwrap();
}


fn line<T>(image: &mut Image<T>, mut start: Coord, mut end: Coord, color: T)
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

    // iterate
    for x in start.x .. end.x {
        let t = (x-start.x) as f32 / (end.x - start.x) as f32;
        let y = (start.y as f32)*(1.-t) + (end.y as f32)*t;
        
        if steep { image.set(y as usize, x as usize, color).unwrap(); }
        else { image.set(x as usize, y as usize, color).unwrap(); }
        
    }
}
