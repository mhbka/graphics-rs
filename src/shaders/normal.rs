use crate::{rasterizer::bary_to_point, ColorSpace, Model, ObjFace, Transform};
use glam::*;
use super::shader::Shader;

// Shading using normal-mapped tga
pub struct NormalMappedShader<T: ColorSpace + Copy> {
    varying_texture_coords: [Vec3; 3],
    uniform_model: Model<T>,
    uniform_transform: Transform,
    uniform_light_dir: Vec3
}

impl<'a, T: ColorSpace + Copy> NormalMappedShader<T> {
    pub fn new(model: Model<T>, transform: Transform) -> Self {
        NormalMappedShader {
            varying_texture_coords: [Vec3::new(0.0, 0.0, 0.0); 3],
            uniform_model: model,
            uniform_transform: transform,
            uniform_light_dir: Vec3::ZERO // need to store here since i'm using this in fragment fn, instead of vertex fn
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for NormalMappedShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        self.uniform_light_dir = light_dir;
        let mut transformed_face = obj_face.vertices.clone();
        for i in 0..3 {
            self.varying_texture_coords[i] = 
                self.uniform_model
                .texture_pixel_coords(obj_face.texture_vertices[i].x, obj_face.texture_vertices[i].y)
                .extend(0.0);
            transformed_face[i] = self.uniform_transform.ndc_transform(obj_face.vertices[i]);
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {

        // compute actual pixel coordinate for texture + normal image
        let interpolated_coords = bary_to_point(&bary_coords, &self.varying_texture_coords);
        
        // get the normal vec at this pixel
        let normal = {
            let untransformed_normal = self.uniform_model
                .get_normal(interpolated_coords.x as usize, interpolated_coords.y as usize);

            self.uniform_transform
                .ndc_inv_tr_transform(untransformed_normal)
                .normalize()
        };

        // transform light vector into ndc
        let light = self.uniform_transform
            .ndc_transform(self.uniform_light_dir)
            .normalize();

        // shade the color
        let intensity = f32::max(0.0, normal.dot(light));
        //*color = self.uniform_model.get_texture_color(interpolated_coords.x as usize, interpolated_coords.y as usize);
        color.shade(intensity);
        false
    }
}




