mod tgaimage;
mod line;
mod obj;
mod colors;
mod triangle_bary;

use tgaimage::*;
use std::time;
use crate::obj::draw_obj;




fn main() {

    let mut image = Image::new(1024, 1024);

    // timed block //
    let now = time::Instant::now();

    draw_obj("african_head.obj", &mut image);

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);
    image.write_tga_file("img.tga", true, false).unwrap();

}



