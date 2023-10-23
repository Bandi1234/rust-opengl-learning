use gl33 as gl;
use crate::asset_manager::utils;

pub struct ImageResource {
    pub bytes : Vec<u8>,
    pub width : i32,
    pub height : i32,
    pub internal_format : gl::GLenum,
    pub format : gl::GLenum,
    pub data_type : gl::GLenum,
    pub min_filter : gl::GLenum,
    pub mag_filter : gl::GLenum
}

pub enum ImageFilter {
    Linear,
    Nearest
}

pub enum ImageType {
    PngRgba8,
    BmpRgb8
}

pub fn load_image(name: &str, img_type : ImageType, min : ImageFilter, mag : ImageFilter) -> ImageResource {
    let bytes : Vec<u8>;
    let width : i32;
    let height : i32;
    let internal_format : gl::GLenum;
    let format : gl::GLenum;
    let data_type : gl::GLenum;
    let min_filter = match min {
        ImageFilter::Linear => gl::GL_LINEAR,
        ImageFilter::Nearest => gl::GL_NEAREST
    };
    let mag_filter = match mag {
        ImageFilter::Linear => gl::GL_LINEAR,
        ImageFilter::Nearest => gl::GL_NEAREST
    };
    match img_type {
        ImageType::PngRgba8 => {
            let img = image::io::Reader::open(String::from(utils::IMG) + name + ".png")
            .expect("File not found!")
            .decode()
            .expect("Image decode failed!");
            width = img.width() as i32;
            height = img.height() as i32;
            bytes = img.flipv().as_bytes().to_owned();
            internal_format = gl::GL_RGBA8;
            format = gl::GL_RGBA;
            data_type = gl::GL_UNSIGNED_BYTE;
        },
        ImageType::BmpRgb8 => {
            let img = image::io::Reader::open(String::from(utils::IMG) + name + ".bmp")
            .expect("File not found!")
            .decode()
            .expect("Image decode failed!");
            width = img.width() as i32;
            height = img.height() as i32;
            bytes = img.flipv().as_bytes().to_owned();
            internal_format = gl::GL_RGB8;
            format = gl::GL_RGB;
            data_type = gl::GL_UNSIGNED_BYTE;
        }
    }
    ImageResource {
        bytes,
        width,
        height,
        internal_format,
        format,
        data_type,
        min_filter,
        mag_filter
    }
}