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

use tinytga::{RawTga};



fn main() {

    // timed block //
    let now = time::Instant::now();

    let mut image = Image::new(1024, 1024);
    draw_obj("african_head.obj", "texture.tga", &mut image);

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);
    image.write_tga_file("img.tga", true, false).unwrap();

}



