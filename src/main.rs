mod tgaimage;
mod line;
mod obj;
mod rasterizer;
mod shaders;
mod transform;
mod model;

use crate::line::add_axis_lines;
use crate::shaders::*;
use crate::tgaimage::*;
use crate::obj::*;
use crate::transform::*;
use crate::rasterizer::triangle;
use crate::model::*;
use glam::*;
use std::time;


fn main() {
    // instantiate common things
    let (height, width) = (1024, 1024);
    let obj_faces = parse_obj("assets/african_head.obj");
    let texture_image: Image<RGB> = convert_from_tinytga("assets/african_head_texture.tga");
    let normal_image: Image<RGB> = convert_from_tinytga("assets/african_head_nm.tga");
    let tangent_normal_image: Image<RGB> = convert_from_tinytga("assets/african_head_nm_tangent.tga");
    let specular_image: Image<Grayscale> = convert_from_tinytga("assets/african_head_spec.tga");
    let transform = initialize_transform(height, width);

    let model = Model {
        obj_faces,
        texture_image,
        normal_image,
        tangent_normal_image,
        specular_image
    };

    // gouraud shader w/ texture (starts at 2 cuz i deleted the first one lol)
    let mut image2: Image<RGB> = Image::new(width, height);
    let mut zbuffer2 = vec![f32::MIN; image2.width * image2.height];

    // normal-mapped shader w/ texture
    let mut image3: Image<RGB> = Image::new(width, height);
    let mut zbuffer3 = zbuffer2.clone();

    // normal-mapped shader w/ texture and specular mapping
    let mut image4: Image<RGB> = Image::new(width, height);
    let mut zbuffer4 = zbuffer2.clone();

    // darboux shader
    let mut image5: Image<RGB> = Image::new(width, height);
    let mut zbuffer5 = zbuffer2.clone();

    // instantiate shaders
    let mut texture_shader = GouraudShader::new(model.clone(), transform.clone());
    let mut normal_mapped_shader = NormalMappedShader::new(model.clone(), transform.clone());
    let mut normal_specular_shader = NormalSpecularShader::new(model.clone(), transform.clone());
    let mut tangent_normal_shader = TangentNormalShader::new(model.clone(), transform.clone());

    // timed block //
    let now = time::Instant::now();

    for mut obj_face in obj_faces {
        // test something
        let obj_face2 = obj_face.clone();

        // map texture coords into texture pixels
        obj_face.texture_vertices = obj_face.texture_vertices.map(|v| {
            Vec3::new(
                v.x * texture_image.width as f32,
                texture_image.height as f32 - v.y * texture_image.height as f32,
                0.0
            ).floor() 
        });
        
        let light_dir = Vec3::new(1.0, 1.0, 3.0).normalize();
        
        let screen_coords2 = Shader::<RGB>::vertex(&mut texture_shader, obj_face.clone(), light_dir);
        let screen_coords3 = Shader::<RGB>::vertex(&mut normal_mapped_shader, obj_face.clone(), light_dir);
        let screen_coords4 = Shader::<RGB>::vertex(&mut normal_specular_shader, obj_face.clone(), light_dir);
        let screen_coords5 = Shader::<RGB>::vertex(&mut tangent_normal_shader, obj_face2.clone(), light_dir);

        assert_eq!(screen_coords2, screen_coords3);
        assert_eq!(screen_coords3, screen_coords4);
        assert_eq!(screen_coords4, screen_coords5);

        triangle(&mut image2, &texture_shader, screen_coords2,  &mut zbuffer2);
        triangle(&mut image3, &normal_mapped_shader, screen_coords3,  &mut zbuffer3);
        triangle(&mut image4, &normal_specular_shader, screen_coords4,  &mut zbuffer4);
        triangle(&mut image5, &tangent_normal_shader, screen_coords5,  &mut zbuffer5);
    }

    let time_taken = now.elapsed();
    // end of timed block //
    
    add_axis_lines(&mut image2, transform.get_whole_transform());
    add_axis_lines(&mut image3, transform.get_whole_transform());
    add_axis_lines(&mut image4, transform.get_whole_transform());
    add_axis_lines(&mut image5, transform.get_whole_transform());

    println!("{:?}", time_taken);
    image2.write_tga_file("output/img2.tga", true, false).unwrap();
    image3.write_tga_file("output/img3.tga", true, false).unwrap();
    image4.write_tga_file("output/img4.tga", true, false).unwrap();
    image5.write_tga_file("output/img5.tga", true, false).unwrap();
}



