use gl33 as gl;
use gl33::global_loader as gl_loader;

pub struct Texture2D {
    renderer_id : u32,
    width : i32,
    height : i32
}

impl Texture2D {
    pub fn new(path : &str, internal_format : gl::GLenum, format : gl::GLenum, data_type : gl::GLenum, filter : gl::GLenum) -> Self {
        let mut renderer_id = 0;
        let width;
        let height;
        unsafe {
            let img = image::io::Reader::open(path).unwrap().decode().unwrap();
            width = img.width() as i32;
            height = img.height() as i32;
            
            gl_loader::glGenTextures(1, &mut renderer_id);
            gl_loader::glBindTexture(gl::GL_TEXTURE_2D, renderer_id);
    
            gl_loader::glTexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_MIN_FILTER, filter.0 as i32);
            gl_loader::glTexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_MAG_FILTER, filter.0 as i32);
            gl_loader::glTexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_WRAP_S, gl::GL_CLAMP_TO_EDGE.0 as i32);
            gl_loader::glTexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_WRAP_T, gl::GL_CLAMP_TO_EDGE.0 as i32);
            gl_loader::glTexImage2D(
                gl::GL_TEXTURE_2D,
                0,
                internal_format.0 as i32,
                width,
                height,
                0,
                format,
                data_type,
                img.flipv().as_bytes().as_ptr().cast()
            );
        }
        Texture2D{renderer_id, width, height}
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
