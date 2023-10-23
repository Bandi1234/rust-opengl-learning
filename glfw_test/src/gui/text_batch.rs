use crate::vertex_types::v_msdf_text::VMSDFText;

use super::font::Font;

pub const BATCH_SIZE : usize = 20000;

pub struct TextBatch {
    vertices : [VMSDFText; 4 * BATCH_SIZE],
    indices : [u32; 6 * BATCH_SIZE],
    quad_count : i32,
    font : Font
}

impl TextBatch {
    pub fn new(font : Font) -> Self {
        let quad_count = 0;
        let vertices = [VMSDFText::default(); 4 * BATCH_SIZE];
        let mut indices = [0 as u32; 6 * BATCH_SIZE];
        for i in 0..BATCH_SIZE {
            let current_index_point = 6 * i;
            let current_vertex_point = 4 * i;
            indices[current_index_point] = current_vertex_point as u32;
            indices[current_index_point + 1] = current_vertex_point  as u32 + 1;
            indices[current_index_point + 2] = current_vertex_point  as u32 + 2;
            indices[current_index_point + 3] = current_vertex_point  as u32 + 2;
            indices[current_index_point + 4] = current_vertex_point  as u32 + 3;
            indices[current_index_point + 5] = current_vertex_point  as u32 + 0;
        }
        TextBatch { vertices, indices, quad_count, font}
    }

    pub fn begin(&mut self) {
        self.quad_count = 0;
    }

    pub fn draw(&mut self, top_left : VMSDFText, top_right : VMSDFText, bottom_right : VMSDFText, bottom_left : VMSDFText) {
        let current_vertex_point = 4 * self.quad_count as usize;
        self.vertices[current_vertex_point] = top_left;
        self.vertices[current_vertex_point + 1] = top_right;
        self.vertices[current_vertex_point + 2] = bottom_right;
        self.vertices[current_vertex_point + 3] = bottom_left;

        self.quad_count += 1;
        if self.quad_count as usize == BATCH_SIZE {
            // self.end();
            self.begin();
        }
    }

    pub fn end(&mut self, window_width : u32, window_height: u32) {
        if self.quad_count == 0 {
            return;
        }
        
        // TODO
    }

    pub fn font(&mut self) -> &mut Font {
        &mut self.font
    }
}