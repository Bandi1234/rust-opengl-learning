use crate::VertexBufferLayout;
use gl33 as gl;

#[derive(Clone, Copy)]
pub struct VMSDFText {
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32,
    pub atlas_u : f32,
    pub atlas_v : f32,
    pub local_u : f32,
    pub local_v : f32,
    pub g_w : f32,
    pub g_h : f32,
}

impl VMSDFText {
    pub fn generate_layout() -> VertexBufferLayout {
        let mut layout = VertexBufferLayout::new();
        layout.push(gl::GL_FLOAT, 3, false);
        layout.push(gl::GL_FLOAT, 4, false);
        layout.push(gl::GL_FLOAT, 2, false);
        layout.push(gl::GL_FLOAT, 2, false);
        layout.push(gl::GL_FLOAT, 2, false);
        layout
    }
}

impl Default for VMSDFText {
    fn default() -> Self {
        VMSDFText {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            atlas_u: 0.0,
            atlas_v: 0.0,
            local_u: 0.0,
            local_v: 0.0,
            g_w: 0.0,
            g_h: 0.0
        }
    }
}