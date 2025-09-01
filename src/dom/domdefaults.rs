use crate::dom::styleengine::{BorderStyle, BoxModelValues, BoxSizing, Color, ComputedElementStyle, Display, ElementStyle, Float, Position, PositionOffsets, TextStyle};

impl Default for BoxSizing {
    fn default() -> Self {
        BoxSizing::ContentBox
    }
}

impl Default for ComputedElementStyle {
    fn default() -> Self {
        Self {
            display: Display::Block,
            width: None,
            height: None,
            bg_color: Color::new(0.0, 0.0, 0.0, 1.0), // Black background
            margin: BoxModelValues::default(),
            padding: BoxModelValues::default(),
            border: BorderStyle::default(), // Default is no border on any side
            box_sizing: BoxSizing::ContentBox,
            float: Float::None,
            position: Position::Static,
            position_offsets: PositionOffsets::default()
        }
    }
}

impl Default for ElementStyle {
    fn default() -> Self {
        Self {
            display: None,
            width: None,
            height: None,
            bg_color: None, // Will default to black in computed style
            color: None,            // Will default to white in computed text style
            margin: None,
            padding: None,
            border: None,
            box_sizing: None,
            position: None,
            position_offsets: None,
            float: None,
        }
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: None, // Will default to white in computed text style
            font_size: None,
            font_family: None,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::Static
    }
}