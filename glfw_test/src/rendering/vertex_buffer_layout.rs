use gl33 as gl;

pub struct VertexBufferLayout {
    stride : i32,
    elements : Vec<VertexBufferElement>
}

impl VertexBufferLayout {
    pub fn new() -> Self {
        Self {stride: 0, elements: Vec::new()}
    }

    pub fn elements(&self) -> &Vec<VertexBufferElement> {
        &self.elements
    }

    pub fn push(&mut self, data_type : gl::GLenum, count : i32, normalized : bool) {
        if normalized {
            self.elements.push(VertexBufferElement { count, data_type, normalized : 1 as std::ffi::c_uchar});
        } else {
            self.elements.push(VertexBufferElement { count, data_type, normalized : 0 as std::ffi::c_uchar});
        }
        self.stride += VertexBufferLayout::size_of_type(data_type) * count;
    }

    pub fn stride(&self) -> i32 {
        self.stride
    }
}

pub struct VertexBufferElement {
    pub count : i32,
    pub data_type : gl::GLenum,
    pub normalized : std::ffi::c_uchar
}

impl VertexBufferLayout {
    pub fn size_of_type(data_type : gl::GLenum) -> i32 {
		match data_type {
		    gl::GL_FLOAT => 4,
		    gl::GL_UNSIGNED_INT => 4,
		    gl::GL_UNSIGNED_BYTE => 1,
            _ => 1
		}
	}
}
