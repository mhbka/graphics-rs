pub mod shader;
pub mod gouraud;
pub mod normal_mapped;

pub use shader::Shader;
pub use gouraud::{GouraudShader, GouraudTextureShader};
pub use normal_mapped::{NormalMappedShader, NormalSpecularShader};




