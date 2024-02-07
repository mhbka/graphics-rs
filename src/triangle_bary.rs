use crate::types::*;
use crate::tgaimage::*;
use std::cmp::{min, max};



// Barycentric coords calculation
fn barycentric(vertex: &[Vec2Di; 3], p: &Vec2Di) -> Vec3Df {
    let a = Vec3Df { x: vertex[2].x as f32 - vertex[0].x as f32, y: vertex[1].x as f32 - vertex[0].x as f32, z: vertex[0].x as f32 - p.x as f32 };
    let b = Vec3Df { x: vertex[2].y as f32 - vertex[0].y as f32, y: vertex[1].y as f32 - vertex[0].y as f32, z: vertex[0].y as f32 - p.y as f32 };

    let u = a.cross_product(&b);

    // Check for degenerate triangle (cross product result is zero)
    if u.z.abs() < 1.0 {
        return Vec3Df { x: -1.0, y: 1.0, z: 1.0 }; // Degenerate triangle
    }

    Vec3Df {
        x: 1.0 - (u.x + u.y) / u.z,
        y: u.y / u.z,
        z: u.x / u.z,
    }
}

// Triangle rasterization function with depth buffer
pub fn triangle<T>(image: &mut Image<T>, face: Face,  color: T, zbuffer: &mut Vec<f32>) 
where T: ColorSpace + Copy {
    let face_2d = face.vertices.map(|v| {
        Vec2Di {
            x: ((1.0 + v.x)*image.width as f32 / 2.0) as i32,
            y: ((1.0 + v.y)* image.height as f32 / 2.0) as i32
        }
    });

    let mut bboxmin = Vec2Di { x: image.width as i32 - 1, y: image.height as i32 - 1 };
    let mut bboxmax = Vec2Di { x: 0, y: 0 };
    let clamp = Vec2Di { x: image.width as i32 - 1, y: image.height as i32 - 1 };

    for vertex in &face_2d {
        bboxmin.x = max(0, min(bboxmin.x, vertex.x));
        bboxmin.y = max(0, min(bboxmin.y, vertex.y));

        bboxmax.x = min(clamp.x, max(bboxmax.x, vertex.x));
        bboxmax.y = min(clamp.y, max(bboxmax.y, vertex.y));
    }

    

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
                zbuffer[(p_x + p_y*image.width as i32) as usize] = p_z;
                image.set(p_x as usize, p_y as usize, color).unwrap();
            }
        }
    }
}
