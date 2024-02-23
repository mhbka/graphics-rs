pub mod shader;
pub mod gouraud;
pub mod normal;

pub use shader::Shader;
pub use gouraud::{GouraudShader, GouraudTextureShader};
pub use normal::{NormalMappedShader, NormalSpecularShader, DarbouxNormalSpecularShader};




