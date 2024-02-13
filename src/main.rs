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

    // decode image with tinytga
    let mut texture_img = convert_from_tinytga("texture.tga");

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);
    texture_img.write_tga_file("img.tga", false, false).unwrap();

}



