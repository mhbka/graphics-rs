mod tgaimage;
mod line;
mod obj;

use tgaimage::*;
use line::*;
use obj::*;
use std::time;

struct Coord {
    x: i32, y: i32
}

fn main() {
    let count = 1_000_000;
    let now = time::Instant::now();



    let time_taken = now.elapsed();
    println!("{:?}", time_taken);
    
    // image.write_tga_file("line.tga", false, true).unwrap();
}



