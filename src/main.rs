mod tgaimage;
mod line;
mod obj;
mod types;
mod colors;
mod triangle_linesweep;
mod triangle_bary;

use tgaimage::*;
use line::*;
use obj::*;
use triangle_bary::*;          
//use triangle_linesweep::*;     <- i broke it
use std::time;
use types::*;



fn main() {
    let (height, width) = (1000, 1000);
    let mut image: Image<RGB> = Image::new(height, width);

    let mut f1 = [
        Vec2Di::new(10, 70),
        Vec2Di::new(50, 160),
        Vec2Di::new(70, 80)
    ];

    let mut f2 = [
        Vec2Di::new(180, 50),
        Vec2Di::new(150, 10),
        Vec2Di::new(70, 180)
    ];

    let mut f3 = [
        Vec2Di::new(180, 150),
        Vec2Di::new(120, 160),
        Vec2Di::new(130, 180)
    ];

    // timed block //
    let now = time::Instant::now();

    draw_obj("african_head.obj", &mut image);

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);
    image.write_tga_file("img.tga", true, true).unwrap();
}



