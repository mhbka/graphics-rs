use crate::{rasterizer::bary_to_point, ColorSpace, Model, ObjFace, Transform};
use glam::*;
use super::shader::Shader;

// Shading using normal-mapped tga + specular lighting
pub struct NormalSpecularShader<T: ColorSpace + Copy> {
    varying_texture_coords: [Vec3; 3],
    uniform_model: Model<T>,
    uniform_transform: Transform,
    uniform_light_dir: Vec3
}

impl<T: ColorSpace + Copy> NormalSpecularShader<T> {
    pub fn new(model: Model<T>, transform: Transform) -> Self {
        NormalSpecularShader {
            varying_texture_coords: [Vec3::new(0.0, 0.0, 0.0); 3],
            uniform_model: model,
            uniform_transform: transform,
            uniform_light_dir: Vec3::ZERO // need to store here since i'm using this in fragment fn, instead of vertex fn
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for NormalSpecularShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        self.uniform_light_dir = light_dir; 
        let mut transformed_face = obj_face.vertices.clone();
        for i in 0..3 {
            self.varying_texture_coords[i] = 
                self.uniform_model
                .texture_pixel_coords(obj_face.texture_vertices[i].x, obj_face.texture_vertices[i].y)
                .extend(0.0);
            transformed_face[i] = 
                self.uniform_transform
                .ndc_transform(obj_face.vertices[i]);
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        // compute actual coords for corresponding pixel in texture + specular + normal images 
        let interpolated_coords = bary_to_point(&bary_coords, &self.varying_texture_coords);

        // get normal of corresponding pixel
        let untransformed_normal = self.uniform_model.get_normal(interpolated_coords.x as usize, interpolated_coords.y as usize);
        let normal = self.uniform_transform
                        .ndc_inv_tr_transform( untransformed_normal)
                        .normalize();
        
        // get transformed light vec
        let light = self.uniform_transform.ndc_transform(self.uniform_light_dir).normalize();

        // diffuse light - normal lighting
        let diffuse_light = normal.dot(light).max(0.0);

        // specular light - "highlight" from reflection of light
        let reflection = (normal * (2.0 * normal.dot(light)) - light).normalize();
        let specular_light = {
            let specularity = self.uniform_model.get_specularity(interpolated_coords.x as usize, interpolated_coords.y as usize);
            (reflection.z.max(0.0)).powf(specularity) // extremely bright at centre, then quickly disappears outwards
        };      
        
        // use weighted ambient + diffuse + specular light to modify each color
        let ambient_w = 5.0;
        let diffuse_w = 1.0;
        let spec_w = 0.6;
        *color = self.uniform_model.get_texture_color(interpolated_coords.x as usize, interpolated_coords.y as usize);
        let mut color_vec = color.to_vec();
        for c in &mut color_vec {
            *c = f32::min(ambient_w + (*c as f32)*(diffuse_w*diffuse_light + spec_w*specular_light), 255.0) as u8;
        }
        color.from_vec(color_vec).unwrap();
        false
    }
}
