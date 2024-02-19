mod tgaimage;
mod line;
mod obj;
mod colors;
mod rasterizer;

use tgaimage::*;
use std::time;
use crate::obj::*;
use rasterizer::triangle;




fn main() {

    let mut image = Image::new(1024, 1024);

    let mut zbuffer = vec![f32::MIN; image.width * image.height];
    let faces_textures_normals = parse_obj("african_head.obj");
    let mut texture_img = convert_from_tinytga();

    // timed block //
    let now = time::Instant::now();

    for tup in faces_textures_normals {
        let (face, texture_face, normals) = (tup.0, tup.1, tup.2);
        triangle(&mut image, &mut texture_img, face, texture_face, normals, &mut zbuffer);
    }

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);
    image.write_tga_file("img.tga", true, false).unwrap();

}



