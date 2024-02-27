pub mod shader;
pub mod gouraud;
pub mod normal;
pub mod tangent_normal;

pub use shader::Shader;
pub use gouraud::GouraudShader;
pub use normal::{NormalMappedShader, NormalSpecularShader};
pub use tangent_normal::TangentNormalShader;




