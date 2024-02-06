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
//use triangle_linesweep::*;
use std::time;
use types::*;



fn main() {
    let (height, width) = (200, 200);
    let mut image: Image<RGB> = Image::new(height, width);

    let mut f1 = Face {vertices: [
        Vertex::new(10.0, 70.0, 0.0),
        Vertex::new(50.0, 160.0, 0.0),
        Vertex::new(70.0, 80.0, 0.0)
    ]};

    let mut f2 = Face {vertices: [
        Vertex::new(180.0, 50.0, 0.0),
        Vertex::new(150.0, 10.0, 0.0),
        Vertex::new(70.0, 180.0, 0.0)
    ]};

    let mut f3 = Face {vertices: [
        Vertex::new(180.0, 150.0, 0.0),
        Vertex::new(120.0, 160.0, 0.0),
        Vertex::new(130.0, 180.0, 0.0)
    ]};

    // timed block //
    let now = time::Instant::now();

    triangle(&mut image, &mut f1.vertices, colors::RED).unwrap();
    triangle(&mut image, &mut f2.vertices, colors::GREEN).unwrap();
    triangle(&mut image, &mut f3.vertices, colors::BLUE).unwrap();

    let time_taken = now.elapsed();
    // end of timed block //

    image.write_tga_file("img.tga", true, true).unwrap();
}



