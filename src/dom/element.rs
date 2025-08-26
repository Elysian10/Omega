#[derive(Debug, Clone)]
pub struct Element {
    pub width: Option<f32>,  // None means "auto" or "unspecified"
    pub height: Option<f32>, // None means "auto" or "unspecified"
    pub color: Color,
}

// Define our own color type to avoid Skia dependencies
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Element {
    pub fn new(color: Color) -> Self {
        Self {
            width: None,
            height: None,
            color,
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

