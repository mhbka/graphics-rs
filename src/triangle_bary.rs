use crate::tgaimage::*;
use glam::*;


// Calculate barycentric weights, given 3 vertices and a point
fn barycentric(vertices: &[Vec2; 3], p: &Vec2) -> Vec3 {
    let u = {
        let a = Vec3::new(vertices[2].x - vertices[0].x, vertices[1].x - vertices[0].x, vertices[0].x - p.x);
        let b = Vec3::new(vertices[2].y - vertices[0].y, vertices[1].y - vertices[0].y, vertices[0].y - p.y);
        a.cross(b)
    };

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
    Vec2::new(
        bc_vertex.x*vertices[0].x + bc_vertex.y*vertices[1].x + bc_vertex.z*vertices[2].x,
        bc_vertex.x*vertices[0].y + bc_vertex.y*vertices[1].y + bc_vertex.z*vertices[2].y
    )
}


// Triangle rasterization function with depth buffer + texture + perspective etc
pub fn triangle<T>(
    image: &mut Image<T>, 
    texture_image: &mut Image<T>, 
    face: &mut [Vec3; 3], 
    texture_face: [Vec3; 3], 
    zbuffer: &mut Vec<f32>, 
    intensity: f32
) 
where T: ColorSpace + Copy + std::fmt::Debug {

    /* 
    let c = -1.5; // distance from camera

    // transformation; perspective of camera from z=5 (i think)
    let face = face.map(|v| {
        Vec3::new(
            v.x / (1.0 - (v.y/c)),
            v.y / (1.0 - (v.y/c)),
            v.z / (1.0 - (v.y/c))
        )
    }); 
    */

    // scale [0,1] coords into image size
    let face_2d = face.map(|v| {
        Vec2::new(
            (1.0 + v.x)*image.width as f32 / 2.0, 
            (1.0 + v.y)* image.height as f32 / 2.0
        )
    });

    // scale texture image too (idk why but this one doesn't use transform + scaling like above)
    // also requires VFLIPPING ? wat dafuq
    let texture_face_2d = texture_face.map(|v|{
        Vec2::new(
            v.x * texture_image.width as f32,
            texture_image.height as f32 - v.y * texture_image.height as f32
        )   
    });

    // shrink bounding box to rasterize over
    let mut bboxmin = Vec2::new(image.width as f32 - 1.0, image.height as f32 - 1.0);
    let mut bboxmax = Vec2::new(0.0, 0.0);
    let clamp = Vec2::new(image.width as f32 - 1.0, image.height as f32 - 1.0);
    for vertex in &face_2d {
        bboxmin.x = f32::max(0.0, f32::min(bboxmin.x, vertex.x)) as f32;
        bboxmin.y = f32::max(0.0, f32::min(bboxmin.y, vertex.y)) as f32;
    
        bboxmax.x = f32::min(clamp.x, f32::max(bboxmax.x, vertex.x)) as f32;
        bboxmax.y = f32::min(clamp.y, f32::max(bboxmax.y, vertex.y)) as f32;
    } 

    // loop over bounding box pixels for valid baryometric + depth buffer check
    for p_x in bboxmin.x as i32 .. bboxmax.x as i32 +1 {
        for p_y in bboxmin.y as i32 .. bboxmax.y as i32 +1 {
            let bc_screen = barycentric(&face_2d, &Vec2::new(p_x as f32, p_y as f32));
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            // depth buffer check for z-value
            let p_z = face[0].z * bc_screen.x
                        + face[1].z * bc_screen.y
                        + face[2].z * bc_screen.z;

            if p_z > zbuffer[(p_x + p_y*image.width as i32) as usize] {

                // use barycentric coordinates to locate corresponding pixel within texture_face in texture_img
                let texture_pixel_index = {
                    let coord = bary_to_point(&bc_screen, &texture_face_2d);
                    (coord.x + coord.y*texture_image.height as f32) as usize
                };
                let mut texture_color = texture_image.data[texture_pixel_index];
                texture_color.shade(intensity);

                // update zbuffer, then set actual pixel with texture pixel's color
                zbuffer[(p_x + p_y*image.width as i32) as usize] = p_z;
                image.set(p_x as usize, p_y as usize, texture_color).unwrap();
            }
        }
    }
}
