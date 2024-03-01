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
use std::{env, time};


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // instantiate common things
    let (height, width) = (1024, 1024);
    
    let obj_faces = parse_obj("assets/african_head.obj");
    let texture_image: Image<RGB> = convert_from_tinytga("assets/grid.tga");
    let normal_image: Image<RGB> = convert_from_tinytga("assets/african_head_nm.tga");
    let tangent_normal_image: Image<RGB> = convert_from_tinytga("assets/african_head_nm_tangent.tga");
    let specular_image: Image<Grayscale> = convert_from_tinytga("assets/african_head_spec.tga");
    let transform = initialize_transform(height, width);

    let model = Model {
        texture_image,
        normal_image,
        tangent_normal_image,
        specular_image
    };

    // gouraud shader w/ texture (starts at 2 cuz i deleted the first one lol)
    let mut gouraud_img: Image<RGB> = Image::new(width, height);
    let mut zbuffer = vec![f32::MIN; width * height];

    // normal-mapped shader w/ texture
    let mut normal_map_img: Image<RGB> = Image::new(width, height);
    let mut zbuffer2 = zbuffer.clone();

    // normal-mapped shader w/ texture and specular mapping
    let mut normal_spec_img: Image<RGB> = Image::new(width, height);
    let mut zbuffer3 = zbuffer.clone();

    // darboux shader
    let mut tangent_normal_img: Image<RGB> = Image::new(width, height);
    let mut zbuffer4 = zbuffer.clone();

    // instantiate shaders
    let mut texture_shader = GouraudShader::new(model.clone(), transform.clone());
    let mut normal_mapped_shader = NormalMappedShader::new(model.clone(), transform.clone());
    let mut normal_specular_shader = NormalSpecularShader::new(model.clone(), transform.clone());
    let mut tangent_normal_shader = TangentNormalShader::new(model.clone(), transform.clone());

    // timed block //
    let now = time::Instant::now();

    for obj_face in obj_faces {
        let light_dir = Vec3::new(0.0, 0.0, 3.0).normalize();
        
        let ndc = Shader::<RGB>::vertex(&mut texture_shader, obj_face.clone(), light_dir);
        let ndc2 = Shader::<RGB>::vertex(&mut normal_mapped_shader, obj_face.clone(), light_dir);
        let ndc3 = Shader::<RGB>::vertex(&mut normal_specular_shader, obj_face.clone(), light_dir);
        let ndc4 = Shader::<RGB>::vertex(&mut tangent_normal_shader, obj_face.clone(), light_dir);

        assert_eq!(ndc, ndc2);
        assert_eq!(ndc2, ndc3);
        assert_eq!(ndc3, ndc4);

        let screen_coords = ndc.map(|v| transform.viewport_transform(v));

        triangle(&mut gouraud_img, &texture_shader, screen_coords,  &mut zbuffer);
        triangle(&mut normal_map_img, &normal_mapped_shader, screen_coords,  &mut zbuffer2);
        triangle(&mut normal_spec_img, &normal_specular_shader, screen_coords,  &mut zbuffer3);
        triangle(&mut tangent_normal_img, &tangent_normal_shader, screen_coords,  &mut zbuffer4);
    }

    let time_taken = now.elapsed();
    // end of timed block //
    
    add_axis_lines(&mut gouraud_img, transform.get_whole_transform());
    add_axis_lines(&mut normal_map_img, transform.get_whole_transform());
    add_axis_lines(&mut normal_spec_img, transform.get_whole_transform());
    add_axis_lines(&mut tangent_normal_img, transform.get_whole_transform());

    println!("{:?}", time_taken);
    gouraud_img.write_tga_file("output/gouraud.tga", true, false).unwrap();
    normal_map_img.write_tga_file("output/normal_map.tga", true, false).unwrap();
    normal_spec_img.write_tga_file("output/normal_spec.tga", true, false).unwrap();
    tangent_normal_img.write_tga_file("output/tang_normal_map.tga", true, false).unwrap();
}



