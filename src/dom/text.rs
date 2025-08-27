use crate::dom::styleengine::Color;

#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
    pub font_size: f32,
    pub font_family: Option<String>,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            font_size: 16.0, // Default font size
            font_family: None,
        }
    }
    
    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }
    
    pub fn with_font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = Some(font_family.into());
        self
    }
}