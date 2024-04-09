use glam::*;
use image::io::Reader as ImageReader;
use std::{
    cell::{Ref, RefCell}, collections::HashMap, env, rc::Rc
};
use super::{
    model_mesh::ModelMesh, 
    model_texture::{ModelTexture, ModelTextureType}, 
    shader::Shader, 
    vertex::Vertex
};
use russimp::{
    material::{PropertyTypeInfo, TextureType, Texture, DataContent}, 
    mesh::Mesh, 
    node::Node, 
    scene::{PostProcess, Scene}
};


pub struct Model {
    meshes: Vec<ModelMesh>,
}

// public impls
impl Model {
    pub unsafe fn new(folder_path: &str, file_name: &str) -> Self {
        let mut model = Model { meshes: Vec::new() };
        model.load_model(folder_path, file_name);
        model
    }

    pub unsafe fn draw(&self, shader: &mut Shader) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }
}

// private impls
impl Model {
    /// Loads a model from file using russimp,
    /// and recursively processes its nodes.
    unsafe fn load_model(&mut self, folder_path: &str, file_name: &str) {
        let mut scene = Scene::from_file(&format!("{folder_path}/{file_name}"),
        vec![
            PostProcess::Triangulate,
            PostProcess::FlipUVs,
            PostProcess::GenerateNormals,
            PostProcess::SplitLargeMeshes,
            PostProcess::OptimizeMeshes
            ])
             .unwrap();

        Model::add_nonembedded_textures(&mut scene, folder_path);
        Model::add_shininess_texture(&mut scene, folder_path); // HACK FIX

        let mut model_textures: HashMap<String, Rc<RefCell<ModelTexture>>> = HashMap::new();

        if let Some(root) = scene.root.as_ref() {
            Model::process_node(self, &scene, &mut model_textures, root);
        }
    }

    /// Add non-embedded textures to materials that have them
    /// using image data from specified image files.
    fn add_nonembedded_textures(scene: &mut Scene, folder_path: &str) {
        for mat in &mut scene.materials {
            for property in &mat.properties {
                if property.semantic != TextureType::None && property.key == "$tex.file" { // if is actual TextureType and is a filename,
                    if let None = mat.textures.get(&property.semantic) { // and texture doesn't exist yet, then add it.
                        let file_name = match &property.data {
                            PropertyTypeInfo::String(filename) => filename,
                            _ => panic!("MaterialProperty with key '$tex.file' has non-string data."),
                        };
                        println!("note: loading non-embedded texture ({:?}) from {}", property.semantic, file_name);
                        let texture_path = format!("{folder_path}/{file_name}");

                        // load image and collect pixels into Vec<u8>
                        let tex_img = ImageReader::open(&texture_path)
                            .expect(&format!("Couldn't open texture image at {texture_path:?}"))
                            .decode()
                            .expect(&format!("Couldn't decode texture image at {texture_path:?}"))
                            .flipv()
                            .into_rgba8();
                        let mut tex_pixels = Vec::with_capacity((tex_img.height() * tex_img.width() * 4) as usize);
                        for pixel in tex_img.pixels() {
                            tex_pixels.extend_from_slice(&pixel.0);
                        }

                        // create a new russimp texture and add to textures hashmap
                        let texture = Texture {
                            height: tex_img.height(),
                            width: tex_img.width(),
                            filename: file_name.clone(),
                            ach_format_hint: String::new(), // idk
                            data: DataContent::Bytes(tex_pixels)
                        };
                        mat.textures.insert(property.semantic, Rc::new(RefCell::new(texture)));
                    }
                }
                
            }
        }
    }

    /// Manually adds shininess texture (as a hack fix)
    fn add_shininess_texture(scene: &mut Scene, folder_path: &str) {
        for mat in &mut scene.materials {
            let texture_path = format!("{folder_path}/ao.jpg");

            let tex_img = ImageReader::open(&format!("{folder_path}/ao.jpg"))
            .expect(&format!("Couldn't open texture image at {texture_path:?}"))
            .decode()
            .expect(&format!("Couldn't decode texture image at {texture_path:?}"))
            .flipv()
            .into_rgba8();
    
            let mut tex_pixels = Vec::with_capacity((tex_img.height() * tex_img.width() * 4) as usize);
            for pixel in tex_img.pixels() {
                tex_pixels.extend_from_slice(&pixel.0);
            }
    
            let texture = Texture {
                height: tex_img.height(),
                width: tex_img.width(),
                filename: "ao.jpg".to_owned(),
                ach_format_hint: String::new(), // idk
                data: DataContent::Bytes(tex_pixels)
            };

            mat.textures.insert(TextureType::Shininess, Rc::new(RefCell::new(texture)));
        }
        
    }

    /// Recursively processes a node's meshes + its child nodes.
    unsafe fn process_node(&mut self, scene: &Scene, textures: &mut HashMap<String, Rc<RefCell<ModelTexture>>>, node: &Rc<Node>) {
        for mesh_id in &node.meshes {
            let mesh = &scene.meshes[*mesh_id as usize];
            let model_mesh = Model::process_mesh(mesh, scene, textures);
            self.meshes.push(model_mesh);
        }

        for child_node in &*node.children.borrow() { // <- that's abit fucked up
            Model::process_node(self, scene, textures, &child_node);
        }
    }

    /// Converts a russimp Mesh into our ModelMesh.
    unsafe fn process_mesh(mesh: &Mesh, scene: &Scene, textures: &mut HashMap<String, Rc<RefCell<ModelTexture>>>) -> ModelMesh {
        let mut vertices = Vec::with_capacity(mesh.vertices.len());
        for i in 0..mesh.vertices.len() {
            vertices.push(
                {
                    let pos_r = mesh.vertices[i];
                    let norm_r = mesh.normals[i];
                    let tex_r = mesh.texture_coords[0].as_deref().unwrap()[i]; // assumes it always exists

                    let position = Vec3::new(pos_r.x, pos_r.y, pos_r.z);
                    let normal = Vec3::new(norm_r.x, norm_r.y, norm_r.z);
                    let texture_coords = Vec2::new(tex_r.x, tex_r.y); // assumes that z-coord is not used (at least for now)

                    Vertex { position, normal, texture_coords }
                }
            )
        }

        let textures =  { 
            let material = &scene.materials[mesh.material_index as usize];

            println!("{:?}", material.properties);

            let mut texs = Vec::with_capacity(3);
            let tex_types = [TextureType::Diffuse, TextureType::Specular, TextureType::Shininess];

            for tex_type in tex_types {
                let assimp_tex = material.textures
                .get(&tex_type)
                .unwrap()
                .borrow();

                let tex = match textures.get(&assimp_tex.filename) {
                    Some(tex) => tex.clone(),
                    None => {
                        let tex = ModelTexture::from_russimp_texture(&*assimp_tex, ModelTextureType::from(tex_type));
                        let tex = Rc::new(RefCell::new(tex));
                        textures.insert(assimp_tex.filename.clone(), tex.clone());
                        tex
                    },
                };
                texs.push(tex);
            }
            texs
        };

        let indices: Vec<u32> = mesh.faces.iter().flat_map(
            |f| f.0.clone() // should be ok since we used PostProcess::Triangulate
            )
            .collect();

        println!("note: processed mesh: {}, vertices: {}, indices: {}", mesh.name, mesh.vertices.len(), indices.len());

        unsafe { ModelMesh::new(vertices, textures, indices) }
    }
}