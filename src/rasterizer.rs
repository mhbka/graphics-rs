use crate::tgaimage::*;
use crate::shader::Shader;
use glam::*;


// Triangle rasterization function with depth buffer + texture + perspective etc
pub fn triangle<T, S>(image: &mut Image<T>, texture_img: &mut Image<T>, screen_coords: [Vec3; 3], shader: &S, zbuffer: &mut Vec<f32>)   
where 
    T: ColorSpace + Copy + std::fmt::Debug,
    S: Shader<T> {
    // instantiate transform matrices
    let eye = Vec3::new(-1.0, -1.0, 3.0);
    let centre = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let model_view = lookat(eye, centre, up);
    let projection = Affine3A::IDENTITY;
    let viewport = viewport(image.width/8, image.height/8, image.width*3/4, image.height*3/4);

    /*
    model - transform raw obj coords into world coords
    view - transform world coords into camera perspective
    projection - transform into perspective
    viewport - transform into screen (pixel) coordinates
      */
    let face = face.map(|v| {
        viewport.transform_point3(
            projection.transform_point3(
                model_view.transform_point3(v)
            )
        )
    });

    // scale texture image too (idk why but this one doesn't use transform + scaling like above)
    // also requires VFLIPPING ? wat dafuq
    let texture_face_2d = texture_face.map(|v|{
        Vec2::new(
            v.x * texture_image.width as f32,
            texture_image.height as f32 - v.y * texture_image.height as f32
        )
        .floor()   
    });

    // shrink bounding box to rasterize over
    let mut bboxmin = Vec2::new(image.width as f32 - 1.0, image.height as f32 - 1.0);
    let mut bboxmax = Vec2::new(0.0, 0.0);
    let clamp = Vec2::new(image.width as f32 - 1.0, image.height as f32 - 1.0);
    
    for vertex in &face {
        bboxmin.x = f32::max(0.0, f32::min(bboxmin.x, vertex.x)) as f32;
        bboxmin.y = f32::max(0.0, f32::min(bboxmin.y, vertex.y)) as f32;
    
        bboxmax.x = f32::min(clamp.x, f32::max(bboxmax.x, vertex.x)) as f32;
        bboxmax.y = f32::min(clamp.y, f32::max(bboxmax.y, vertex.y)) as f32;
    } 

    // loop over bounding box pixels for valid baryometric + depth buffer check
    for p_x in bboxmin.x as i32 .. bboxmax.x as i32 + 1 {
        for p_y in bboxmin.y as i32 .. bboxmax.y as i32 + 1 {
            let bc_screen = barycentric(&face, &Vec3::new(p_x as f32, p_y as f32, 0.0));
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

                // use them for interpolating intensity, then shade the color that we have
                let intensity = bary_to_scalar(&bc_screen, &intensities);
                //texture_color.shade(intensity);

                // update zbuffer, then set actual pixel with texture pixel's color
                zbuffer[(p_x + p_y*image.width as i32) as usize] = p_z;
                image.set(p_x as usize, p_y as usize, texture_color).unwrap();
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

// Convert barycentric coords into a 2D point
fn bary_to_point(bc_coords: &Vec3, vertices: &[Vec2; 3]) -> Vec2 {
    Vec2::new(
        bc_coords.x*vertices[0].x + bc_coords.y*vertices[1].x + bc_coords.z*vertices[2].x,
        bc_coords.x*vertices[0].y + bc_coords.y*vertices[1].y + bc_coords.z*vertices[2].y
    ).floor()
}

// Convert barycentric coords into a single scalar
fn bary_to_scalar(bc_coords: &Vec3, weights: &[f32; 3]) -> f32 {
    bc_coords.x*weights[0] + bc_coords.y*weights[1] + bc_coords.z*weights[2]
}