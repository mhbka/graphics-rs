use image::{io::Reader as ImageReader, GenericImageView, Pixel, Pixels};
use gl::types::*;

/// Wrapper struct for a texture.
pub struct Texture {
    filename: String,
    texture: u32,
}

// Public fns
impl Texture {
    pub unsafe fn new(filename: &str) -> Self {
        // gen and bind a texture object
        let mut texture = 0;
        gl::GenTextures(1, &mut texture as *mut u32);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // set options for the texture
        Texture::set_options(texture);

        // load texture image data and copy tocurrently bound texture
        let (width, height, flattened_pixels) = Texture::load_image_data_rgb(filename);
        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGB as i32, // ??
            width as i32, 
            height as i32, 
            0, 
            gl::RGB,
            gl::UNSIGNED_BYTE,
            flattened_pixels.as_ptr() as *const GLvoid
        );

        // generate a mipmap for this texture
        gl::GenerateMipmap(gl::TEXTURE_2D);

        Texture {filename: filename.to_owned(), texture}
    }
}

// Internal implementations
impl Texture {
    fn load_image_data_rgb(filename: &str) -> (u32, u32, Vec<u8>) {
        let img = ImageReader::open(&format!("assets/textures/{filename}"))
            .expect(&format!("Couldn't load texture image: {filename}"))
            .decode()
            .expect(&format!("Couldn't decode texture image: {filename}"))
            .to_rgb8();

        let (width, height) = img.dimensions();

        // openGL expects a flat array of u8, so we must flatten before returning
        let flattened_pixels: Vec<u8> = img.pixels().map(|p| p.0).flatten().collect();

        (width, height, flattened_pixels)
    }

    unsafe fn set_options(texture: u32) {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32)
    }
}
