use crate::types::*;
use crate::tgaimage::*;
use std::cmp::{min, max};

// Cross product function for 2D vectors
fn cross_product_2d(a: &Coord, b: &Coord) -> i32 {
    a.x * b.y - a.y * b.x
}

// Barycentric coordinates calculation
fn barycentric(pts: &[Coord; 3], p: &Coord) -> Vertex {
    let u = Vertex {
        x: cross_product_2d(&Coord { x: pts[2].x - pts[0].x, y: pts[1].x - pts[0].x },
                            &Coord { x: pts[0].x - p.x, y: pts[2].x - pts[0].x }) as f32,
        y: cross_product_2d(&Coord { x: pts[2].y - pts[0].y, y: pts[1].y - pts[0].y },
                            &Coord { x: pts[0].y - p.y, y: pts[2].y - pts[0].y }) as f32,
        z: 0.0, // 2D cross product has zero z-component
    };

    if u.z.abs() < 1.0 {
        return Vertex { x: -1.0, y: 1.0, z: 1.0 }; // Degenerate triangle
    }

    Vertex {
        x: 1.0 - (u.x + u.y) / u.z,
        y: u.y / u.z,
        z: u.x / u.z,
    }
}

// Triangle rasterization function
pub fn triangle<T>(pts: &[Coord; 3], image: &mut Image<T>, color: T) 
where T: ColorSpace + Copy {
    let mut bboxmin = Coord { x: image.width as i32 - 1, y: image.height as i32 - 1 };
    let mut bboxmax = Coord { x: 0, y: 0 };
    let clamp = Coord { x: image.width as i32 - 1, y: image.height as i32 - 1 };

    for i in 0..3 {
        bboxmin.x = max(0, min(bboxmin.x, pts[i].x));
        bboxmin.y = max(0, min(bboxmin.y, pts[i].y));

        bboxmax.x = min(clamp.x, max(bboxmax.x, pts[i].x));
        bboxmax.y = min(clamp.y, max(bboxmax.y, pts[i].y));
    }

    let mut p = Coord { x: bboxmin.x, y: bboxmin.y };

    while p.x <= bboxmax.x {
        while p.y <= bboxmax.y {
            let bc_screen = barycentric(pts, &p);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            image.set(p.x as usize, p.y as usize, color);
            p.y += 1;
        }
        p.x += 1;
        p.y = bboxmin.y;
    }
}