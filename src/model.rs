use glam::Vec3;

use crate::{ColorSpace, Grayscale, Image, ObjFace, RGB};

/// Represents a model.
/// A model contains ObjFaces (vertices, texture coordinates, normals).
/// It also contains texture, normal, tangent normal, and specular images.
/// There are also helper functions.
#[derive(Clone)]
pub struct Model<T: ColorSpace + Copy> {
    pub obj_faces: Vec<ObjFace>,
    pub texture_image: Image<T>,
    pub normal_image: Image<RGB>,
    pub tangent_normal_image: Image<RGB>,
    pub specular_image: Image<Grayscale>
}

impl <T: ColorSpace + Copy> Model<T> {
    pub fn get_texture_color(&self, x: usize, y: usize) -> T {
        self.texture_image.get(x, y).unwrap()
    }

    pub fn get_normal(&self, x: usize, y: usize) -> Vec3 {
        let normal_color = self.normal_image.get(x, y).unwrap();
        Vec3::new(
            2.0 * (normal_color.r as f32 / 255.0) - 1.0,
            2.0 * (normal_color.g as f32 / 255.0) - 1.0,
            2.0 * (normal_color.b as f32 / 255.0) - 1.0,
        )
    }

    pub fn get_tangent_normal(&self, x: usize, y: usize) -> Vec3 {
        let normal_color = self.tangent_normal_image.get(x, y).unwrap();
        Vec3::new(
            2.0 * (normal_color.r as f32 / 255.0) - 1.0,
            2.0 * (normal_color.g as f32 / 255.0) - 1.0,
            2.0 * (normal_color.b as f32 / 255.0) - 1.0,
        )
    }

    pub fn get_specularity(&self, x: usize, y: usize) -> f32 {
        self.specular_image
            .get(x, y)
            .unwrap()
            .i as f32 / 255.0
    }
}