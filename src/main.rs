mod tgaimage;
mod line;
mod obj;
mod types;
mod colors;
mod triangle_linesweep;
mod triangle_bary;
mod rasterize;

use tgaimage::*;
use line::*;
use rasterize::*;

          
//use triangle_linesweep::*;     <- i broke it
use std::time;
use types::*;



fn main() {
    let (height, width) = (1000, 1000);
    let mut image: Image<RGB> = Image::new(height, width);

    // timed block //
    let now = time::Instant::now();

    let (mut red_s, mut red_e) = (Vec2Di::new(20, 34), Vec2Di::new(744, 400));
    let (mut green_s, mut green_e) = (Vec2Di::new(120, 434), Vec2Di::new(444, 400));
    let (mut blue_s, mut blue_e) = (Vec2Di::new(330,463), Vec2Di::new(594, 200));
    let mut ybuffer = vec![i32::MIN; image.width];

    rasterize_2D(&mut image, &mut red_s, &mut red_e, colors::RED, &mut ybuffer);
    image.write_tga_file("red.tga", true, true).unwrap();

    rasterize_2D(&mut image, &mut green_s, &mut green_e, colors::GREEN, &mut ybuffer);
    image.write_tga_file("green.tga", true, true).unwrap();

    rasterize_2D(&mut image, &mut blue_s, &mut blue_e, colors::BLUE, &mut ybuffer);
    image.write_tga_file("blue.tga", true, true).unwrap();

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);

}



