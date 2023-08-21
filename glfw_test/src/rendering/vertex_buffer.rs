use gl33 as gl;
use gl33::global_loader as gl_loader;

pub struct VPosition {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

pub struct VPositionTexture {
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub u : f32,
    pub v : f32
}

pub struct VertexBuffer {
    pub renderer_id : std::ffi::c_uint,
}

impl VertexBuffer {
    pub fn new<T>(data : Vec<T>, is_dynamic : bool) -> Self {
        let mut renderer_id = 0;

        unsafe {
            gl_loader::glGenBuffers(1, &mut renderer_id);
            gl_loader::glBindBuffer(gl::GL_ARRAY_BUFFER, renderer_id);
            if is_dynamic {
                gl_loader::glBufferData(
                    gl::GL_ARRAY_BUFFER,
                    (data.len() * std::mem::size_of::<T>()) as isize,
                    data.as_ptr().cast(),
                    gl::GL_DYNAMIC_DRAW
                );
            } else {
                gl_loader::glBufferData(
                    gl::GL_ARRAY_BUFFER,
                    (data.len() * std::mem::size_of::<T>()) as isize,
                    data.as_ptr().cast(),
                    gl::GL_STATIC_DRAW
                );
            }
        }
        VertexBuffer{renderer_id}
    }

    pub fn bind(&self) {
        unsafe {
            gl_loader::glBindBuffer(gl::GL_ARRAY_BUFFER, self.renderer_id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl_loader::glBindBuffer(gl::GL_ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl_loader::glDeleteBuffers(1, &self.renderer_id);
        }
    }
}
