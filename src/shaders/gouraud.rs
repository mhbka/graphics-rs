use crate::{rasterizer::bary_to_point, ColorSpace, Model, ObjFace, Transform};
use glam::*;
use super::shader::Shader;

// Gouraud shading with texture
pub struct GouraudShader<T: ColorSpace + Copy> {
    varying_intensity: [f32; 3],
    varying_texture_coords: [Vec3; 3],
    uniform_model: Model<T>,
    uniform_transform: Transform
}

impl<T: ColorSpace + Copy> GouraudShader<T> {
    pub fn new(model: Model<T>, transform: Transform) -> Self {
        GouraudShader {
            varying_intensity: [0.0, 0.0, 0.0],
            varying_texture_coords: [Vec3::new(0.0, 0.0, 0.0); 3],
            uniform_model: model,
            uniform_transform: transform
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for GouraudShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        let mut transformed_face = obj_face.vertices.clone();
        for i in 0..3 {
            let normal = self.uniform_transform
                .ndc_inv_tr_transform(obj_face.normals[i])
                .normalize();
            self.varying_texture_coords[i] = self.uniform_model
                .texture_pixel_coords(obj_face.texture_vertices[i].x, obj_face.texture_vertices[i].y)
                .extend(0.0);
            transformed_face[i] = self.uniform_transform
                .ndc_transform(obj_face.vertices[i]);
            self.varying_intensity[i] = f32::max(0.0, normal.dot(light_dir));
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        *color = {
            let interpolated_coords = bary_to_point(&bary_coords, &self.varying_texture_coords);
            self.uniform_model.get_texture_color(interpolated_coords.x as usize, interpolated_coords.y as usize)
        }; 
        let intensity = Vec3::from_array(self.varying_intensity).dot(bary_coords);
        color.shade(intensity);
        false
    }
}