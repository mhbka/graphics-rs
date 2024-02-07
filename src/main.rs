mod tgaimage;
mod line;
mod obj;
mod types;
mod colors;
mod triangle_linesweep;
mod triangle_bary;

use tgaimage::*;
use line::*;
//use triangle_linesweep::*;     <- i broke it
use std::time;
use types::*;
use crate::obj::draw_obj;


fn main() {
    let (height, width) = (1000, 1000);
    let mut image: Image<RGB> = Image::new(height, width);

    // timed block //
    let now = time::Instant::now();

    draw_obj("african_head.obj", &mut image);

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);
    image.write_tga_file("img.tga", true, true).unwrap();

}



