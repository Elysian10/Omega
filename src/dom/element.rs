use crate::dom::styleengine::Color;

#[derive(Debug, Clone)]
pub struct Element {
    pub width: Option<f32>,  // None means "auto" or "unspecified"
    pub height: Option<f32>, // None means "auto" or "unspecified"
}

impl Element {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
        }
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
    
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }
}

