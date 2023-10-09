use gl33 as gl;
use gl33::{global_loader as gl_loader, ShaderType};

pub struct Shader {
    pub renderer_id : std::ffi::c_uint,
    uniform_location_cache : std::collections::HashMap<String, i32>,
}

// Base functionality
impl Shader {
    pub fn new(shaders : (Vec<u8>, Vec<u8>)) -> Self {
        let program = gl_loader::glCreateProgram();

        let vertex_shader = Self::compile_shader(gl::GL_VERTEX_SHADER, shaders.0.as_slice());
        let fragment_shader = Self::compile_shader(gl::GL_FRAGMENT_SHADER, shaders.1.as_slice());

        gl_loader::glAttachShader(program, vertex_shader);
        gl_loader::glAttachShader(program, fragment_shader);
        gl_loader::glLinkProgram(program);
        gl_loader::glDeleteShader(vertex_shader);
        gl_loader::glDeleteShader(fragment_shader);
        Shader { renderer_id: program, uniform_location_cache: std::collections::HashMap::new() }
    }

    pub fn compile_shader(shader_type : ShaderType, bytes : &[u8]) -> std::ffi::c_uint {
        let id = gl_loader::glCreateShader(shader_type);
        unsafe {
            gl_loader::glShaderSource(
                id,
                1,
                &(bytes.as_ptr().cast()),
                &(bytes.len().try_into().unwrap())
            );
        }
        gl_loader::glCompileShader(id);

        unsafe {
            let mut success = 0;
            gl_loader::glGetShaderiv(id, gl::GL_COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl_loader::glGetShaderInfoLog(
                    id,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                if shader_type == gl::GL_VERTEX_SHADER {
                    panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
                } else {
                    panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
                }
            }
        }
        id
    }

    pub fn bind(&self) {
        gl_loader::glUseProgram(self.renderer_id);
    }

    pub fn unbind() {
        gl_loader::glUseProgram(0);
    }

    fn get_uniform_location(&mut self, name : &str) -> i32 {
        unsafe {
            if self.uniform_location_cache.contains_key(name) {
                return self.uniform_location_cache[name];
            }
            let c_str = std::ffi::CString::new(name).unwrap();
            let loc = gl_loader::glGetUniformLocation(self.renderer_id, c_str.as_ptr() as *const u8);
            self.uniform_location_cache.insert(String::from(name), loc);
            loc
        }
    }
}

// Uniform setters
impl Shader {
    pub fn set_uniform_1f(&mut self, name : &str, v0 : f32) {
        unsafe {
            gl_loader::glUniform1f(self.get_uniform_location(name), v0);
        }
    }

    pub fn set_uniform_2f(&mut self, name : &str, v : &nalgebra_glm::Vec2) {
        unsafe {
            gl_loader::glUniform2f(self.get_uniform_location(name), v.x, v.y);
        }
    }

    pub fn set_uniform_3f(&mut self, name : &str, v : &nalgebra_glm::Vec3) {
        unsafe {
            gl_loader::glUniform3f(self.get_uniform_location(name), v.x, v.y, v.z);
        }
    }

    pub fn set_uniform_4f(&mut self, name : &str, v : &nalgebra_glm::Vec4) {
        unsafe {
            gl_loader::glUniform4f(self.get_uniform_location(name), v.x, v.y, v.z, v.w);
        }
    }

    pub fn set_uniform_mat4(&mut self, name : &str, m : &nalgebra_glm::Mat4) {
        unsafe {
            gl_loader::glUniformMatrix4fv(self.get_uniform_location(name), 1, 0, m.as_ptr());
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl_loader::glDeleteProgram(self.renderer_id);
    }
}
