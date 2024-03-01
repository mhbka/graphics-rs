use crate::{rasterizer::bary_to_point, ColorSpace, Model, ObjFace, Shader, Transform};
use glam::*;

// Shading using normal-mapped Darboux frame tga + specular lighting
pub struct TangentNormalShader<T: ColorSpace + Copy> {
    varying_texture_coords: [Vec3; 3],
    varying_normals: [Vec3; 3], // need to interpolate normal for darboux transform computation
    varying_ndc: [Vec3; 3],
    varying_uv: [Vec3; 3],
    uniform_model: Model<T>,
    uniform_transform: Transform,
    uniform_light_dir: Vec3 // need to store here since i'm using this in FS instead of VS
}

impl<T: ColorSpace + Copy> TangentNormalShader<T> {
    pub fn new(model: Model<T>, transform: Transform) -> Self {
        TangentNormalShader {
            varying_texture_coords: [Vec3::ZERO; 3],
            varying_normals: [Vec3::ZERO; 3],
            varying_ndc: [Vec3::ZERO; 3],
            varying_uv: [Vec3::ZERO; 3],
            uniform_model: model,
            uniform_transform: transform,
            uniform_light_dir: Vec3::ZERO 
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for TangentNormalShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        self.uniform_light_dir = light_dir;
        self.varying_uv = obj_face.texture_vertices;
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
        let interpolated_coords = bary_to_point(&bary_coords, &self.varying_uv);

        // get tangent normal of corresponding pixel, compute tangent basis, then convert to actual normal
        let normal = {
            // get raw tangent normal
            let tang_norm_coords = self.uniform_model.tang_normal_pixel_coords(interpolated_coords.x, interpolated_coords.y);
            let tangent_normal = self.uniform_model.get_tangent_normal(tang_norm_coords.x as usize, tang_norm_coords.y as usize).normalize();

            // get interpolated normal 
            let interpolated_normal = bary_to_point(&bary_coords, &self.varying_normals).normalize();

            // compute darboux transform + 2 vectors that form tangent basis
            /**/
            let darboux_transform = Mat3::from_cols(
                self.varying_ndc[1] - self.varying_ndc[0],
                self.varying_ndc[2] - self.varying_ndc[0],
                interpolated_normal
            ).transpose();
                        

            /* 
            let ndc_tri = Mat3::from_cols(self.varying_ndc[0], self.varying_ndc[1], self.varying_ndc[2]);
            let darboux_transform = Mat3::from_cols(
                ndc_tri.row(1) - ndc_tri.row(0),
                ndc_tri.row(2) - ndc_tri.row(0),
                interpolated_normal
            ).transpose();*/
            
             
            /* */
            let basis_i = darboux_transform.inverse() * Vec3::new(
                self.varying_uv[0].y - self.varying_uv[0].x,
                self.varying_uv[0].z - self.varying_uv[0].x,
                0.0 
            );
            let basis_j = darboux_transform.inverse() * Vec3::new(
                self.varying_uv[1].y - self.varying_uv[1].x,
                self.varying_uv[1].z - self.varying_uv[1].x,
                0.0 
            );
            

            /* 
            let basis_i = darboux_transform.inverse() * Vec3::new(
                self.varying_uv[1].x - self.varying_uv[0].x,
                self.varying_uv[2].x - self.varying_uv[0].x,
                0.0 
            );
            let basis_j = darboux_transform.inverse() * Vec3::new(
                self.varying_uv[1].y - self.varying_uv[0].y,
                self.varying_uv[2].y - self.varying_uv[0].y,
                0.0 
            );
            */
            

            // change basis from tangent basis to global coordinates
            let tangent_transform = Mat3::from_cols(
                basis_i.normalize(), 
                basis_j.normalize(), 
                interpolated_normal
                );
        
            (tangent_transform * tangent_normal).normalize()
        };
        //println!("{bary_coords:?}   ->   {normal}");
        
        // transform light vector into ndc
        let light = self.uniform_transform
            .ndc_transform(self.uniform_light_dir)
            .normalize();

        // shade the color
        let intensity = f32::max(0.0, normal.dot(light));
        let texture_pixel_coords = self.uniform_model.texture_pixel_coords(interpolated_coords.x, interpolated_coords.y);
        //*color = self.uniform_model.get_texture_color(texture_pixel_coords.x as usize, texture_pixel_coords.y as usize);
        color.shade(intensity);
        false
    }
}