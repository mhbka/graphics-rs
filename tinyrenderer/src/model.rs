use glam::{Vec2, Vec3};

use crate::{ColorSpace, Grayscale, Image, RGB};

/// Represents a model.
/// A model contains ObjFaces (vertices, texture coordinates, normals).
/// It also contains texture, normal, tangent normal, and specular images.
/// There are also helper functions.
#[derive(Clone)]
pub struct Model<T: ColorSpace + Copy> {
    pub texture_image: Image<T>,
    pub normal_image: Image<RGB>,
    pub tangent_normal_image: Image<RGB>,
    pub specular_image: Image<Grayscale>
}

// Converting [0, 1] coordinates into the relevant image's pixel coordinates.
impl <T: ColorSpace + Copy> Model<T> {
    pub fn texture_pixel_coords(&self, x: f32, y: f32) -> Vec2 {
        if x>1.0 || x<0.0 || y>1.0 || y<0.0 {
            panic!("x or y ({x}, {y}) is out of bounds ([0, 1])");
        }
        let pixel_x = (x * self.texture_image.width as f32).floor();
        let pixel_y = (self.texture_image.height as f32 - (y * self.texture_image.height as f32)).floor(); //strangely, need to vflip
        Vec2::new(pixel_x, pixel_y)
    }

    pub fn normal_pixel_coords(&self, x: f32, y: f32) -> Vec2 {
        if x>1.0 || x<0.0 || y>1.0 || y<0.0 {
            panic!("x or y ({x}, {y}) is out of bounds ([0, 1])");
        }
        let pixel_x = (x * self.normal_image.width as f32).floor();
        let pixel_y = (self.texture_image.height as f32 - (y * self.texture_image.height as f32)).floor();
        Vec2::new(pixel_x, pixel_y)
    }

    pub fn tang_normal_pixel_coords(&self, x: f32, y: f32) -> Vec2 {
        if x>1.0 || x<0.0 || y>1.0 || y<0.0 {
            panic!("x or y ({x}, {y}) is out of bounds ([0, 1])");
        }
        let pixel_x = (x * self.tangent_normal_image.width as f32).floor();
        let pixel_y = (self.tangent_normal_image.height as f32 - (y * self.tangent_normal_image.height as f32)).floor();
        Vec2::new(pixel_x, pixel_y)
    }

    pub fn specular_pixel_coords(&self, x: f32, y: f32) -> Vec2 {
        if x>1.0 || x<0.0 || y>1.0 || y<0.0 {
            panic!("x or y ({x}, {y}) is out of bounds ([0, 1])");
        }
        let pixel_x = (x * self.specular_image.width as f32).floor();
        let pixel_y = (self.texture_image.height as f32 - (y * self.texture_image.height as f32)).floor();
        Vec2::new(pixel_x, pixel_y)
    }
}

// Obtaining useful information from model images.
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
            .i as f32
    }
}