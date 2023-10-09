use gl33 as gl;
use gl33::global_loader as gl_loader;
use crate::asset_manager::image_loader;

pub struct Texture2D {
    renderer_id : u32,
    width : i32,
    height : i32
}

impl Texture2D {
    pub fn new(img : image_loader::ImageResource) -> Self {
        let mut renderer_id = 0;
        unsafe {
            gl_loader::glGenTextures(1, &mut renderer_id);
            gl_loader::glBindTexture(gl::GL_TEXTURE_2D, renderer_id);
    
            gl_loader::glTexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_MIN_FILTER, img.min_filter.0 as i32);
            gl_loader::glTexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_MAG_FILTER, img.mag_filter.0 as i32);
            gl_loader::glTexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_WRAP_S, gl::GL_CLAMP_TO_EDGE.0 as i32);
            gl_loader::glTexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_WRAP_T, gl::GL_CLAMP_TO_EDGE.0 as i32);
            gl_loader::glTexImage2D(
                gl::GL_TEXTURE_2D,
                0,
                img.internal_format.0 as i32,
                img.width,
                img.height,
                0,
                img.format,
                img.data_type,
                img.bytes.as_ptr().cast()
            );
        }
        Texture2D{renderer_id, width : img.width, height : img.height}
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn bind(&self, slot : u32) {
        unsafe {
            gl_loader::glActiveTexture(gl::GLenum(gl::GL_TEXTURE0.0 + slot));
            gl_loader::glBindTexture(gl::GL_TEXTURE_2D, self.renderer_id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl_loader::glBindTexture(gl::GL_TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            gl_loader::glDeleteTextures(1, &self.renderer_id);
        }
    }
}
