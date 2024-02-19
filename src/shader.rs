use std::f32::
use glam::*;
use crate::tgaimage::*;

pub trait Shader<T: ColorSpace + Copy> {
    // transforms coordinates + prepares data for fragment shader
    fn vertex(&mut self, face: [Vec3; 3], normals: [Vec3; 3], light_dir: Vec3, transform: Affine3A) -> [Vec3; 3];

    // determine color of pixel + whether or not to render it
    fn fragment(&self, bar: Vec3, color: T) -> bool;
}

pub struct GouraudShader {
    varying_intensity: [f32; 3]
}

impl<T: ColorSpace + Copy> Shader<T> for GouraudShader {
    fn vertex(&mut self, face: [Vec3; 3], normals: [Vec3; 3], light_dir: Vec3, transform: Affine3A) -> [Vec3; 3] {
        let mut tr_face = Vec::new();
        for i in 0..3 {
            self.varying_intensity[i] = (0.0).max(normals[i].dot(light_dir));
            tr_face.push(transform.transform_point3(face[i]));
        }
        tr_face
    }

    fn fragment(&self, bary_coords: Vec3, &mut color: T) -> bool {
        let intensity = Vec3::from_array(self.varying_intensity) * bary_coords;
        color.shade(intensity);
        false
    }
}