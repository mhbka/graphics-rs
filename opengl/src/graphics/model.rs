use std::rc::Rc;
use glam::*;
use super::{model_mesh::ModelMesh, shader::Shader, vertex::Vertex};
use russimp::{
    mesh::Mesh, node::Node, scene::{PostProcess, Scene}
};

pub struct Model {
    meshes: Vec<ModelMesh>,
}

impl Model {
    pub unsafe  fn new(filepath: &str) -> Self {
        let mut model = Model { meshes: Vec::new() };
        model.load_model(filepath);
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
    unsafe fn load_model(&mut self, filepath: &str) {
        let scene = Scene::from_file(filepath,
        vec![
            PostProcess::Triangulate,
            PostProcess::FlipUVs,
            PostProcess::GenerateNormals,
            PostProcess::SplitLargeMeshes,
            PostProcess::OptimizeMeshes
            ])
             .unwrap();
        
        if let Some(root) = scene.root.as_ref() {
            Model::process_node(self, &scene, root);
        }
    }

    /// Recursively processes a node's meshes + its child nodes.
    unsafe fn process_node(&mut self, scene: &Scene, node: &Rc<Node>) {
        for mesh_id in &node.meshes {
            let mesh = &scene.meshes[*mesh_id as usize];
            let model_mesh = Model::process_mesh(mesh, scene);
            self.meshes.push(model_mesh);
        }

        for child_node in &*node.children.borrow() { // <- that's abit fucked up
            Model::process_node(self, scene, &child_node);
        }
    }

    /// Converts a russimp Mesh into our ModelMesh.
    unsafe fn process_mesh(mesh: &Mesh, scene: &Scene) -> ModelMesh {
        let mut vertices = Vec::with_capacity(mesh.vertices.len());
        for i in 0..mesh.vertices.len() {
            vertices.push(
                {
                    let pos_r = mesh.vertices[i];
                    let norm_r = mesh.normals[i];
                    let tex_r = mesh.texture_coords[i].unwrap()[0]; // assumes there's always a texture, also we are only using 1st texture coord

                    let position = Vec3::new(pos_r.x, pos_r.y, pos_r.z);
                    let normal = Vec3::new(norm_r.x, norm_r.y, norm_r.z);
                    let texture_coords = Vec2::new(tex_r.x, tex_r.y); // assumes that z-coord is not used (at least for now)

                    Vertex { position, normal, texture_coords }
                }
            )
        }

        let textures =  { 
            let material = scene.materials[mesh.material_index as usize];
            

        };

        let indices: Vec<u32> = mesh.faces.iter().flat_map(
            |f| f.0 // should be ok since we used PostProcess::Triangulate
            )
            .collect();

        unsafe { ModelMesh::new(vertices, textures, indices) }
    }
}