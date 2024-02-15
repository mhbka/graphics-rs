// use crate::types::*;
use crate::tgaimage::*;
use glam::*;
use std::cmp::{min, max};


// Calculate barycentric weights, given 3 vertices and a point
fn barycentric(vertices: &[Vec2; 3], p: &Vec2) -> Vec3 {
    let a = Vec3::new(vertices[2].x - vertices[0].x, vertices[1].x - vertices[0].x, vertices[0].x - p.x);
    let b = Vec3::new(vertices[2].y - vertices[0].y, vertices[1].y - vertices[0].y, vertices[0].y - p.y);
    let u = a.cross(b);

    // Check for degenerate triangle (ie, cross product result is zero);
    // if yes, return vec with a negative value
    if u.z.abs() < 1.0 {
        return Vec3::new(-1.0, 1.0, 1.0);
    }

    Vec3::new(
        1.0 - (u.x + u.y) / u.z,
        u.y / u.z,
        u.x / u.z,
    )
}


// Convert barycentric coords into a 3D point
fn bary_to_point(bc_vertex: &Vec3, vertices: &[Vec2; 3]) -> Vec2 {
    Vec2 {
        x: (bc_vertex.x*vertices[0].x + bc_vertex.y*vertices[1].x + bc_vertex.z*vertices[2].x) as i32,
        y: (bc_vertex.x*vertices[0].y + bc_vertex.y*vertices[1].y + bc_vertex.z*vertices[2].y) as i32,
    }
}


// Triangle rasterization function with depth buffer + texture + perspective etc
pub fn triangle<T>(
    image: &mut Image<T>, 
    texture_image: &mut Image<T>, 
    face: &mut Face, 
    texture_face: Face, 
    zbuffer: &mut Vec<f32>, 
    intensity: f32
) 
where T: ColorSpace + Copy + std::fmt::Debug {

    let c = -1.5; // distance from camera

    // transformation; perspective of camera from z=5 (i think)
    face.vertices = face.vertices.map(|v| {
        Vec3::new()
            x: v.x / (1.0 - (v.y/c)),
            y: v.y / (1.0 - (v.y/c)),
            z: v.z / (1.0 - (v.y/c))
        }
    }); 

    // scale [0,1] coords into image size
    let face_2d = face.vertices.map(|v| {
        Vec2 {
            x: ((1.0 + v.x)*image.width / 2.0) as i32,
            y: ((1.0 + v.y)* image.height / 2.0) as i32
        }
    });

    // scale texture image too (idk why but this one doesn't use transform + scaling like above)
    // also requires VFLIPPING ? wat dafuq
    let texture_face_2d = texture_face.vertices.map(|v|{
        Vec2 {
            x: (v.x*texture_image.width) as i32,
            y: (texture_image.height - v.y* texture_image.height) as i32
        }
    });

    // shrink bounding box to rasterize over
    let mut bboxmin = Vec2 { x: image.width as i32 - 1, y: image.height as i32 - 1 };
    let mut bboxmax = Vec2 { x: 0, y: 0 };
    let clamp = Vec2 { x: image.width as i32 - 1, y: image.height as i32 - 1 };

    for vertex in &face_2d {
        bboxmin.x = max(0, min(bboxmin.x, vertex.x));
        bboxmin.y = max(0, min(bboxmin.y, vertex.y));

        bboxmax.x = min(clamp.x, max(bboxmax.x, vertex.x));
        bboxmax.y = min(clamp.y, max(bboxmax.y, vertex.y));
    }

    // loop over bounding box pixels for valid baryometric + depth buffer check
    for p_x in bboxmin.x .. bboxmax.x {
        for p_y in bboxmin.y .. bboxmax.y {
            let bc_screen = barycentric(&face_2d, &Vec2 {x: p_x, y: p_y});
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            // depth buffer check for z-value
            let p_z = face.vertices[0].z * bc_screen.x
                        + face.vertices[1].z * bc_screen.y
                        + face.vertices[2].z * bc_screen.z;

            if p_z > zbuffer[(p_x + p_y*image.width as i32) as usize] {

                // use barycentric coordinates to locate corresponding pixel within texture_face in texture_img
                let texture_pixel_coord = bary_to_point(&bc_screen, &texture_face_2d);
                let mut texture_color = texture_image.data[(texture_pixel_coord.x + texture_pixel_coord.y*texture_image.height as i32) as usize];
                texture_color.shade(intensity);

                // set actual pixel with texture pixel's color
                zbuffer[(p_x + p_y*image.width as i32) as usize] = p_z;
                image.set(p_x as usize, p_y as usize, texture_color).unwrap();
            }
        }
    }
}
