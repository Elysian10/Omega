use crate::dom::styleengine::{AlignContent, AlignItems, AlignSelf, BorderStyle, BoxModelValues, BoxSizing, Color, ComputedStyle, Display, Style, FlexDirection, FlexWrap, Float, JustifyContent, Position, PositionOffsets, Font};

impl Default for BoxSizing {
    fn default() -> Self {
        BoxSizing::ContentBox
    }
}

impl Default for ComputedStyle {
    fn default() -> Self {
        Self {
            display: Display::Block,
            width: None,
            height: None,
            bg_color: Color::BLACK, // Black background
            margin: BoxModelValues::default(),
            padding: BoxModelValues::default(),
            border: BorderStyle::default(), // Default is no border on any side
            box_sizing: BoxSizing::ContentBox,
            float: Float::None,
            position: Position::Static,
            position_offsets: PositionOffsets::default(),

            flex_direction: FlexDirection::default(),
            justify_content: JustifyContent::default(),
            align_items: AlignItems::default(),
            align_content: AlignContent::default(),
            flex_wrap: FlexWrap::default(),
            align_self: AlignSelf::default(),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            color: Color::WHITE,
            font_family: "Arial".to_owned(),
            font_size: 16.0
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            display: None,
            width: None,
            height: None,
            bg_color: None,
            color: None,
            margin: None,
            padding: None,
            border: None,
            box_sizing: None,
            position: None,
            position_offsets: None,
            float: None,
            font: None,

            flex_direction: None,
            justify_content: None,
            align_items: None,
            align_content: None,
            flex_wrap: None,
            align_self: None,
            flex_grow: None,
            flex_shrink: None,
        }
    }
}

impl Default for Font {
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

impl Default for FlexDirection {
    fn default() -> Self {
        FlexDirection::Row
    }
}


impl Default for JustifyContent {
    fn default() -> Self {
        JustifyContent::Center
    }
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::Center
    }
}

impl Default for AlignContent {
    fn default() -> Self {
        AlignContent::Center
    }
}

impl Default for FlexWrap {
    fn default() -> Self {
        FlexWrap::NoWrap
    }
}

impl Default for AlignSelf {
    fn default() -> Self {
        AlignSelf::Auto
    }
}

impl BoxModelValues{
    pub fn all(value: f32) -> Self {
        Self {
            top: Some(value),
            right: Some(value),
            bottom: Some(value),
            left: Some(value),
        }
    }

    pub fn top(mut self, value: f32) -> Self {
        self.top = Some(value);
        self
    }
    
    pub fn right(mut self, value: f32) -> Self {
        self.right = Some(value);
        self
    }
    
    pub fn bottom(mut self, value: f32) -> Self {
        self.bottom = Some(value);
        self
    }
    
    pub fn left(mut self, value: f32) -> Self {
        self.left = Some(value);
        self
    }

    pub fn horizontal_padding(&self) -> f32 {
        self.left.unwrap_or(0.0) + self.right.unwrap_or(0.0)
    }
    
    pub fn vertical_padding(&self) -> f32 {
        self.top.unwrap_or(0.0) + self.bottom.unwrap_or(0.0)
    }

    pub fn get_right(&self) -> f32 {
        self.right.unwrap_or(0.0)
    }
    
    pub fn get_bottom(&self) -> f32 {
        self.bottom.unwrap_or(0.0)
    }
    
    pub fn get_left(&self) -> f32 {
        self.left.unwrap_or(0.0)
    }
    
    pub fn get_top(&self) -> f32 {
        self.top.unwrap_or(0.0)
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const CYAN: Self = Self { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const DARK_GRAY: Self = Self { r: 0.2, g: 0.2, b: 0.2, a: 1.0 };
    pub const GRAY: Self = Self { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const LIGHT_GRAY: Self = Self { r: 0.8, g: 0.8, b: 0.8, a: 1.0 };
    pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Self = Self { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
}