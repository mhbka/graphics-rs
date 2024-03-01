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
    let obj_name = "diablo3_pose";

    // instantiate common things
    let (height, width) = (1024, 1024);
    let eye = Vec3::new(3.0, 3.0, 9.0);
    let centre = Vec3::ZERO;
    let up = Vec3::Y;
    let light_source = Vec3::new(3.0, 3.0, 1.0);
    
    let obj_faces = parse_obj(&format!("assets/{obj_name}/{obj_name}.obj"));
    let texture_image: Image<RGB> = convert_from_tinytga(&format!("assets/{obj_name}/{obj_name}_diffuse.tga"));
    let normal_image: Image<RGB> = convert_from_tinytga(&format!("assets/{obj_name}/{obj_name}_nm.tga"));
    let tangent_normal_image: Image<RGB> = convert_from_tinytga(&format!("assets/{obj_name}/{obj_name}_nm_tangent.tga"));
    let specular_image: Image<Grayscale> = convert_from_tinytga(&format!("assets/{obj_name}/{obj_name}_spec.tga"));

    let transform = initialize_transform(
        height,
        width,
        eye,
        centre,
        up
    );

    let depth_transform = initialize_transform( // same as above, but using light_source as the "eye"
        height,
        width,
        light_source,
        centre,
        up
    );

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

    // tangent normal shader
    let mut tangent_normal_img: Image<RGB> = Image::new(width, height);
    let mut zbuffer4 = zbuffer.clone();

    // WIP: shadow shader
    let mut depth_img: Image<RGB> = Image::new(width, height);
    let mut shadowbuffer = zbuffer.clone();

    // instantiate shaders
    let mut texture_shader = GouraudShader::new(model.clone(), transform.clone());
    let mut normal_mapped_shader = NormalMappedShader::new(model.clone(), transform.clone());
    let mut normal_specular_shader = NormalSpecularShader::new(model.clone(), transform.clone());
    let mut tangent_normal_shader = TangentNormalShader::new(model.clone(), transform.clone());
    let mut depth_shader = DepthShader::new(light_source, depth_transform.clone());

    // timed block //
    let now = time::Instant::now();

    for obj_face in obj_faces {
        
        let ndc = Shader::<RGB>::vertex(&mut texture_shader, obj_face.clone(), light_source);
        let ndc2 = Shader::<RGB>::vertex(&mut normal_mapped_shader, obj_face.clone(), light_source);
        let ndc3 = Shader::<RGB>::vertex(&mut normal_specular_shader, obj_face.clone(), light_source);
        let ndc4 = Shader::<RGB>::vertex(&mut tangent_normal_shader, obj_face.clone(), light_source);
        let ndc5 = Shader::<RGB>::vertex(&mut depth_shader, obj_face.clone(), light_source);

        assert_eq!(ndc, ndc2);
        assert_eq!(ndc2, ndc3);
        assert_eq!(ndc3, ndc4);

        let screen_coords = ndc.map(|v| transform.viewport_transform(v));
        let depth_screen_coords = ndc5.map(|v| depth_transform.viewport_transform(v));

        triangle(&mut gouraud_img, &texture_shader, screen_coords,  &mut zbuffer);
        triangle(&mut normal_map_img, &normal_mapped_shader, screen_coords,  &mut zbuffer2);
        triangle(&mut normal_spec_img, &normal_specular_shader, screen_coords,  &mut zbuffer3);
        triangle(&mut tangent_normal_img, &tangent_normal_shader, screen_coords,  &mut zbuffer4);
        triangle(&mut depth_img, &depth_shader, depth_screen_coords,  &mut shadowbuffer);

    }

    let time_taken = now.elapsed();
    // end of timed block //
    
    /* 
    add_axis_lines(&mut gouraud_img, transform.get_whole_transform());
    add_axis_lines(&mut normal_map_img, transform.get_whole_transform());
    add_axis_lines(&mut normal_spec_img, transform.get_whole_transform());
    add_axis_lines(&mut tangent_normal_img, transform.get_whole_transform());
    */

    println!("{:?}", time_taken);
    gouraud_img.write_tga_file("output/gouraud.tga", true, false).unwrap();
    normal_map_img.write_tga_file("output/normal_map.tga", true, false).unwrap();
    normal_spec_img.write_tga_file("output/normal_spec.tga", true, false).unwrap();
    tangent_normal_img.write_tga_file("output/tang_normal_map.tga", true, false).unwrap();
    depth_img.write_tga_file("output/depth.tga", true, false).unwrap();
}



