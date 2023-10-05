extern crate gl33;
extern crate glfw;
extern crate core;
extern crate nalgebra_glm;
extern crate image;

pub mod rendering;
pub mod asset_manager;
pub mod window;

use gl33 as gl;

pub use crate::rendering::{index_buffer::IndexBuffer, shader::Shader, vertex_buffer::{VertexBuffer, VPositionTexture, VPosition}, vertex_buffer_layout::VertexBufferLayout, vertex_array::VertexArray, renderer, texture_2d::Texture2D};

// TODO separate proper asset management (custom formats, caching, folder structure)
// TODO separate transform logic (kinda fucked rn)
// TODO fix unstable fps count and unstable render size
// TODO logging
// TODO error handling (glfw errors, gl errors, rust errors)
// TODO separate input system from glfw (thats gonna suck)
// TODO fbx import
// TODO combine uniforms shaders and textures into materials
// TODO combine vertex data materials and logic into objects
// TODO audio with so_loud
// TODO rewrite font / gui renderer here (in rust)
// TODO frame buffers
// TODO PBR
// TODO separate logic (ECS maybe? scripting maybe?)

fn main() {
    let  (mut window, events) = window::Window::new("Hello rust", 1280, 720);

    let mut r : f32 = 0.0;

    window.setup_gl(3, 3);

    asset_manager::init();

    let mut shader;
    let start = std::time::Instant::now();
    let (vertex_array, index_buffer) = asset_manager::load_model_cached("textured_monke");
    println!("{} ms elapsed", start.elapsed().as_millis());
    vertex_array.bind();
    index_buffer.bind();

    shader = Shader::new("test_vertex.glsl", "test_fragment.glsl");
    shader.bind();

    let texture = Texture2D::new(
        "textured_monke.png",
        gl::GL_RGBA8,
        gl::GL_RGBA,
        gl::GL_UNSIGNED_BYTE,
        gl::GL_LINEAR
    );
    texture.bind(0);

    // Loop until the user closes the window
    while !window.should_close() {
        // Logic
        r += 200.0 * (1.0 / 60.0);
        if r >= 360.0 {
            r -= 360.0;
        }

        let model = nalgebra_glm::identity();
        let model = nalgebra_glm::translate(
            &model,
            &nalgebra_glm::vec3(0.0, -2.0, 0.0)
        );
        let model = nalgebra_glm::scale(
            &model,
            &nalgebra_glm::vec3(0.1, 0.1, 0.1)
        );
        let model = nalgebra_glm::rotate(
            &model,
            (r *  std::f32::consts::PI) / 180.0,
            &nalgebra_glm::vec3(0.0, 1.0, 0.0)
        );
        
        
        let view = nalgebra_glm::look_at(
            &nalgebra_glm::vec3(4.0, 3.0, 3.0),
            &nalgebra_glm::vec3(0.0, 0.0, 0.0),
            &nalgebra_glm::vec3(0.0, 1.0, 0.0)
        );
        let projection = nalgebra_glm::perspective(
            16.0 / 9.0,
            std::f32::consts::PI / 4.0,
            0.1,
            100.0
        );
        let mvp = projection * view * model;

        // Drawing
        renderer::clear();
        renderer::clear_color(0.2, 0.3, 0.3, 1.0);

        shader.bind();

        shader.set_uniform_mat4("u_mvp", &mvp);
        
        renderer::draw(&vertex_array, &index_buffer, &shader);

        // Events
        window.swap_buffers();

        window.handle_events(&events);
    }
}
