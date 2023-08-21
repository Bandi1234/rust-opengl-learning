use gl33::global_loader as gl_loader;

use super::vertex_buffer::VertexBuffer;
use super::vertex_buffer_layout::VertexBufferLayout;

pub struct VertexArray {
    renderer_id : std::ffi::c_uint,
    vbo : VertexBuffer,
    layout : VertexBufferLayout
}

impl VertexArray {
    pub fn new(vbo : VertexBuffer, layout : VertexBufferLayout) -> Self {
        unsafe {
            let mut renderer_id = 0;
            gl_loader::glGenVertexArrays(1, &mut renderer_id);
            let vao = Self{renderer_id, vbo, layout};
            vao.bind();
            vao.vbo.bind();
            let elements = vao.layout.elements();
            let mut offset  = 0;
            for (i, element) in elements.iter().enumerate() {
                gl_loader::glEnableVertexAttribArray(i as u32);
                gl_loader::glVertexAttribPointer(
                    i as u32,
                    element.count,
                    element.data_type,
                    element.normalized,
                    vao.layout.stride(),
                    offset as *const _
                );
                offset += element.count * VertexBufferLayout::size_of_type(element.data_type);
            }
            vao
        }
    }

    pub fn bind(&self) {
        gl_loader::glBindVertexArray(self.renderer_id);
    }

    pub fn unbind() {
        gl_loader::glBindVertexArray(0);
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl_loader::glDeleteBuffers(1, &self.renderer_id);
        }
    }
}
