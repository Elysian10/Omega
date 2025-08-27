// style.rs
use crate::dom::element::Color;

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

// User-facing style definition. Properties are optional.
#[derive(Debug, Clone, Default)]
pub struct Style {
    pub background_color: Option<Color>,
    // Future properties would go here:
    // pub color: Option<Color>,
    // pub width: Option<StyleValue>, // e.g. Px(100.0), Percent(50.0), Auto
    // pub padding: Option<f32>,
}

// Final, resolved style values. Properties are NOT optional.
#[derive(Debug, Clone)]
pub struct ComputedStyle {
    pub background_color: Color,
}

impl Default for ComputedStyle {
    fn default() -> Self {
        Self {
            // Define the default values for the entire application
            background_color: Color::new(0.0, 0.0, 0.0, 0.0), // Transparent
        }
    }
}