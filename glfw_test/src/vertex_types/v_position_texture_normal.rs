use crate::VertexBufferLayout;

use gl33 as gl;

#[derive(PartialEq)]
pub struct VPositionTextureNormal {
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub u : f32,
    pub v : f32,
    pub n_x : f32,
    pub n_y : f32,
    pub n_z : f32
}

impl VPositionTextureNormal {
    pub fn generate_layout() -> VertexBufferLayout {
        let mut layout = VertexBufferLayout::new();
        layout.push(gl::GL_FLOAT, 3, false);
        layout.push(gl::GL_FLOAT, 2, false);
        layout.push(gl::GL_FLOAT, 3, false);
        layout
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let mut res : [u8; 32] = [0; 32];
        let mut i = 0;
        let data : [f32; 8] = [self.x, self.y, self.z, self.u, self.v, self.n_x, self.n_y, self.n_z];
        for d in data {
            for b in d.to_be_bytes() {
                res[i] = b;
                i += 1;
            }
        }
        res
    }

    pub fn from_bytes(data : [u8; 32]) -> Self {
        let mut floats : [f32; 8] = [0.0; 8];
        for i in 0..8 {
            let float_data = [
                data[4 * i],
                data[4 * i + 1],
                data[4 * i + 2],
                data[4 * i + 3]
            ];
            floats[i] = f32::from_be_bytes(float_data);
        }
        VPositionTextureNormal {
            x: floats[0],
            y: floats[1],
            z: floats[2],
            u: floats[3],
            v: floats[4],
            n_x: floats[5],
            n_y: floats[6],
            n_z: floats[7]
        }
    }
}