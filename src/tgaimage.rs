use std::{fs::File, io::{self, BufWriter, Write}};

use tinytga::RawTga;

///// Colorspaces
pub trait ColorSpace {
    fn new() -> Self;
    fn shade(&mut self, intensity: f32);
    const BPP: u8;
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Grayscale {
    pub i: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct RGB {
    pub b: u8, pub g: u8, pub r: u8
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct RGBA {
    pub b: u8, pub g: u8, pub r: u8, pub a: u8
}

impl ColorSpace for Grayscale {
    fn new() -> Self {
        Grayscale {i: 0}
    }
    fn shade(&mut self, intensity: f32) {
        if intensity > 1.0 { return }
        self.i = ((self.i as f32) * intensity) as u8;
    }
    const BPP: u8 = 1;
}

impl ColorSpace for RGB {
    fn new() -> Self {
        RGB {r: 0, g: 0, b: 0}
    }
    fn shade(&mut self, intensity: f32) {
        if intensity > 1.0 { return }
        self.r = ((self.r as f32) * intensity) as u8;
        self.g = ((self.g as f32) * intensity) as u8;
        self.b = ((self.b as f32) * intensity) as u8;
    }
    const BPP: u8 = 3;
}


impl ColorSpace for RGBA {
    fn new() -> Self {
        RGBA {r: 0, g: 0, b: 0, a: 0}
    }
    fn shade(&mut self, intensity: f32) {
        if intensity > 1.0 { return }
        self.r = ((self.r as f32) * intensity) as u8;
        self.g = ((self.g as f32) * intensity) as u8;
        self.b = ((self.b as f32) * intensity) as u8;
        self.a = ((self.a as f32) * intensity) as u8;
    }
    const BPP: u8 = 4;
}

///// Image header

const DEVELOPER_AREA_REF: [u8; 4] = [0, 0, 0, 0];
const EXTENSION_AREA_REF: [u8; 4] = [0, 0, 0, 0];
const FOOTER: [u8; 18] = *b"TRUEVISION-XFILE.\0";

#[derive(Default)]
#[repr(packed)]
#[allow(dead_code)]
struct Header {
    idlength: u8,
    colormaptype: u8,
    datatypecode: u8,
    colormaporigin: u16,
    colormaplength: u16,
    colormapdepth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bitsperpixel: u8,
    imagedescriptor: u8,
}

///// Image

// converts sized type to raw u8, for writing out
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

/// Represents a TGA image.
/// I'm too lazy to write a fn to read from file, use tinytga instead for that.
pub struct Image <T: ColorSpace> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>
}

const MAX_CHUNK_LENGTH: u8 = 128;

impl <T: ColorSpace + Copy> Image<T>  {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width: width,
            height: height,
            data: vec![T::new(); width * height]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: T) -> Result<(), String> {
        if x>=self.width || y>=self.height {
            return Err(format!("OOB pixel coordinate: {x} by {y} ({} by {})", self.width, self.height));
        }
        self.data[x + y*self.width] = color;
        Ok(())
    }

    fn data_vec(&self) -> Vec<u8> {
        self.data
            .iter()
            .flat_map(|p| unsafe { any_as_u8_slice(p) })
            .copied()
            .collect::<Vec<u8>>()
    }
    pub fn write_tga_file(&self, filename: &str, vflip: bool, rle: bool) -> io::Result<()> {
        let mut out = BufWriter::new(
            File::options()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(filename)?,
        );

        let header = Header {
            idlength: 0,
            bitsperpixel: T::BPP << 3,
            width: self.width as u16,
            height: self.height as u16,
            datatypecode: if T::BPP == Grayscale::BPP {
                match rle { true => 11, false => 3 }
            } else {
                match rle { true => 10, false => 2 }
            },
            imagedescriptor: if vflip { 0x00 } else { 0x20 },
            ..Default::default()
        };

        out.write(unsafe {any_as_u8_slice(&header)})
            .expect("Error writing TGA header.");

        if !rle {   
            out.write(self.data_vec().as_slice())
                .expect("Error writing image data.");
        } else {
            self.write_rle_data(&mut out)
                .expect("Error writing image data with RLE.");
        };

        out.write(&DEVELOPER_AREA_REF)
            .expect("Error writing developer area ref.");  
        out.write(&EXTENSION_AREA_REF)
            .expect("Error writing extension area ref.");  
        out.write(&FOOTER)
            .expect("Error writing footer.");  

        Ok(())
    }

    fn write_rle_data(&self, out: &mut dyn Write) -> io::Result<()> {
        let data = self.data_vec();
        let n_pixels = self.width * self.height;
        let mut current_pixel = 0;
        while current_pixel < n_pixels {
            let chunk_start = current_pixel * T::BPP as usize;
            let mut current_byte = chunk_start;
            let mut run_length: u8 = 1;
            let mut raw = true;
            while current_pixel + (run_length as usize) < n_pixels && run_length < MAX_CHUNK_LENGTH
            {
                let next_pixel = current_byte + (T::BPP as usize);
                let succ_eq = data[current_byte..next_pixel]
                    == data[next_pixel..next_pixel + (T::BPP as usize)];
                current_byte += T::BPP as usize;
                if run_length == 1 {
                    raw = !succ_eq;
                }
                if raw && succ_eq {
                    run_length -= 1;
                    break;
                }
                if !raw && !succ_eq {
                    break;
                }
                run_length += 1;
            }
            current_pixel += run_length as usize;
            out.write(&[if raw {
                run_length - 1
            } else {
                run_length + 127
            }])?;
            out.write(
                &data[chunk_start
                    ..chunk_start + (if raw { run_length * T::BPP } else { T::BPP }) as usize],
            )?;
        }
        Ok(())
    }
}       

// converts tinytga image into our format   
pub fn convert_from_tinytga() -> Image<RGB> {
    let data = include_bytes!("../texture.tga");
    let img = RawTga::from_slice(data).unwrap();
    let (height, width) = (img.size().height, img.size().width);
    let raw_pixels: Vec<_> = img.pixels().collect();
    let mut new_pixels = vec![RGB::new(); (height*width) as usize];
    
    for pixel in raw_pixels {
        let (x, y) = (pixel.position.x, pixel.position.y);
        let color = pixel.color;
        new_pixels[(x + y*width as i32) as usize] = RGB {
            b: (color & 0xFF) as u8,
            g: ((color >> 8) & 0xFF) as u8,
            r: ((color >> 16) & 0xFF) as u8,
            // a: ((color >> 24) & 0xFF) as u8
        };
    }

    let mut image: Image<RGB> = Image::new(width as usize, height as usize);
    image.data = new_pixels;
    image
}