use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AtlasLayout {
    pub atlas : Atlas,
    pub metrics : Metrics,
    pub glyphs : Vec<Glyph>,
    pub kerning : Vec<KerningPair>
}

#[derive(Serialize, Deserialize)]
pub struct Atlas {
    #[serde(rename = "type")]
    pub atlas_type : String,
    #[serde(rename = "distanceRange")]
    pub distance_range : i32,
    pub size : i32,
    pub width : i32,
    pub height : i32,
    #[serde(rename = "yOrigin")]
    pub y_origin : String
}

#[derive(Serialize, Deserialize)]
pub struct Metrics {
    #[serde(rename = "emSize")]
    pub em_size : i32,
    #[serde(rename = "lineHeight")]
    pub line_height : f32,
    pub ascender : f32,
    #[serde(rename = "underlineY")]
    pub underline_y : f32,
    #[serde(rename = "underlineThickness")]
    pub underline_thickness : f32
}


#[derive(Serialize, Deserialize)]
pub struct Glyph {
    pub unicode : u32,
    pub advance : f32,
    #[serde(rename = "planeBounds")]
    pub plane_bounds : Option<PlaneBounds>,
    #[serde(rename = "atlasBounds")]
    pub atlas_bounds : Option<AtlasBounds>
}


#[derive(Serialize, Deserialize)]
pub struct PlaneBounds {
    pub left : f32,
    pub bottom : f32,
    pub right : f32,
    pub top : f32
}

#[derive(Serialize, Deserialize)]
pub struct AtlasBounds {
    pub left : f32,
    pub bottom : f32,
    pub right : f32,
    pub top : f32
}


#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy)]
pub struct KerningPair {
    pub unicode1 : u32,
    pub unicode2 : u32,
    pub advance :f32
}