mod shader;
mod gouraud;
mod normal;
mod normal_spec;
mod tangent_normal;
mod shadow;

pub use shader::Shader;
pub use gouraud::GouraudShader;
pub use normal::NormalMappedShader;
pub use normal_spec::NormalSpecularShader;
pub use tangent_normal::TangentNormalShader;
pub use shadow::{DepthShader, ShadowShader};




