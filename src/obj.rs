use std::fs;
use crate::tgaimage::*;
use crate::line::*;
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


pub fn draw_obj<T>(filepath: &str, image: &mut Image<T>, color: T)
where T: ColorSpace + Copy {
    let faces = parse_obj(filepath);
    let translation = 1.0;
    let scaling = 2.0;
    let mut err_vert = 0;

    for face in faces {
        for i in 0..face.vertices.len() {
            let v0 = &face.vertices[i];
            let v1 = &face.vertices[(i+1) % face.vertices.len()];
            
            let start = Coord {
                x: ((v0.x+translation) * image.width as f32/scaling) as i32,
                y: ((v0.y+translation) * image.height as f32/scaling) as i32
            };

            let end = Coord {
                x: ((v1.x+translation) * image.width as f32/scaling) as i32,
                y: ((v1.y+translation) * image.height as f32/scaling) as i32 
            };

            match line(image, start, end, color) {
                Ok(_) => continue,
                Err(_) => err_vert+=1,
            };
        }
    }
}

pub fn parse_obj(filepath: &str) -> Vec<Face> { 
    let contents = fs::read_to_string(filepath)
        .expect(&format!("No filepath: {filepath}")[..]);

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    for line in contents.lines() {
        if line.starts_with("v ") {
            match parse_vertex(&line) {
                Ok((_, vertex)) => vertices.push(vertex),
                Err(_) => continue
            }
        }

        else if line.starts_with("vt ") {
            //ignore
        }

        else if line.starts_with("vn ") {
            //ignore
        }

        else if line.starts_with("f ") {
            match parse_face(&line, &vertices) {
                Ok((_, mut returned_faces)) => faces.push(returned_faces.remove(0)),
                Err(_) => continue,
            }
        }
    }
    faces   
}

// Vertex parsing

fn parse_vertex(input: &str) -> IResult<&str, Vertex> {
    let (input, _) = char('v')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, (x, _, y, _, z)) = tuple((float, space1, float, space1, float))(input)?;
    Ok((input, Vertex { x, y, z }))
}

// Face parsing

fn parse_face<'a>(input: &'a str, vertices: &Vec<Vertex>) -> IResult<&'a str, Vec<Face>> {
    let (input, _) = char('f')(input)?;
    let (input, _) = multispace0(input)?;

    let (input, v1_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v2_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v3_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    
    let mut faces = Vec::with_capacity(3);
    faces.push(Face {vertices: [vertices[v1_vec[0]-1], vertices[v2_vec[0]-1], vertices[v3_vec[0]-1]]});
    //faces.push(Face {v1: vertices[v1_vec[1]-1], v2: vertices[v2_vec[1]-1], v3: vertices[v3_vec[1]-1]});
    //faces.push(Face {v1: vertices[v1_vec[2]-1], v2: vertices[v2_vec[2]-1], v3: vertices[v3_vec[2]-1]});

    Ok((input, faces))
}
