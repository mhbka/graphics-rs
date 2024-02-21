use glam::*;
use crate::{tgaimage::*, transform, ObjFace};
use crate::rasterizer::bary_to_point;

pub trait Shader<T: ColorSpace + Copy> {
    // transforms coordinates + prepares data for fragment shader
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3];
    // modify color of pixel + return whether or not to discard it
    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool;
}


// Gouraud shading without texture
pub struct GouraudShader {
    varying_intensity: [f32; 3],
    uniform_transform: Affine3A
}

impl GouraudShader {
    pub fn new(transform: Affine3A) -> Self {
        GouraudShader { 
            varying_intensity: [0.0, 0.0, 0.0],
            uniform_transform: transform
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for GouraudShader {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        let mut transformed_face = obj_face.vertices.clone();
        let normals = obj_face.normals;
        for i in 0..3 {
            self.varying_intensity[i] = f32::max(0.0, normals[i].dot(light_dir));
            transformed_face[i] = self.uniform_transform.transform_point3(obj_face.vertices[i]);
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
    uniform_texture: Image<T>,
    uniform_transform: Affine3A
}

impl<T: ColorSpace + Copy> GouraudTextureShader<T> {
    pub fn new(texture: Image<T>, transform: Affine3A) -> Self {
        GouraudTextureShader {
            varying_intensity: [0.0, 0.0, 0.0],
            varying_texture_coords: [Vec3::new(0.0, 0.0, 0.0); 3],
            uniform_texture: texture,
            uniform_transform: transform
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for GouraudTextureShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        let mut transformed_face = obj_face.vertices.clone();
        let normals = obj_face.normals;
        for i in 0..3 {
            self.varying_intensity[i] = f32::max(0.0, normals[i].dot(light_dir));
            self.varying_texture_coords[i] = obj_face.texture_vertices[i];
            transformed_face[i] = self.uniform_transform.transform_point3(obj_face.vertices[i]);
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        *color = {
            let interpolated_coords = bary_to_point(&bary_coords, &self.varying_texture_coords);
            self.uniform_texture.get(interpolated_coords.x as usize, interpolated_coords.y as usize).unwrap()
        };
        let intensity = Vec3::from_array(self.varying_intensity).dot(bary_coords);
        color.shade(intensity);
        false
    }
}


// Shading with normal vector recalculation
pub struct NormalCalcShader<T: ColorSpace + Copy> {
    varying_texture_coords: [Vec3; 3],
    uniform_texture: Image<T>,
    uniform_normal: Image<RGB>, // image containing xyz -> rgb mapping for normal vector of each pixel
    uniform_transform: Affine3A,
    uniform_transform_invt: Affine3A,
    uniform_light_dir: Vec3
}

impl<T: ColorSpace + Copy> NormalCalcShader<T> {
    pub fn new(texture_img: Image<T>, normal_img: Image<RGB>, transform: Affine3A) -> Self {
        NormalCalcShader {
            varying_texture_coords: [Vec3::new(0.0, 0.0, 0.0); 3],
            uniform_texture: texture_img,
            uniform_normal: normal_img,
            uniform_transform: transform,
            uniform_transform_invt: Affine3A::from_mat4(Mat4::from(transform.clone()).inverse().transpose()),
            uniform_light_dir: Vec3::ZERO // need to store here since i'm using this in fragment fn, instead of vertex fn
        }
    }
}


impl<T: ColorSpace + Copy> Shader<T> for NormalCalcShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        self.uniform_light_dir = light_dir; 
        let mut transformed_face = obj_face.vertices.clone();
        for i in 0..3 {
            self.varying_texture_coords[i] = obj_face.texture_vertices[i];
            transformed_face[i] = self.uniform_transform.transform_point3(obj_face.vertices[i]);
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        let interpolated_coords = bary_to_point(&bary_coords, &self.varying_texture_coords);
        *color = self.uniform_texture.get(interpolated_coords.x as usize, interpolated_coords.y as usize).unwrap();
        let normal = {
            let normal_color = self.uniform_normal.get(interpolated_coords.x as usize, interpolated_coords.y as usize).unwrap();
            let untransformed_normal = Vec3::new( //map rgb -> xyz
                2.0 * (normal_color.r as f32 / 255.0) - 1.0,
                2.0 * (normal_color.g as f32 / 255.0) - 1.0,
                2.0 * (normal_color.b as f32 / 255.0) - 1.0
            );
            //println!("{:?}", untransformed_normal);
            self.uniform_transform_invt.transform_point3(untransformed_normal).normalize()
        };
        let light = self.uniform_transform.transform_point3(self.uniform_light_dir).normalize();
        let intensity = normal.dot(light);
        color.shade(intensity);
        false
    }
}
