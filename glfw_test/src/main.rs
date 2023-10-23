extern crate gl33;
extern crate glfw;
extern crate core;
extern crate nalgebra_glm;
extern crate image;
extern crate serde_json;
extern crate serde;

pub mod rendering;
pub mod asset_manager;
pub mod window;
pub mod gui;
pub mod vertex_types;

pub use crate::rendering::{index_buffer::IndexBuffer, shader::Shader, vertex_buffer::{VertexBuffer, VPositionTexture, VPosition}, vertex_buffer_layout::VertexBufferLayout, vertex_array::VertexArray, renderer, texture_2d::Texture2D};
pub use crate::asset_manager::{utils as assets, model_loader as models, shader_loader as shaders, image_loader as images};

// TODO logging, timers
// TODO rewrite font / gui renderer here (in rust)
    // TextBatch (new, begin, draw, end)
    // Separate vertex type for characters
    // TextRenderer -> placement logic
    // Font -> loading and processing data (needs JSON deserialization)
// TODO separate input system from glfw (thats gonna suck)
// TODO separate transform logic (kinda fucked rn), will require proper handling of rotations :(((
// TODO error handling (glfw errors, gl errors, rust errors)
// TODO fbx import
// TODO combine uniforms shaders and textures into materials
// TODO combine vertex data materials and logic into objects
// TODO audio with so_loud
// TODO frame buffers
// TODO PBR
// TODO separate logic (ECS maybe? scripting maybe?)

fn main() {
    let  (mut window, events) = window::Window::new("Hello rust", 1280, 720);
    window.setup_gl(3, 3);
    assets::init();

    let mut r : f32 = 0.0;
    const FPS : f64 = 144.0;

    let minimum_delta = 1.0 / FPS;

    let mut shader;
    let start = std::time::Instant::now();
    let (vertex_array, index_buffer) = models::load_model_cached("textured_monke");
    println!("Model load: {} ms", start.elapsed().as_millis());
    vertex_array.bind();
    index_buffer.bind();

    let start = std::time::Instant::now();
    shader = Shader::new(shaders::load_shaders("test_vertex", "test_fragment"));
    println!("Shader compilation and load: {} ms", start.elapsed().as_millis());
    shader.bind();

    let start = std::time::Instant::now();
    let texture = Texture2D::new(images::load_image(
        "textured_monke",
        images::ImageType::PngRgba8,
        images::ImageFilter::Nearest,
        images::ImageFilter::Nearest
    ));
    println!("Texture load: {} ms", start.elapsed().as_millis());
    texture.bind(0);
    
    let mut now = std::time::Instant::now();

    // Loop until the user closes the window
    while !window.should_close() {
        let delta = now.elapsed().as_millis() as f64 / 1000.0;
        if delta < minimum_delta {
            continue;
        }

        // Logic
        r += 360.0 * delta as f32;
        if r >= 360.0 {
            r -= 360.0;
        }

        let model = nalgebra_glm::identity();
        let model = nalgebra_glm::translate(
            &model,
            &nalgebra_glm::vec3(0.0, -0.5, 0.0)
        );
        let model = nalgebra_glm::scale(
            &model,
            &nalgebra_glm::vec3(1.0, 1.0, 1.0)
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

        now = std::time::Instant::now();
    }
}
