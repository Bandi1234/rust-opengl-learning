use crate::asset_manager::utils;

pub fn load_shaders(vert : &str, frag : &str) -> (Vec<u8>, Vec<u8>) {
    (
        std::fs::read(String::from(utils::SHADER) + vert + ".glsl").expect("File not found!"),
        std::fs::read(String::from(utils::SHADER) + frag + ".glsl").expect("File not found!")
    )
}