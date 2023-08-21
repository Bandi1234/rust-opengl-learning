use gl33 as gl;
use gl33::global_loader as gl_loader;

pub struct IndexBuffer {
    renderer_id : std::ffi::c_uint,
    count : isize,
}

impl IndexBuffer {
    pub fn new(data : Vec<u32>, count : isize, is_dynamic : bool) -> Self {
        let mut renderer_id = 0;

        unsafe {
            gl_loader::glGenBuffers(1, &mut renderer_id);
            gl_loader::glBindBuffer(gl::GL_ELEMENT_ARRAY_BUFFER, renderer_id);
            if is_dynamic {
                gl_loader::glBufferData(
                    gl::GL_ELEMENT_ARRAY_BUFFER,
                    (std::mem::size_of::<u32>() as isize) * count,
                    data.as_ptr().cast(),
                    gl::GL_DYNAMIC_DRAW
                );
            } else {
                gl_loader::glBufferData(
                    gl::GL_ELEMENT_ARRAY_BUFFER,
                    (std::mem::size_of::<u32>() as isize) * count,
                    data.as_ptr().cast(),
                    gl::GL_STATIC_DRAW
                );
            }
        }
        IndexBuffer {renderer_id, count}
    }

    pub fn bind(&self) {
        unsafe {
            gl_loader::glBindBuffer(gl::GL_ELEMENT_ARRAY_BUFFER, self.renderer_id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl_loader::glBindBuffer(gl::GL_ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn count(&self) -> isize {
        self.count
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl_loader::glDeleteBuffers(1, &self.renderer_id);
        }
    }
}
