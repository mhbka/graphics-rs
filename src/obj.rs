use std::fs;
use nalgebra_glm::*;

use crate::tgaimage::*;
use crate::triangle_bary::*;
use crate::types::*;
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

        // calculate vector of 2 sides of the face
        let side_1 = glm::Vec3::new() {
            x: face.vertices[1].x - face.vertices[0].x,
            y: face.vertices[1].y - face.vertices[0].y,
            z: face.vertices[1].z - face.vertices[0].z,
        };

        let side_2 = Vec3Df {
            x: face.vertices[2].x - face.vertices[0].x,
            y: face.vertices[2].y - face.vertices[0].y,
            z: face.vertices[2].z - face.vertices[0].z,
        };

        // calculate normal of the face using the 2 sides, and normalize
        let mut normal = side_1.cross_product(&side_2);
        normal.normalize();

        let rrr = glm::vec3();
            
        // calculate weight of light (scalar product of normal + z-coordinate)
        let light = Vec3Df {x:0.0, y:0.0, z:1.0};
        let intensity = normal.scalar_product(&light);
        if intensity > 0.0 {
            triangle(image, &mut texture_img, &mut face, texture_face, &mut zbuffer, intensity);
        }
    }
}


// parse the object from file
// returns a (Face, Face) tuple; the 1st face contains actual vertices, and the 2nd contains texture coordinates (ie, with 0 z-value).
pub fn parse_obj(filepath: &str) -> Vec<(Face, Face)> { 
    let contents = fs::read_to_string(filepath)
        .expect(&format!("No such file at this filepath: {filepath}")[..]);

    let mut vertices = Vec::new();
    let mut texture_coords = Vec::new();
    let mut faces_and_textures = Vec::new();

    for line in contents.lines() {
        if line.starts_with("v ") {
            match parse_vertex(&line) {
                Ok((_, vec)) => vertices.push(vec),
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
fn parse_vertex(input: &str) -> IResult<&str, Vec3Df> {
    let (input, _) = char('v')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, (x, _, y, _, z)) = tuple((float, space1, float, space1, float))(input)?;
    Ok((input, Vec3Df { x, y, z }))
}


// face parsing
// in each of the 3 triplets, parses 1st value (actual vertice) and 2nd value (texture vertice)
fn parse_face<'a>(input: &'a str, vertices: &Vec<Vec3Df>, texture_coords: &Vec<Vec3Df>) -> IResult<&'a str, (Face, Face)> {
    let (input, _) = char('f')(input)?;
    let (input, _) = multispace0(input)?;

    let (input, v1_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v2_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v3_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    
    let face = Face {vertices: [vertices[v1_vec[0]-1], vertices[v2_vec[0]-1], vertices[v3_vec[0]-1]]};
    let face_textures = Face {vertices: [texture_coords[v1_vec[1]-1], texture_coords[v2_vec[1]-1], texture_coords[v3_vec[1]-1]]};

    Ok((input, (face, face_textures)))
}


// texture parsing
fn parse_texture(input: &str) -> IResult<&str, Vec3Df> {
    let (input, _) = tag("vt")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, (x, _, y, _, z)) = tuple((float, space1, float, space1, float))(input)?;

    // Note: z seems to always be 0, but we need to store f32 so we use Vec3Df anyway
    Ok((input, Vec3Df { x, y, z }))
}   

// texture face parsing
fn parse_texture_face<'a> (input: &'a str, texture_vertices: &Vec3Df) -> IResult<&'a str, [usize; 3]> {
    let (input, _) = tag("f")(input)?;
    let (input, _) = multispace0(input)?;
    
    let (input, v1_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v2_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v3_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;

    Ok((input, [v1_vec[1], v2_vec[1], v3_vec[1]]))
}