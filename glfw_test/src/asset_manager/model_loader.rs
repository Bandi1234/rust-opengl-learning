
use core::panic;
use std::collections::HashMap;

use crate::rendering::{vertex_buffer::VertexBuffer, vertex_array::VertexArray, index_buffer::IndexBuffer};
use crate::asset_manager::utils;
use crate::vertex_types::v_position_texture_normal::VPositionTextureNormal;

pub fn load_model_raw(name : &str) -> (VertexArray, IndexBuffer) {
    let raw_path = &(String::from(utils::MODEL) + name + ".obj");
    if !utils::f_exists(&raw_path) {
        panic!("Raw model (.obj) file does not exist! {}", raw_path);
    }
    let (v, i) = read_obj(&raw_path);
    gen_buffer(v, i)
}

pub fn load_model_cached(name : &str) -> (VertexArray, IndexBuffer) {
    let raw_path = &(String::from(utils::MODEL) + name + ".obj");
    let cached_path = &(String::from(utils::M_CACHE) + name + ".mod");
    if !utils::f_exists(&raw_path) {
        panic!("Raw model (.obj) file does not exist! {}", raw_path);
    }
    if utils::f_exists(&cached_path) {
        if utils::f_modif_dur(&raw_path) < utils::f_modif_dur(&cached_path) {
            // need to cache
            let (v, i) = read_obj(&raw_path);
            save_model(&cached_path, &v, &i);
            gen_buffer(v, i)
        } else {
            let (v, i) = read_model(&cached_path);
            gen_buffer(v, i)
        }
    } else {
        let (v, i) = read_obj(&raw_path);
        save_model(&cached_path, &v, &i);
        gen_buffer(v, i)
    }
}

fn read_model(path : &str) -> (Vec::<VPositionTextureNormal>, Vec::<u32>) {
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
    (vertices, indices)
}

fn read_obj(path : &str) -> (Vec::<VPositionTextureNormal>, Vec::<u32>) {
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

    (vertices, indices)
}

fn save_model(target : &str, vertices : &Vec::<VPositionTextureNormal>, indices : &Vec::<u32>) {
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
    std::fs::write(target, bytes).expect("Couldn't write file.");
}

fn gen_buffer(vertices : Vec::<VPositionTextureNormal>, indices : Vec::<u32>) -> (VertexArray, IndexBuffer){
    let vbo = VertexBuffer::new::<VPositionTextureNormal>(
        vertices,
        false
    );
    let layout = VPositionTextureNormal::generate_layout();
    let vertex_array = VertexArray::new(vbo, layout);
    vertex_array.bind();

    let len = indices.len();
    let index_buffer = IndexBuffer::new(indices, len as isize, false);
    index_buffer.bind();
    (vertex_array, index_buffer)
}
