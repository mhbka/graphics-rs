use crate::tgaimage::*;
use crate::shaders::Shader;
use glam::*;


// Triangle rasterization function with depth buffer + texture + perspective etc
pub fn triangle<T, S>(
    image: &mut Image<T>,
    shader: &S, 
    screen_coords: [Vec3; 3], 
    zbuffer: &mut Vec<f32>)

where 
    T: ColorSpace + Copy + std::fmt::Debug,
    S: Shader<T> {

    // shrink bounding box to rasterize over
    let mut bboxmin = Vec2::new(image.width as f32 - 1.0, image.height as f32 - 1.0);
    let mut bboxmax = Vec2::new(0.0, 0.0);
    let clamp = Vec2::new(image.width as f32 - 1.0, image.height as f32 - 1.0);
    
    for vertex in &screen_coords {
        bboxmin.x = f32::max(0.0, f32::min(bboxmin.x, vertex.x)) as f32;
        bboxmin.y = f32::max(0.0, f32::min(bboxmin.y, vertex.y)) as f32;
    
        bboxmax.x = f32::min(clamp.x, f32::max(bboxmax.x, vertex.x)) as f32;
        bboxmax.y = f32::min(clamp.y, f32::max(bboxmax.y, vertex.y)) as f32;
    } 

    // loop over bounding box pixels for valid baryometric + depth buffer check
    for p_x in bboxmin.x as i32 .. bboxmax.x as i32 + 1 {
        for p_y in bboxmin.y as i32 .. bboxmax.y as i32 + 1 {
            let bc_screen = barycentric(&screen_coords, &Vec3::new(p_x as f32, p_y as f32, 0.0));
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
                }

            // depth buffer check for z-value
            let p_z = screen_coords[0].z * bc_screen.x
                        + screen_coords[1].z * bc_screen.y
                        + screen_coords[2].z * bc_screen.z;

            if p_z > zbuffer[(p_x + p_y*image.width as i32) as usize] {

                // instantiate a color and run it through shader
                let mut texture_color = T::white();
                let discard = shader.fragment(bc_screen, &mut texture_color);

                // if don't discard, update zbuffer + set pixel
                if !discard {
                    zbuffer[(p_x + p_y*image.width as i32) as usize] = p_z;
                    image.set(p_x as usize, p_y as usize, texture_color).unwrap();
                }
            }
        }
    }
}


// Calculate barycentric weights, given 3 vertices and a point
// Pass in Vec3, but we only use x and y
fn barycentric(vertices: &[Vec3; 3], p: &Vec3) -> Vec3 {
    let a = Vec3::new(vertices[2].x - vertices[0].x, vertices[1].x - vertices[0].x, vertices[0].x - p.x);
    let b = Vec3::new(vertices[2].y - vertices[0].y, vertices[1].y - vertices[0].y, vertices[0].y - p.y);
    let u = a.cross(b);    

    // Check for degenerate triangle (ie, cross product result is zero)
    if u.z.abs() < 1.0 {
        return Vec3::new(-1.0, 1.0, 1.0);
    }

    Vec3::new(
        1.0 - (u.x + u.y) / u.z,
        u.y / u.z,
        u.x / u.z,
    )
}

// Convert barycentric coords into a point
pub fn bary_to_point(bc_coords: &Vec3, vertices: &[Vec3; 3]) -> Vec3 {
    Vec3::new(
        bc_coords.x*vertices[0].x + bc_coords.y*vertices[1].x + bc_coords.z*vertices[2].x,
        bc_coords.x*vertices[0].y + bc_coords.y*vertices[1].y + bc_coords.z*vertices[2].y,
        bc_coords.x*vertices[0].z + bc_coords.y*vertices[1].z + bc_coords.z*vertices[2].z,
    )
}