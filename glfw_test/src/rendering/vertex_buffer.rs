
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
