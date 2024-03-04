use crate::{ColorSpace, ObjFace};
use glam::*;

pub trait Shader<T: ColorSpace + Copy> {

    /// Transforms raw coordinates into normalized device coordinates + prepares necessary data for fragment shader
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3];

    /// Modifies color of a pixel at the barymetric coordinates + returns whether to render it
    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool;
}