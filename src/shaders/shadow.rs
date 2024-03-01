use crate::{rasterizer::bary_to_point, ColorSpace, Model, ObjFace, Shader, Transform};
use glam::*;

/// Shadow shader is composed of 2 shaders: depth shader + actual shader.
/// Depth shader places camera at light source and captures visibility information from there.
/// Actual shader is a standard shader, but also uses visibility information for shadows.


// Calculates visibility information by placing camera at light source.
pub struct DepthShader {
    varying_tri: [Vec3; 3],
    uniform_light_source: Vec3,
    uniform_depth_transform: Transform //note: model_view must be from light_dir perspective
}

impl DepthShader {
    pub fn new(light_source: Vec3, depth_transform: Transform) -> Self {
        DepthShader {
            varying_tri: [Vec3::ZERO; 3],
            uniform_light_source: light_source,
            uniform_depth_transform: depth_transform
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for DepthShader {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        self.uniform_light_source = light_dir;
        for i in 0..3 { 
            self.varying_tri[i] = self.uniform_depth_transform
                .ndc_transform(obj_face.vertices[i]);
        }
        self.varying_tri
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        let screen_coords = self.uniform_depth_transform
            .viewport_transform(
                bary_to_point(&bary_coords, &self.varying_tri)
        );
        let depth = Vec3::distance(self.uniform_light_source, screen_coords);
        *color = T::white();
        color.shade(screen_coords.z/depth);
        false
    }
}


// Actual shader
/* 
pub struct ShadowShader<T: ColorSpace + Copy> {

}

impl <T: ColorSpace + Copy> ShadowShader<T> {

}

impl<T: ColorSpace + Copy> Shader<T> for ShadowShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        
    }
}
*/
