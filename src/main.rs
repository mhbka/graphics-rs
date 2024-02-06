mod tgaimage;
mod line;
mod obj;
mod types;
mod triangle;

use tgaimage::*;
use line::*;
use obj::*;
use std::time;



fn main() {

    let (height, width) = (1000, 1000);
    let color = RGB {r:255, g:255, b: 255};

    // timed block //
    let now = time::Instant::now();


    let time_taken = now.elapsed();
    // end of timed block //

}



