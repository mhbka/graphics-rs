use crate::types::*;
use crate::tgaimage::*;
use std::cmp::{min, max};



// Barycentric coords calculation
fn barycentric(pts: &[Vec2Di; 3], p: &Vec2Di) -> Vec3Df {
    let a = Vec3Df { x: pts[2].x as f32 - pts[0].x as f32, y: pts[1].x as f32 - pts[0].x as f32, z: pts[0].x as f32 - p.x as f32 };
    let b = Vec3Df { x: pts[2].y as f32 - pts[0].y as f32, y: pts[1].y as f32 - pts[0].y as f32, z: pts[0].y as f32 - p.y as f32 };

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

// Triangle rasterization function
pub fn triangle<T>(image: &mut Image<T>, pts: &[Vec2Di; 3],  color: T) 
where T: ColorSpace + Copy {
    let mut bboxmin = Vec2Di { x: image.width as i32 - 1, y: image.height as i32 - 1 };
    let mut bboxmax = Vec2Di { x: 0, y: 0 };
    let clamp = Vec2Di { x: image.width as i32 - 1, y: image.height as i32 - 1 };

    for i in 0..3 {
        bboxmin.x = max(0, min(bboxmin.x, pts[i].x));
        bboxmin.y = max(0, min(bboxmin.y, pts[i].y));

        bboxmax.x = min(clamp.x, max(bboxmax.x, pts[i].x));
        bboxmax.y = min(clamp.y, max(bboxmax.y, pts[i].y));
    }

    for p_x in bboxmin.x .. bboxmax.x {
        for p_y in bboxmin.y .. bboxmax.y {
            let bc_screen = barycentric(pts, &Vec2Di {x: p_x, y: p_y});
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            image.set(p_x as usize, p_y as usize, color).unwrap();
        }
    }
}