mod tgaimage;
mod line;
mod obj;
mod rasterizer;
mod shaders;
mod transform;

use crate::shaders::*;
use crate::tgaimage::*;
use crate::obj::*;
use crate::transform::*;
use crate::rasterizer::triangle;
use glam::*;
use std::time;


fn main() {

    // fetch image and texture
    let mut image: Image<RGB> = Image::new(1024, 1024);
    let mut image2: Image<RGB> = Image::new(1024, 1024);
    let texture_image: Image<RGB> = convert_from_tinytga("texture.tga");

    // inst some stuff
    let mut zbuffer = vec![f32::MIN; image.width * image.height];
    let mut zbuffer2 = vec![f32::MIN; image.width * image.height];
    let obj_faces = parse_obj("african_head.obj");
    let transform = initialize_transform(image.height, image.width);

    // inst shaders
    let mut shader = GouraudShader::new(transform.clone());
    let mut texture_shader = GouraudTextureShader::new(texture_image.clone(), transform.clone());

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
        
        let screen_coords = Shader::<RGB>::vertex(&mut texture_shader, obj_face.clone(), Vec3::new(-1.0, -1.0, 3.0).normalize());
        let screen_coords2 = Shader::<RGB>::vertex(&mut shader, obj_face, Vec3::new(-1.0, -1.0, 3.0).normalize());
        assert_eq!(screen_coords, screen_coords2);
        triangle(&mut image, &shader, screen_coords, &mut zbuffer);
        triangle(&mut image2, &texture_shader, screen_coords2,  &mut zbuffer2);
    }

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);
    image.write_tga_file("img.tga", true, false).unwrap();
    image2.write_tga_file("img2.tga", true, false).unwrap();

}



