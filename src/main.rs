mod tgaimage;
mod line;
mod obj;
mod rasterizer;
mod shader;
mod transform;

use crate::shader::*;
use crate::tgaimage::*;
use crate::obj::*;
use crate::transform::*;
use crate::rasterizer::triangle;
use glam::*;
use std::time;


fn main() {

    // fetch image and texture
    let mut image = Image::new(1024, 1024);
    let mut image2 = Image::new(1024, 1024);
    let mut texture_image = convert_from_tinytga("texture.tga");

    // inst some stuff
    let mut zbuffer = vec![f32::MIN; image.width * image.height];
    let obj_faces = parse_obj("african_head.obj");
    let transform = initialize_transform(image.height, image.width);

    // inst shaders
    let mut shader = GouraudShader::new();
    let mut text_shader = GouraudTextureShader::new(texture_image.clone());

    // timed block //
    let now = time::Instant::now();

    for mut obj_face in obj_faces {
        // map text. coords into text. image
        obj_face.texture_vertices = obj_face.texture_vertices.map(|v| {
            Vec3::new(
                v.x * texture_image.width as f32,
                texture_image.height as f32 - v.y * texture_image.height as f32,
                0.0
            ).floor() 
        });

        let screen_coords = Shader::<RGB>::vertex(&mut text_shader, obj_face.clone(), Vec3::new(-1.0, -1.0, 3.0).normalize(), &transform);
        Shader::<RGB>::vertex(&mut shader, obj_face, Vec3::new(-1.0, -1.0, 3.0).normalize(), &transform);
        triangle(&mut image, &mut texture_image, &shader, screen_coords,  &mut zbuffer);
        triangle(&mut image2, &mut texture_image, &text_shader, screen_coords,  &mut zbuffer);
    }

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);
    image.write_tga_file("img.tga", true, false).unwrap();
    image2.write_tga_file("img2.tga", true, false).unwrap();

}



