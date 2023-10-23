use std::collections::HashMap;

use serde_json;

use crate::rendering::texture_2d::Texture2D;
use crate::asset_manager::{utils as assets, image_loader as images};

use super::atlas_layout::{AtlasLayout, Glyph, Atlas, Metrics};

pub struct Font {
    name : String,
    texture : Texture2D,
    // batch : TextBatch
    atlas : Atlas,
    metrics : Metrics,
    glyphs : HashMap<char, Glyph>,
    kerning_pairs : HashMap<(char, char), f32>
}

impl Font {
    pub fn new(name : &str) -> Self {
        let texture = Texture2D::new(images::load_image(
            name,
            images::ImageType::BmpRgb8,
            images::ImageFilter::Linear, 
            images::ImageFilter::Linear
        ));

        let full = std::fs::read_to_string(String::from(assets::LAYOUT) + name + ".json").expect("File not found!");
        let layout : AtlasLayout = serde_json::from_str(&full).expect("Json deser error.");

        let mut glyphs = HashMap::<char, Glyph>::new();
        let mut kerning_pairs = HashMap::<(char, char), f32>::new(); 

        for glyph in layout.glyphs {
            glyphs.insert(char::from_u32(glyph.unicode).expect("Watt"), glyph);
        }

        for pair in layout.kerning {
            kerning_pairs.insert(
                (
                    char::from_u32(pair.unicode1).unwrap(), 
                    char::from_u32(pair.unicode2).unwrap()
                ),
                pair.advance
            );
        }
        Font {name: String::from(name), texture, metrics: layout.metrics, atlas : layout.atlas, glyphs, kerning_pairs}
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn texture(&self) -> &Texture2D {
        &self.texture
    }

    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    pub fn atlas(&self) -> &Atlas {
        &self.atlas
    }

    pub fn glyphs(&self) -> &HashMap<char, Glyph> {
        &self.glyphs
    }

    pub fn kerning_pairs(&self) -> &HashMap<(char, char), f32> {
        &self.kerning_pairs
    }
}