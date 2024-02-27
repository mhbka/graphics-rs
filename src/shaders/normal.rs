use crate::{rasterizer::bary_to_point, ColorSpace, Model, ObjFace, Transform};
use glam::*;
use super::shader::Shader;

// Shading using normal-mapped tga
pub struct NormalMappedShader<T: ColorSpace + Copy> {
    varying_texture_coords: [Vec3; 3],
    uniform_model: Model<T>,
    uniform_transform: Transform,
    uniform_light_dir: Vec3
}

impl<'a, T: ColorSpace + Copy> NormalMappedShader<T> {
    pub fn new(model: Model<T>, transform: Transform) -> Self {
        NormalMappedShader {
            varying_texture_coords: [Vec3::new(0.0, 0.0, 0.0); 3],
            uniform_model: model,
            uniform_transform: transform,
            uniform_light_dir: Vec3::ZERO // need to store here since i'm using this in fragment fn, instead of vertex fn
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for NormalMappedShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        self.uniform_light_dir = light_dir;
        let mut transformed_face = obj_face.vertices.clone();
        for i in 0..3 {
            self.varying_texture_coords[i] = obj_face.texture_vertices[i];
            transformed_face[i] = self.uniform_transform.
                                        get_whole_transform()
                                        .transform_point3(obj_face.vertices[i]);
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        // set M and MIT
        let transform = self.uniform_transform.projection * self.uniform_transform.model_view;
        let transform_inv_tr = Mat4::from(transform).inverse().transpose();

        // compute actual pixel coordinate for texture + normal image
        let interpolated_coords = bary_to_point(&bary_coords, &self.varying_texture_coords);
        
        // get the normal vec at this pixel
        let normal = {
            let untransformed_normal = self.uniform_model.get_normal(interpolated_coords.x as usize, interpolated_coords.y as usize);
            transform_inv_tr
                .transform_point3(untransformed_normal)
                .normalize()
        };

        // transform light vector and get intensity here
        let light = transform.transform_point3(self.uniform_light_dir);
        let intensity = normal.dot(light);

        // shade the color
        //*color = self.uniform_texture.get(interpolated_coords.x as usize, interpolated_coords.y as usize).unwrap();
        color.shade(intensity);
        false
    }
}


// Shading using normal-mapped tga + specular lighting
pub struct NormalSpecularShader<T: ColorSpace + Copy> {
    varying_texture_coords: [Vec3; 3],
    uniform_model: Model<T>,
    uniform_transform: Transform,
    uniform_light_dir: Vec3
}

impl<T: ColorSpace + Copy> NormalSpecularShader<T> {
    pub fn new(model: Model<T>, transform: Transform) -> Self {
        NormalSpecularShader {
            varying_texture_coords: [Vec3::new(0.0, 0.0, 0.0); 3],
            uniform_model: model,
            uniform_transform: transform,
            uniform_light_dir: Vec3::ZERO // need to store here since i'm using this in fragment fn, instead of vertex fn
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for NormalSpecularShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        self.uniform_light_dir = light_dir; 
        let mut transformed_face = obj_face.vertices.clone();
        for i in 0..3 {
            self.varying_texture_coords[i] = obj_face.texture_vertices[i];
            transformed_face[i] = self.uniform_transform
                                    .get_whole_transform()
                                    .transform_point3(obj_face.vertices[i]);
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        // set M and MIT
        let (projection, model_view) = (self.uniform_transform.projection, self.uniform_transform.model_view);
        let transform = projection * model_view;
        let transform_inv_tr = Mat4::from(transform).inverse().transpose();

        // compute actual coords for corresponding pixel in texture + specular + normal images 
        let interpolated_coords = bary_to_point(&bary_coords, &self.varying_texture_coords);

        // get normal of corresponding pixel
        let normal = transform_inv_tr.transform_point3( 
                self.uniform_model
                .get_normal(interpolated_coords.x as usize, interpolated_coords.y as usize)
            )
            .normalize();
        
        // get transformed light vec
        let light = transform.transform_point3(self.uniform_light_dir).normalize();

        // diffuse light - normal lighting
        let diffuse_light = normal.dot(light).max(0.0);

        // specular light - "highlight" from reflection of light
        let reflection = (normal * (2.0 * normal.dot(light)) - light).normalize();
        let specular_light = {
            let specularity = self.uniform_model.get_specularity(interpolated_coords.x as usize, interpolated_coords.y as usize);
            (reflection.z.max(0.0)).powf(specularity) // extremely bright at centre, then quickly disappears outwards
        };      
        
        // use weighted ambient + diffuse + specular light to modify each color
        let ambient_w = 5.0;
        let diffuse_w = 1.0;
        let spec_w = 0.6;
        *color = self.uniform_model.get_texture_color(interpolated_coords.x as usize, interpolated_coords.y as usize);
        let mut color_vec = color.to_vec();
        for c in &mut color_vec {
            *c = f32::min(ambient_w + (*c as f32)*(diffuse_w*diffuse_light + spec_w*specular_light), 255.0) as u8;
        }
        color.from_vec(color_vec).unwrap();
        false
    }
}


