mod tgaimage;
mod line;
mod obj;
mod rasterizer;
mod shader;
mod transform;

use crate::shader::GouraudShader;
use crate::shader::Shader;
use crate::tgaimage::*;
use crate::obj::*;
use crate::transform::*;
use crate::rasterizer::triangle;
use glam::*;
use std::time;


fn main() {

    let mut image = Image::new(1024, 1024);

    let mut zbuffer = vec![f32::MIN; image.width * image.height];
    let faces_textures_normals = parse_obj("african_head.obj");
    let mut texture_img = convert_from_tinytga("texture.tga");

    let mut shader = GouraudShader::new();
    let transform = initialize_transform(image.height, image.width);

    // timed block //
    let now = time::Instant::now();

    for tup in faces_textures_normals {
        let (face, texture_face, normals) = (tup.0, tup.1, tup.2);
        let face = Shader::<RGB>::vertex(&mut shader, face, normals, Vec3::new(0.0, 0.0, 1.0), &transform);
        triangle(&mut image, &mut texture_img, face, texture_face, &shader, &mut zbuffer);
    }

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);
    image.write_tga_file("img.tga", true, false).unwrap();

}



