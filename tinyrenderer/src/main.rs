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
    let eye = Vec3::new(1.0, 1.0, 4.0);
    let centre = Vec3::ZERO;
    let up = Vec3::Y;
    let light_source = Vec3::new(1.0, 1.0, 0.0);
    
    let obj_faces = parse_obj(&format!("assets/{obj_name}/{obj_name}.obj"));
    let texture_image: Image<RGB> = convert_from_tinytga(&format!("assets/{obj_name}/{obj_name}_diffuse.tga"));
    let normal_image: Image<RGB> = convert_from_tinytga(&format!("assets/{obj_name}/{obj_name}_nm.tga"));
    let tangent_normal_image: Image<RGB> = convert_from_tinytga(&format!("assets/{obj_name}/{obj_name}_nm_tangent.tga"));
    let specular_image: Image<Grayscale> = convert_from_tinytga(&format!("assets/{obj_name}/{obj_name}_spec.tga"));

    let transform = initialize_transform(height, width, eye, centre, up);
    let depth_transform = initialize_transform(height, width, light_source, centre, up);
    let shadow_transform = 
        depth_transform.get_whole_transform() * (
            transform 
            .get_whole_transform() // this transforms from object space into screen space,
            .inverse() // so its inverse transforms from screen space to object space,
        ); // so the overall effect is: screen space -> object space -> shadow screen space

    let model = Model {
        texture_image,
        normal_image,
        tangent_normal_image,
        specular_image
    };

    // shadow shader (including depth shader)
    let mut depth_img: Image<RGB> = Image::new(width, height);
    let mut shadowbuffer = vec![f32::MIN; width * height];

    let mut shadow_img: Image<RGB> = Image::new(width, height);
    let mut zbuffer = vec![f32::MIN; width * height];

    // instantiate shaders
    let mut depth_shader = DepthShader::new(light_source, depth_transform.clone());

    // timed block //
    let now = time::Instant::now();

    // first, calculate shadowbuffer
    for obj_face in obj_faces.clone() {
        let ndc = Shader::<RGB>::vertex(&mut depth_shader, obj_face.clone(), light_source);
        let depth_screen_coords = ndc.map(|v| depth_transform.viewport_transform(v));
        triangle(&mut depth_img, &depth_shader, depth_screen_coords,  &mut shadowbuffer);
    }

    // instantiate and use actual shader
    let mut shadow_shader = ShadowShader::new(model, transform, shadow_transform, shadowbuffer);
    for obj_face in obj_faces {
        let ndc = Shader::<RGB>::vertex(&mut shadow_shader, obj_face.clone(), light_source);
        let screen_coords = ndc.map(|v| transform.viewport_transform(v));
        triangle(&mut shadow_img, &shadow_shader, screen_coords,  &mut zbuffer);
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
    depth_img.write_tga_file("output/depth.tga", true, false).unwrap();
    shadow_img.write_tga_file("output/shadow.tga", true, false).unwrap();
}



