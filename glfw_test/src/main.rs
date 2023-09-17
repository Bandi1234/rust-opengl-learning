extern crate gl33;
extern crate glfw;
extern crate core;
extern crate nalgebra_glm;
extern crate image;

mod rendering;

use std::collections::HashMap;

use gl33 as gl;
use gl33::global_loader as gl_loader;
use glfw::{Action, Context, Key};
use rendering::vertex_buffer::VPositionTextureNormal;

pub use crate::rendering::{index_buffer::IndexBuffer, shader::Shader, vertex_buffer::{VertexBuffer, VPositionTexture, VPosition}, vertex_buffer_layout::VertexBufferLayout, vertex_array::VertexArray, renderer, texture_2d::Texture2D};

// TODO glfw code abstraction + extra events (errors, joystick, etc)
// TODO proper asset management (custom formats, caching, folder structure)
// TODO fix unstable fps count and unstable screen size
// TODO separate input system from glfw
// TODO fbx import
// TODO combine uniforms shaders and textures into materials
// TODO combine vertex data materials and logic into objects
// TODO audio with so_loud
// TODO rewrite font / gui renderer here (in rust)
// TODO frame buffers
// TODO PBR
// TODO separate logic (ECS maybe? scripting maybe?)

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let mut is_fullscreen: bool = false;
    let monitor: glfw::Monitor = glfw::Monitor::from_primary();

    let mut r : f32 = 0.0;
    
    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(800, 600, "Hello Rust!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_size_polling(true);

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    unsafe {
        gl_loader::load_global_gl(&|p| {
            let c_str = std::ffi::CStr::from_ptr(p as *const i8);
            let rust_str = c_str.to_str().unwrap();
            glfw.get_proc_address_raw(rust_str) as _
        });
    }

    renderer::enable_basic_blending();

    on_resize(800, 600);

    let mut shader;

    let (vertex_array, index_buffer) = parse_obj_new("textured_monke.obj");
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
        r += 1.0;
        if r >= 360.0 {
            r -= 360.0;
        }

        let model = nalgebra_glm::identity();
        let model = nalgebra_glm::scale(
            &model,
            &nalgebra_glm::vec3(2.0, 2.0, 2.0)
        );
        let model = nalgebra_glm::rotate(
            &model,
            (r *  std::f32::consts::PI) / 180.0,
            &nalgebra_glm::vec3(0.0, 1.0, 0.0)
        );
        let model = nalgebra_glm::translate(
            &model,
            &nalgebra_glm::vec3(0.0, 0.0, 0.0)
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


        renderer::clear();
        renderer::clear_color(0.2, 0.3, 0.3, 1.0);

        shader.bind();

        shader.set_uniform_mat4("u_mvp", &mvp);
        
        renderer::draw(&vertex_array, &index_buffer, &shader);

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            //println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Key(Key::F, _, Action::Press, _) => {
                    toggle_fullscreen(&mut is_fullscreen, &mut window, &monitor);
                }
                glfw::WindowEvent::Size(x, y) => {
                    on_resize(x, y)
                }
                _ => {}
            }
        }
    }
}

fn on_resize(width: i32, height: i32) {
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
    renderer::set_viewport(viewport_x, viewport_y, viewport_width, viewport_height);
}

fn toggle_fullscreen(is_fullscreen : &mut bool, window : &mut glfw::Window, monitor : &glfw::Monitor) {
    let video_mode = monitor.get_video_mode().expect("No video mode?");
    if *is_fullscreen {
        window.set_monitor(
            glfw::WindowMode::Windowed,
            300,
            300,
            800,
            600,
            Some(video_mode.refresh_rate)
        );
        *is_fullscreen = false;
    } else {
        window.set_monitor(
            glfw::WindowMode::FullScreen(monitor),
            0,
            0,
            video_mode.width,
            video_mode.height,
            Some(video_mode.refresh_rate)
        );
        *is_fullscreen = true;
    }
}

fn parse_obj_new(path : &str) -> (VertexArray, IndexBuffer) {
    let file = std::fs::read_to_string(path).expect("File not found");
    
    let mut positions = Vec::<(f32, f32, f32)>::new();
    let mut normals = Vec::<(f32, f32, f32)>::new();
    let mut tex_coords = Vec::<(f32, f32)>::new();

    let mut vertices = Vec::<VPositionTextureNormal>::new();
    let mut vert_map = HashMap::<(usize, usize, usize), usize>::new();
    let mut indices = Vec::<u32>::new();

    let file = file.split('\n');
    for line in file {
        let mut line = line.split(' ');
        let entry_type = line.next().unwrap();
        match entry_type {
            "v" => {
                
                // Positions
                let x : f32 = line.next().unwrap().trim().parse().unwrap();
                let y : f32 = line.next().unwrap().trim().parse().unwrap();
                let z : f32 = line.next().unwrap().trim().parse().unwrap();
                positions.push((x, y, z));
            }

            "vn" => {
                // Normals
                let x : f32 = line.next().unwrap().trim().parse().unwrap();
                let y : f32 = line.next().unwrap().trim().parse().unwrap();
                let z : f32 = line.next().unwrap().trim().parse().unwrap();
                normals.push((x, y, z));
            }

            "vt" => {
                let u : f32 = line.next().unwrap().trim().parse().unwrap();
                let v : f32 = line.next().unwrap().trim().parse().unwrap();
                tex_coords.push((u, v));
            }

            "f" => {
                // A face and its vertices
                for _ in 0..3 {
                    let mut vert_info = line.next().unwrap().split('/');

                    let pos_index = vert_info.next().unwrap().trim().parse::<usize>().unwrap() - 1;
                    let tex_coord_index = vert_info.next().unwrap().trim().parse::<usize>().unwrap() - 1;
                    let normal_index = vert_info.next().unwrap().trim().parse::<usize>().unwrap() - 1;

                    let (x, y, z) = positions[pos_index];
                    let (u, v) = tex_coords[tex_coord_index];
                    let (n_x, n_y, n_z) = normals[normal_index];
                    
                    let key = (pos_index, tex_coord_index, normal_index);

                    if vert_map.contains_key(&key) {
                        indices.push(vert_map[&key] as u32);
                    } else {
                        let vert = VPositionTextureNormal{x, y, z, u, v, n_x, n_y, n_z};
                        vertices.push(vert);
                        indices.push((vertices.len() - 1) as u32);
                        vert_map.insert(key, vertices.len() - 1);
                    }
                }
            }

            _ => {
                // Other
                continue;
            }
        }
    }

    let vbo = VertexBuffer::new::<VPositionTextureNormal>(
        vertices,
        false
    );
    let mut layout = VertexBufferLayout::new();
    layout.push(gl::GL_FLOAT, 3, false);
    layout.push(gl::GL_FLOAT, 2, false);
    layout.push(gl::GL_FLOAT, 3, false);
    let vertex_array = VertexArray::new(vbo, layout);
    vertex_array.bind();

    let len = indices.len();
    let index_buffer = IndexBuffer::new(indices, len as isize, false);
    index_buffer.bind();
    (vertex_array, index_buffer)
}