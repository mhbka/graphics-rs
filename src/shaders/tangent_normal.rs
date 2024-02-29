use crate::{rasterizer::bary_to_point, ColorSpace, Model, ObjFace, Shader, Transform};
use glam::*;

// Shading using normal-mapped Darboux frame tga + specular lighting
pub struct TangentNormalShader<T: ColorSpace + Copy> {
    varying_texture_coords: [Vec3; 3],
    varying_normals: [Vec3; 3], // need to interpolate normal for darboux transform computation
    varying_ndc: [Vec3; 3],
    uniform_model: Model<T>,
    uniform_transform: Transform,
    uniform_light_dir: Vec3 // need to store here since i'm using this in FS instead of VS
}

impl<T: ColorSpace + Copy> TangentNormalShader<T> {
    pub fn new(model: Model<T>, transform: Transform) -> Self {
        TangentNormalShader {
            varying_texture_coords: [Vec3::new(0.0, 0.0, 0.0); 3],
            varying_normals: [Vec3::new(0.0, 0.0, 0.0); 3],
            varying_ndc: [Vec3::new(0.0, 0.0, 0.0); 3],
            uniform_model: model,
            uniform_transform: transform,
            uniform_light_dir: Vec3::ZERO 
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for TangentNormalShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        self.uniform_light_dir = light_dir; 
        self.varying_texture_coords = obj_face.texture_vertices;
        for i in 0..3 {
            self.varying_texture_coords[i] = self.uniform_model
                .texture_pixel_coords(obj_face.texture_vertices[i].x, obj_face.texture_vertices[i].y)
                .extend(0.0);
            self.varying_normals[i] = self.uniform_transform
                .ndc_inv_tr_transform(obj_face.normals[i]);
            self.varying_ndc[i] = self.uniform_transform
                .ndc_transform(obj_face.vertices[i]);
            };
        self.varying_ndc
    } 

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        // compute actual coords for corresponding pixel in texture + specular + normal images 
        let interpolated_coords = bary_to_point(&bary_coords, &self.varying_texture_coords);

        // get tangent normal of corresponding pixel, compute tangent basis, then convert to global coordinatess
        let normal = {
            // get tangent normal
            let tangent_normal = self.uniform_model.get_tangent_normal(interpolated_coords.x as usize, interpolated_coords.y as usize);

            // get interpolated normal 
            let interpolated_normal = bary_to_point(&bary_coords, &self.varying_normals).normalize();

            // compute darboux transform + 2 vectors that form tangent basis
            let darboux_transform = Mat3::from_cols(
                self.varying_ndc[1] - self.varying_ndc[0],
                self.varying_ndc[2] - self.varying_ndc[0],
                interpolated_normal
            );
            
            let basis_1 = darboux_transform.inverse() * Vec3::new(
                self.varying_texture_coords[0].y - self.varying_texture_coords[0].x,
                self.varying_texture_coords[0].z - self.varying_texture_coords[0].x,
                0.0 
            );
            let basis_2 = darboux_transform.inverse() * Vec3::new(
                self.varying_texture_coords[1].y - self.varying_texture_coords[1].x,
                self.varying_texture_coords[1].z - self.varying_texture_coords[1].x,
                0.0 
            );

            // change basis from tangent basis to global coordinates
            let tangent_transform = Mat3::from_cols(basis_1.normalize(), basis_2.normalize(), tangent_normal).transpose();
            let n = tangent_transform * tangent_normal;

            self.uniform_transform
                .ndc_inv_tr_transform(n)
                .normalize()
        };
        
        // get transformed light vec
        let light = self.uniform_transform
            .ndc_transform(self.uniform_light_dir)
            .normalize();

        // diffuse light - normal lighting
        let intensity = normal.dot(light).max(0.0);

        *color = self.uniform_model.get_texture_color(interpolated_coords.x as usize, interpolated_coords.y as usize);
        color.shade(intensity);
        false
    }
}