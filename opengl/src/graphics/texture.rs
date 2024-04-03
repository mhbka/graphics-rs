use image::{io::Reader as ImageReader, ColorType};
use gl::types::*;

#[derive(Debug)]
pub enum TextureType {
    DIFFUSE,
    SPECULAR
}

impl ToString for TextureType {
    fn to_string(&self) -> String {
        match *self {
            TextureType::DIFFUSE => "diffuse".to_owned(),
            TextureType::SPECULAR => "specular".to_owned(),
        }
    }
}

/// Wrapper struct for a texture.
pub struct Texture {
    pub filename: String,
    pub id: u32,
    pub variant: TextureType
}

// Public fns
impl Texture {
    pub unsafe fn new(filename: &str, variant: TextureType) -> Self {
        // gen and bind a new texture
        let mut id = 0;
        gl::GenTextures(1, &mut id as *mut u32);
        gl::BindTexture(gl::TEXTURE_2D, id);

        // set options for the texture
        Texture::set_options();

        // load texture image data and copy to currently bound texture
        let (channels, width, height, flattened_pixels) = Texture::load_image_data(filename);
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

        // generate a mipmap for this texture
        gl::GenerateMipmap(gl::TEXTURE_2D);

        Texture {
            filename: filename.to_owned(), 
            id,
            variant
        }
    }
}

// Internal implementations
impl Texture {
    fn load_image_data(filename: &str) -> (u32, u32, u32, Vec<u8>) {
        let img = ImageReader::open(&format!("assets/textures/{filename}"))
            .expect(&format!("Couldn't load texture image: {filename}"))
            .decode()
            .expect(&format!("Couldn't decode texture image: {filename}"))
            .flipv(); // OpenGL expects y=0 to be at the bottom of image, but images usually have y=0 at the top

        let channels = match img.color() {
            ColorType::Rgb8 => gl::RGB,
            ColorType::Rgba8 => gl::RGBA,
            other => panic!("Unsupported ColorType when loading texture image {filename}: {other:?}"),
        };
        let (width, height) = (img.width(), img.height());
        let flattened_pixels: Vec<u8> = img.into_bytes();

        (channels, width, height, flattened_pixels)
    }

    unsafe fn activate_texture_unit(texture_unit: GLenum) -> bool {
        gl::ActiveTexture(texture_unit);
        
        let err = gl::GetError();
        if err != 0 { 
            println!("error: problem during activating texture unit ({err})");
            return false;
        }
        true
    }

    unsafe fn set_options() {
        // TODO: make this configurable?
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32)
    }
}
