use crate::{ColorSpace, ObjFace};
use glam::*;

pub trait Shader<T: ColorSpace + Copy> {
    // transforms coordinates + prepares data for fragment shader
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3];
    // modify color of pixel + return whether or not to discard it
    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool;
}