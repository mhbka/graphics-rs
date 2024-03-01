use crate::{ColorSpace, Shader};

pub struct DepthShader<T: ColorSpace + Copy> {

}

impl <T: ColorSpace + Copy> DepthShader<T> {

}

impl<T: ColorSpace + Copy> Shader<T> for DepthShader<T> {
    fn vertex(&mut self, obj_face: crate::ObjFace, light_dir: glam::Vec3) -> [glam::Vec3; 3] {
        
    }

    fn fragment(&self, bary_coords: glam::Vec3, color: &mut T) -> bool {
        
    }
}