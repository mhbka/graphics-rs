use std::fs;
use glam::*;

use crate::tgaimage::*;
use crate::triangle_bary::*;
use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, space1, digit1},
    number::complete::float,
    combinator::map_res,
    sequence::tuple,
    multi::separated_list0,
    IResult,
};



// draw the object into the image
pub fn draw_obj(obj_filepath: &str, image: &mut Image<RGB>) {
    let mut zbuffer = vec![f32::MIN; image.width * image.height];

    let faces_and_textures = parse_obj(obj_filepath);

    let mut texture_img = convert_from_tinytga();
    
    for tup in faces_and_textures {

        // destruct into the face and texture
        let (mut face, texture_face) = (tup.0, tup.1);

        // create vectors of 2 sides of the face
        let side_1 = Vec3::new(
            face[1].x - face[0].x,
            face[1].y - face[0].y,
            face[1].z - face[0].z,
        );

        let side_2 = Vec3::new(
            face[2].x - face[0].x,
            face[2].y - face[0].y,
            face[2].z - face[0].z,
        );

        // calculate normal of the face using the 2 sides, and normalize
        let normal = side_1.cross(side_2).normalize();
            
        // calculate weight of light (scalar product of normal + z-coordinate)
        let light = Vec3::new(0.0, 0.0, 1.1);
        let intensity = normal.dot(light);
        if intensity > 0.0 {
            triangle(image, &mut texture_img, &mut face, texture_face, &mut zbuffer, intensity);
        }
    }
}


// parse the object from file
// returns tuple of 3 Vec3; the 1st contains actual vertices, and the 2nd contains corresponding texture coords.
pub fn parse_obj(filepath: &str) -> Vec<([Vec3; 3], [Vec3; 3])> { 
    let contents = fs::read_to_string(filepath)
        .expect(&format!("No such file at this filepath: {filepath}")[..]);

    let mut vertices = Vec::new();
    let mut texture_coords = Vec::new();
    let mut faces_and_textures = Vec::new();

    for line in contents.lines() {
        if line.starts_with("v ") {
            match parse_vertex(&line) {
                Ok((_, vec)) =>vertices.push(vec),
                Err(_) => continue
            }
        }

        else if line.starts_with("vt ") {
            match parse_texture(&line) {
                Ok((_, coord)) => texture_coords.push(coord),
                Err(_) => continue
            }
        }

        else if line.starts_with("vn ") {
            //ignore
        }

        else if line.starts_with("f ") {
            match parse_face(&line, &vertices, &texture_coords) {
                Ok((_, (face, texture_face))) => faces_and_textures.push((face, texture_face)),
                Err(_) => continue,
            }
        }
    }
    faces_and_textures   
}


// vertex parsing
fn parse_vertex(input: &str) -> IResult<&str, Vec3> {
    let (input, _) = char('v')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, (x, _, y, _, z)) = tuple((float, space1, float, space1, float))(input)?;
    Ok((input, Vec3 { x, y, z }))
}


// face parsing
// in each of the 3 triplets, parses 1st value (actual vertice) and 2nd value (texture vertice)
fn parse_face<'a>(input: &'a str, vertices: &Vec<Vec3>, texture_coords: &Vec<Vec3>) -> IResult<&'a str, ([Vec3; 3], [Vec3; 3])> {
    let (input, _) = char('f')(input)?;
    let (input, _) = multispace0(input)?;

    let (input, v1_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v2_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v3_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    
    let face = [vertices[v1_vec[0]-1], vertices[v2_vec[0]-1], vertices[v3_vec[0]-1]];
    let face_textures = [texture_coords[v1_vec[1]-1], texture_coords[v2_vec[1]-1], texture_coords[v3_vec[1]-1]];

    Ok((input, (face, face_textures)))
}


// texture parsing
fn parse_texture(input: &str) -> IResult<&str, Vec3> {
    let (input, _) = tag("vt")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, (x, _, y, _, z)) = tuple((float, space1, float, space1, float))(input)?;

    Ok((input, Vec3 { x, y, z })) // Note: z seems to always be 0, but we'll store it anyway just in case
}   