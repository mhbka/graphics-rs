use image::{io::Reader as ImageReader, ColorType};
use russimp::material::{DataContent, Texture, TextureType};
use gl::types::*;

#[derive(Debug)]
pub enum ModelTextureType {
    DIFFUSE,
    SPECULAR
}

impl ToString for ModelTextureType {
    fn to_string(&self) -> String {
        match *self {
            ModelTextureType::DIFFUSE => "diffuse".to_owned(),
            ModelTextureType::SPECULAR => "specular".to_owned(),
        }
    }
}

/// A graphic texture.
/// 
/// Note: named as such due to conflict with russimp's Texture type.
pub struct ModelTexture {
    pub filename: String,
    pub id: u32,
    pub variant: ModelTextureType
}

// Public fns
impl ModelTexture {
    /// Generate a new ModelTexture from an image file.
    pub unsafe fn new(filename: &str, variant: ModelTextureType) -> Self {
        let mut id = 0;
        gl::GenTextures(1, &mut id as *mut u32);
        gl::BindTexture(gl::TEXTURE_2D, id);

        ModelTexture::set_options();

        let (channels, width, height, flattened_pixels) = ModelTexture::load_image_data(filename);
        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            channels as i32, // ??
            width as i32, 
            height as i32, 
            0, 
            channels,
            gl::UNSIGNED_BYTE,
            flattened_pixels.as_ptr() as *const GLvoid
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        ModelTexture {
            filename: filename.to_owned(), 
            id,
            variant
        }
    }

    /// Transform a russimp Texture to a native ModelTexture.
    pub unsafe fn from_russimp_texture(texture: &Texture, variant: ModelTextureType) -> Self {
        let mut id = 0;
        gl::GenTextures(1, &mut id as *mut u32);
        gl::BindTexture(gl::TEXTURE_2D, id);

        ModelTexture::set_options();

        // TODO: verify that DataContent::Bytes is also 4 bytes per pixel
        let pixel_data_ptr = match &texture.data {
            DataContent::Bytes(bytes) => bytes.as_ptr() as *const GLvoid,
            DataContent::Texel(texels) => texels.as_ptr() as *const GLvoid
        };
        let channels = gl::RGBA; 
        println!("{}", texture.ach_format_hint); // maybe with this?

        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            channels as i32, // ??
            texture.width as i32, 
            texture.height as i32, 
            0, 
            channels,
            gl::UNSIGNED_BYTE,
            pixel_data_ptr
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        ModelTexture {
            filename: texture.filename.clone(), 
            id,
            variant
        }
    }
}

// Internal implementations
impl ModelTexture {
    fn load_image_data(filename: &str) -> (u32, u32, u32, Vec<u8>) {
        let img = ImageReader::open(&format!("assets/textures/{filename}"))
            .expect(&format!("Couldn't load ModelTexture image: {filename}"))
            .decode()
            .expect(&format!("Couldn't decode ModelTexture image: {filename}"))
            .flipv(); // OpenGL expects y=0 to be at the bottom of image, but images usually have y=0 at the top

        let channels = match img.color() {
            ColorType::Rgb8 => gl::RGB,
            ColorType::Rgba8 => gl::RGBA,
            other => panic!("Unsupported ColorType when loading ModelTexture image {filename}: {other:?}"),
        };
        let (width, height) = (img.width(), img.height());
        let flattened_pixels: Vec<u8> = img.into_bytes();

        (channels, width, height, flattened_pixels)
    }

    unsafe fn set_options() {
        // TODO: make this configurable?
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32)
    }
}
