use glam::*;
use crate::transform::Transform;
use crate::tgaimage::*;

pub trait Shader<T: ColorSpace + Copy> {
    // transforms coordinates + prepares data for fragment shader
    fn vertex(&mut self, face: [Vec3; 3], normals: [Vec3; 3], light_dir: Vec3, transform: Transform) -> [Vec3; 3];

    // determine color of pixel + whether or not to render it
    fn fragment(&self, bar: Vec3, color: &mut T) -> bool;
}

pub struct GouraudShader {
    varying_intensity: [f32; 3]
}

impl GouraudShader {
    pub fn new() -> Self {
        GouraudShader { varying_intensity: [0.0, 0.0, 0.0] }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for GouraudShader {

    fn vertex(&mut self, face: [Vec3; 3], normals: [Vec3; 3], light_dir: Vec3, transform: Transform) -> [Vec3; 3] {
        let mut tr_face = face.clone();
        for i in 0..3 {
            self.varying_intensity[i] = f32::max(0.0, normals[i].dot(light_dir));
            tr_face[i] = transform.transform_point(face[i]);
        }
        tr_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        let intensity = Vec3::from_array(self.varying_intensity).dot(bary_coords);
        color.shade(intensity);
        false
    }
}