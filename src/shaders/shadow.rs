use crate::{rasterizer::bary_to_point, ColorSpace, Model, ObjFace, Shader, Transform};
use glam::*;

/// Shadow shader is composed of 2 shaders: depth shader + actual shader.
/// Depth shader places camera at light source and captures visibility information from there.
/// Actual shader uses this information to place shadows.
pub struct DepthShader<T: ColorSpace + Copy> {
    varying_tri: [Vec3; 3],
    uniform_model: Model<T>,
    uniform_transform: Transform
}

impl <T: ColorSpace + Copy> DepthShader<T> {
    pub fn new(model: Model<T>, transform: Transform) -> Self {
        DepthShader {
            varying_tri: [Vec3::ZERO; 3],
            uniform_model: model,
            uniform_transform: transform
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for DepthShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        for i in 0..3 {
            self.varying_tri[i] = 
                self.uniform_transform
                .ndc_transform(obj_face.vertices[i]);
        }
        self.varying_tri
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        let coords = bary_to_point(&bary_coords, &self.varying_tri);
        color.shade(coords[])

        false
    }
}