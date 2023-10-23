struct TextLine {
    pub start_index : i32,
    pub length : i32,
    pub spaces : i32,
    pub is_paragraph_end : bool,
    pub is_empty : bool,
    pub total_advance : f32
}

impl TextLine {
    pub fn new(start_index : i32) -> Self {
        TextLine {
            start_index,
            length: 0,
            spaces: 0,
            is_paragraph_end: false,
            is_empty: true,
            total_advance: 0.0
        }
    }
}