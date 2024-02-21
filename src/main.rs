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

    // instantiate common things
    let (height, width) = (1024, 1024);
    let obj_faces = parse_obj("african_head.obj");
    let texture_image = convert_from_tinytga("texture.tga");
    let transform = initialize_transform(height, width);

    // instantiate for gouraud shader
    let mut image: Image<RGB> = Image::new(width, height);
    let mut zbuffer = vec![f32::MIN; image.width * image.height];

    // instantiate for gouraud shader w/ texture
    let mut image2: Image<RGB> = Image::new(width, height);
    let mut zbuffer2 = vec![f32::MIN; image.width * image.height];

    // instantiate for normal-mapped shader w/ texture
    let mut image3: Image<RGB> = Image::new(width, height);
    let mut zbuffer3 = vec![f32::MIN; image.width * image.height];
    let normal_img = convert_from_tinytga("normal.tga");

    // instantiate shaders
    let mut shader = GouraudShader::new(transform.clone());
    let mut texture_shader = GouraudTextureShader::new(texture_image.clone(), transform.clone());
    let mut normal_mapped_shader = NormalCalcShader::new(texture_image.clone(), normal_img, transform.clone());
    

    // timed block //
    let now = time::Instant::now();

    for mut obj_face in obj_faces {
        // map texture coords into texture pixels
        obj_face.texture_vertices = obj_face.texture_vertices.map(|v| {
            Vec3::new(
                v.x * texture_image.width as f32,
                texture_image.height as f32 - v.y * texture_image.height as f32,
                0.0
            ).floor() 
        });
        
        let light_dir = Vec3::new(-1.0, -1.0, 3.0).normalize();

        let screen_coords = Shader::<RGB>::vertex(&mut texture_shader, obj_face.clone(), light_dir);
        let screen_coords2 = Shader::<RGB>::vertex(&mut shader, obj_face.clone(), light_dir);
        let screen_coords3 = Shader::<RGB>::vertex(&mut normal_mapped_shader, obj_face, light_dir);

        assert_eq!(screen_coords, screen_coords2);
        assert_eq!(screen_coords2, screen_coords3);

        triangle(&mut image, &shader, screen_coords, &mut zbuffer);
        triangle(&mut image2, &texture_shader, screen_coords2,  &mut zbuffer2);
        triangle(&mut image3, &normal_mapped_shader, screen_coords3,  &mut zbuffer3);
    }

    let time_taken = now.elapsed();
    // end of timed block //

    println!("{:?}", time_taken);
    image.write_tga_file("img.tga", true, false).unwrap();
    image2.write_tga_file("img2.tga", true, false).unwrap();
    image3.write_tga_file("img3.tga", true, false).unwrap();

}



