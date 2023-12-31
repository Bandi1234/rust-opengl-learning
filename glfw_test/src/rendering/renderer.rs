use gl33 as gl;
use gl33::global_loader as gl_loader;

use crate::{VertexArray, IndexBuffer, Shader};

pub fn draw(vertex_array : &VertexArray, index_buffer : &IndexBuffer, shader : &Shader) {
    shader.bind();
    vertex_array.bind();
    index_buffer.bind();
    unsafe {
        gl_loader::glDrawElements(gl::GL_TRIANGLES, index_buffer.count() as i32, gl::GL_UNSIGNED_INT, 0 as *const _)
    }
}

pub fn clear() {
    unsafe {
        gl_loader::glClear(gl::GL_COLOR_BUFFER_BIT);
        gl_loader::glClear(gl::GL_DEPTH_BUFFER_BIT);
    }
}

pub fn clear_color(r : f32, g : f32, b : f32, a : f32) {
    unsafe {
        gl_loader::glClearColor(r, g, b, a);
    }
}

pub fn set_viewport(x : i32, y : i32, width : i32, height : i32) {
    unsafe {
        gl_loader::glViewport(x, y, width, height);
    }
}

pub fn enable_basic_blending() {
    unsafe {
        gl_loader::glEnable(gl::GL_DEPTH_TEST);
        gl_loader::glDepthFunc(gl::GL_LESS);
        gl_loader::glEnable(gl::GL_BLEND);
        gl_loader::glBlendFunc(gl::GL_SRC_ALPHA, gl::GL_ONE_MINUS_SRC_ALPHA);
    }
}

pub fn on_resize(width: i32, height: i32) {
    let viewport_x;
    let viewport_y;
    let viewport_width;
    let viewport_height;
    if width as f64 / height as f64 > (16.0 / 9.0) {
        // Window wider than widescreen
        viewport_height = height;
        viewport_y = 0;
        viewport_width = (height / 9) * 16;
        viewport_x = 0 + ((width - viewport_width) / 2);
    } else {
        // Window taller than widescreen
        viewport_width = width;
        viewport_x = 0;
        viewport_height = (width / 16) * 9;
        viewport_y = 0 + ((height - viewport_height) / 2);
    }
    set_viewport(viewport_x, viewport_y, viewport_width, viewport_height);
}