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
        let coords = bary_to_point(&bary_coords, &self.varying_tri);
        *color = T::white();
        color.shade((coords.z+1.0)/2.0); //extrapolate to [0, 1] then shade by z-value
        false
    }
}


// Actual shader (normal + specular mapping), but uses shadowbuffer to locate z-values to shade as shadows
pub struct ShadowShader<T: ColorSpace + Copy> {
    varying_texture_coords: [Vec3; 3],
    varying_screen_coords: [Vec3; 3],
    uniform_model: Model<T>,
    uniform_transform: Transform,
    uniform_shadow_transform: Affine3A, // transforms screen coords of current fragment into shadow screen coords
    uniform_shadowbuffer: Vec<f32>, // buffer from depth shader
    uniform_light_dir: Vec3
}

impl<T: ColorSpace + Copy> ShadowShader<T> {
    pub fn new(model: Model<T>, transform: Transform, shadow_transform: Affine3A, shadowbuffer: Vec<f32>) -> Self {
        ShadowShader {
            varying_texture_coords: [Vec3::ZERO; 3],
            varying_screen_coords: [Vec3::ZERO; 3],
            uniform_model: model,
            uniform_transform: transform,
            uniform_shadow_transform: shadow_transform,
            uniform_shadowbuffer: shadowbuffer,
            uniform_light_dir: Vec3::ZERO
        }
    }
}

impl<T: ColorSpace + Copy> Shader<T> for ShadowShader<T> {
    fn vertex(&mut self, obj_face: ObjFace, light_dir: Vec3) -> [Vec3; 3] {
        self.uniform_light_dir = light_dir; 
        let mut transformed_face = obj_face.vertices.clone();
        for i in 0..3 {
            self.varying_texture_coords[i] = 
                self.uniform_model
                .texture_pixel_coords(obj_face.texture_vertices[i].x, obj_face.texture_vertices[i].y)
                .extend(0.0);
            transformed_face[i] = 
                self.uniform_transform
                .ndc_transform(obj_face.vertices[i]);
            self.varying_screen_coords[i] = 
                self.uniform_transform
                .get_whole_transform()
                .transform_point3(obj_face.vertices[i])
        }
        transformed_face
    }

    fn fragment(&self, bary_coords: Vec3, color: &mut T) -> bool {
        // compute corresponding point in shadow buffer
        let sb_coords = 
            self.uniform_shadow_transform.transform_point3(
                bary_to_point(&bary_coords, &self.varying_screen_coords)
            );
        
        // compute index in shadow buffer (x + y*width)
        // NOTE: using normal_image width is a hack!!! it should be depth_image's width, but that's not accessible currently
        let sb_idx = (sb_coords.x + sb_coords.y*self.uniform_model.normal_image.width as f32) as usize;   

        // if current point z-value is less than shadowbuffer z-value, reduce brightness (ie create shadow)
        let magic_value = 43.34; 
        let shadow = 0.3 + 0.7 * f32::from(self.uniform_shadowbuffer[sb_idx] < (sb_coords.z+magic_value));

        // compute actual coords for corresponding pixel in texture + specular + normal images 
        let interpolated_coords = bary_to_point(&bary_coords, &self.varying_texture_coords);

        // get normal of corresponding pixel
        let untransformed_normal = self.uniform_model.get_normal(interpolated_coords.x as usize, interpolated_coords.y as usize);
        let normal = self.uniform_transform
            .ndc_inv_tr_transform( untransformed_normal)
            .normalize();
        
        // get transformed light vec
        let light = self.uniform_transform.ndc_transform(self.uniform_light_dir).normalize();

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
        let spec_w = 0.0;
        *color = self.uniform_model.get_texture_color(interpolated_coords.x as usize, interpolated_coords.y as usize);
        let mut color_vec = color.to_vec();
        for c in &mut color_vec {
            *c = f32::min(ambient_w + (*c as f32) * shadow * (diffuse_w*diffuse_light + spec_w*specular_light), 255.0) as u8;
        }
        color.from_vec(color_vec).unwrap();
        false
    }
}
