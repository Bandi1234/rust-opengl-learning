use std::collections::HashMap;

use gl33 as gl;
use crate::rendering::{vertex_buffer::VPositionTextureNormal, vertex_buffer::VertexBuffer, vertex_array::VertexArray, vertex_buffer_layout::VertexBufferLayout, index_buffer::IndexBuffer};

pub fn parse_obj_new(path : &str) -> (VertexArray, IndexBuffer) {
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

pub fn convert_obj(path : &str) {
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

    //Save file
    let mut bytes = Vec::<u8>::new();
    for b in (vertices.len() as u32).to_be_bytes() {
        bytes.push(b);
    }
    for vert in vertices {
        for b in vert.to_bytes() {
            bytes.push(b);
        }
    }
    for b in (indices.len() as u32).to_be_bytes() {
        bytes.push(b);
    }
    for index in indices {
        for b in index.to_be_bytes() {
            bytes.push(b);
        }
    }
    /*let f_name : &str = std::path::Path::new(path).file_name().unwrap().into();
    let f_name = f_name.to_owned();*/
    std::fs::write("asd.asd", bytes).expect("Couldn't write file.");
}

pub fn read_model(path : &str) -> (VertexArray, IndexBuffer) {
    let mut vertices = Vec::<VPositionTextureNormal>::new();
    let mut indices = Vec::<u32>::new();
    
    let bytes = std::fs::read(path).expect("File not found!");

    let vert_len = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let vert_len : usize = vert_len as usize;

    let mut i = 4;
    for _ in 0..vert_len {
        let mut data : [u8; 32] = [0; 32];
        for j in 0..32 {
            data[j] = bytes[i + j];
        }
        i += 32;
        vertices.push(VPositionTextureNormal::from_bytes(data));
    }

    let ind_len = u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
    i+=4;
    let ind_len : usize = ind_len as usize;

    for _ in 0..ind_len {
        let mut data : [u8; 4] = [0; 4];
        for j in 0..4 {
            data[j] = bytes[i + j];
        }
        i += 4;
        indices.push(u32::from_be_bytes(data));
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