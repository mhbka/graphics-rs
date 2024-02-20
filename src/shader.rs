use glam::*;
use crate::transform::Transform;
use crate::{tgaimage::*, ObjFace};

pub trait Shader<T: ColorSpace + Copy> {
    // transforms coordinates + prepares data for fragment shader
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3, transform: &Transform) -> [Vec3; 3];

    // modify color of pixel + return whether or not to discard it
    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool;
}

// Gouraud shading without texture
pub struct GouraudShader {
    varying_intensity: [f32; 3]
}

impl GouraudShader {
    pub fn new() -> Self {
        GouraudShader { varying_intensity: [0.0, 0.0, 0.0] }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for GouraudShader {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3, transform: &Transform) -> [Vec3; 3] {
        let mut transformed_face = obj_face.vertices.clone();
        let normals = obj_face.normals;
        for i in 0..3 {
            self.varying_intensity[i] = f32::max(0.0, normals[i].dot(light_dir));
            transformed_face[i] = transform.transform_point(obj_face.vertices[i]);
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
    texture: Image<T>
}

impl<T: ColorSpace + Copy> GouraudTextureShader<T> {
    pub fn new(texture: Image<T>) -> Self {
        GouraudTextureShader {
            varying_intensity: [0.0, 0.0, 0.0],
            varying_texture_coords: [Vec3::new(0.0, 0.0, 0.0); 3],
            texture
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for GouraudTextureShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3, transform: &Transform) -> [Vec3; 3] {
        let mut transformed_face = obj_face.vertices.clone();
        let normals = obj_face.normals;
        for i in 0..3 {
            self.varying_intensity[i] = f32::max(0.0, normals[i].dot(light_dir));
            self.varying_texture_coords[i] = obj_face.texture_vertices[i];
            transformed_face[i] = transform.transform_point(obj_face.vertices[i]);
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        *color = {
            let mut interpolated_coords = Vec3::new(0.0, 0.0, 0.0);
            for i in 0..3 { interpolated_coords += self.varying_texture_coords[i] * bary_coords; }
            let (x, y) = (interpolated_coords.x, interpolated_coords.y);
            self.texture.data[(x + y*self.texture.height as f32) as usize].clone()
        };
        let intensity = Vec3::from_array(self.varying_intensity).dot(bary_coords);
        color.shade(intensity);
        false
    }
}