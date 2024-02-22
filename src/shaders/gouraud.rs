use crate::{rasterizer::bary_to_point, ColorSpace, Image, ObjFace, Transform};
use glam::*;
use super::shader::Shader;

// Gouraud shading without texture
pub struct GouraudShader {
    varying_intensity: [f32; 3],
    uniform_transform: Transform
}

impl GouraudShader {
    pub fn new(transform: Transform) -> Self {
        GouraudShader { 
            varying_intensity: [0.0, 0.0, 0.0],
            uniform_transform: transform
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for GouraudShader {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        let mut transformed_face = obj_face.vertices.clone();
        let normals = obj_face.normals;
        for i in 0..3 {
            self.varying_intensity[i] = f32::max(0.0, normals[i].dot(light_dir));
            transformed_face[i] = self.uniform_transform.get_whole_transform().transform_point3(obj_face.vertices[i]);
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        let intensity = Vec3::from_array(self.varying_intensity).dot(bary_coords);
        color.shade(intensity);
        false
    }
}


// Gouraud shading with texture
pub struct GouraudTextureShader<T: ColorSpace + Copy> {
    varying_intensity: [f32; 3],
    varying_texture_coords: [Vec3; 3],
    uniform_texture: Image<T>,
    uniform_transform: Transform
}

impl<T: ColorSpace + Copy> GouraudTextureShader<T> {
    pub fn new(texture: Image<T>, transform: Transform) -> Self {
        GouraudTextureShader {
            varying_intensity: [0.0, 0.0, 0.0],
            varying_texture_coords: [Vec3::new(0.0, 0.0, 0.0); 3],
            uniform_texture: texture,
            uniform_transform: transform
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for GouraudTextureShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        let mut transformed_face = obj_face.vertices.clone();
        let normals = obj_face.normals;
        for i in 0..3 {
            self.varying_intensity[i] = f32::max(0.0, normals[i].dot(light_dir));
            self.varying_texture_coords[i] = obj_face.texture_vertices[i];
            transformed_face[i] = self.uniform_transform.get_whole_transform().transform_point3(obj_face.vertices[i]);
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        *color = {
            let interpolated_coords = bary_to_point(&bary_coords, &self.varying_texture_coords);
            self.uniform_texture.get(interpolated_coords.x as usize, interpolated_coords.y as usize).unwrap()
        };
        let intensity = Vec3::from_array(self.varying_intensity).dot(bary_coords);
        color.shade(intensity);
        false
    }
}