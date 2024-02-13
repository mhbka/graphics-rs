use crate::types::*;
use crate::tgaimage::*;
use std::cmp::{min, max};



// Calculate barycentric weights, given 3 vertices and a point
fn barycentric(vertices: &[Vec2Di; 3], p: &Vec2Di) -> Vec3Df {
    let a = Vec3Df { x: vertices[2].x as f32 - vertices[0].x as f32, y: vertices[1].x as f32 - vertices[0].x as f32, z: vertices[0].x as f32 - p.x as f32 };
    let b = Vec3Df { x: vertices[2].y as f32 - vertices[0].y as f32, y: vertices[1].y as f32 - vertices[0].y as f32, z: vertices[0].y as f32 - p.y as f32 };

    let u = a.cross_product(&b);

    // Check for degenerate triangle (ie, cross product result is zero);
    // if yes, return vec with a negative value
    if u.z.abs() < 1.0 {
        return Vec3Df { x: -1.0, y: 1.0, z: 1.0 };
    }

    Vec3Df {
        x: 1.0 - (u.x + u.y) / u.z,
        y: u.y / u.z,
        z: u.x / u.z,
    }
}

// Convert barycentric coords into a 3D point
fn bary_to_point(bc_vertex: &Vec3Df, vertices: &[Vec2Di; 3]) -> Vec2Di {
    Vec2Di {
        x: (bc_vertex.x*vertices[0].x as f32 + bc_vertex.y*vertices[1].x as f32 + bc_vertex.z*vertices[2].x as f32) as i32,
        y: (bc_vertex.x*vertices[0].y as f32 + bc_vertex.y*vertices[1].y as f32 + bc_vertex.z*vertices[2].y as f32) as i32,
    }
}

// Triangle rasterization function with depth buffer + texture
pub fn triangle<T>(image: &mut Image<T>, texture_image: &Image<T>, face: Face, texture_face: Face, zbuffer: &mut Vec<f32>) 
where T: ColorSpace + Copy + std::fmt::Debug {

    // scale [0,1] coords into image pixel coords
    let face_2d = face.vertices.map(|v| {
        Vec2Di {
            x: ((1.0 + v.x)*image.width as f32 / 2.0) as i32,
            y: ((1.0 + v.y)* image.height as f32 / 2.0) as i32
        }
    });

    // scale texture image too (idk why but this one doesn't use transform + scaling like above)
    // also requires VFLIP THE TEXTURE ? wat dafuq
    let texture_face_2d = texture_face.vertices.map(|v|{
        Vec2Di {
            x: (v.x*texture_image.width as f32) as i32,
            y: (texture_image.height as f32 - v.y* texture_image.height as f32) as i32
        }
    });

    // shrink bounding box to rasterize over
    let mut bboxmin = Vec2Di { x: image.width as i32 - 1, y: image.height as i32 - 1 };
    let mut bboxmax = Vec2Di { x: 0, y: 0 };
    let clamp = Vec2Di { x: image.width as i32 - 1, y: image.height as i32 - 1 };

    for vertex in &face_2d {
        bboxmin.x = max(0, min(bboxmin.x, vertex.x));
        bboxmin.y = max(0, min(bboxmin.y, vertex.y));

        bboxmax.x = min(clamp.x, max(bboxmax.x, vertex.x));
        bboxmax.y = min(clamp.y, max(bboxmax.y, vertex.y));
    }

    // loop over bounding box pixels for valid baryometric + depth buffer check
    for p_x in bboxmin.x .. bboxmax.x {
        for p_y in bboxmin.y .. bboxmax.y {
            let bc_screen = barycentric(&face_2d, &Vec2Di {x: p_x, y: p_y});
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
                let texture_color = texture_image.data[(texture_pixel_coord.x + texture_pixel_coord.y*texture_image.height as i32) as usize];
                
                // println!("{:?}: {:?}", texture_pixel_coord, texture_color);

                // set actual pixel with texture pixel's color
                zbuffer[(p_x + p_y*image.width as i32) as usize] = p_z;
                image.set(p_x as usize, p_y as usize, texture_color).unwrap();
            }
        }
    }
}
