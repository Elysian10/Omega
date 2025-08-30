// /src/dom/styleengine.rs

use skia_safe::Color4f;

use crate::dom::dom::{Dom, NodeContent};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    None,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BoxModelValues {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

// An enum to represent different border styles.
#[derive(Debug, Clone, Copy, Default)]
pub struct BorderStyle {
    pub top: Option<BorderSide>,
    pub right: Option<BorderSide>,
    pub bottom: Option<BorderSide>,
    pub left: Option<BorderSide>,
}

#[derive(Debug, Clone, Copy)]
pub struct BorderSide {
    pub width: f32,
    pub color: Color,
}

// Element style with width and height properties
#[derive(Debug, Clone)]
pub struct ElementStyle {
    pub display: Option<Display>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub background_color: Option<Color>,
    pub color: Option<Color>,
    pub margin: Option<BoxModelValues>,
    pub padding: Option<BoxModelValues>,
    pub border: Option<BorderStyle>,
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub color: Option<Color>,
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
}

// Computed element style with width and height properties
#[derive(Debug, Clone)]
pub struct ComputedElementStyle {
    pub display: Display,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub background_color: Color,
    pub margin: BoxModelValues,
    pub padding: BoxModelValues,
    pub border: BorderStyle, // Changed from BorderStyle enum to BorderStyle struct
}

#[derive(Debug, Clone)]
pub struct ComputedTextStyle {
    pub color: Color,
    pub font_size: f32,
    pub font_family: String,
}

#[derive(Debug, Clone)]
pub enum ComputedStyle {
    Element(ComputedElementStyle),
    Text(ComputedTextStyle),
}

impl From<Color> for Color4f {
    fn from(color: Color) -> Self {
        Color4f::new(color.r, color.g, color.b, color.a)
    }
}

impl BorderStyle {
    pub fn uniform(width: f32, color: Color) -> Self {
        let side = BorderSide { width, color };
        Self {
            top: Some(side),
            right: Some(side),
            bottom: Some(side),
            left: Some(side),
        }
    }

    pub fn all(width: f32, color: Color) -> Self {
        Self::uniform(width, color)
    }

    pub fn top(mut self, width: f32, color: Color) -> Self {
        self.top = Some(BorderSide { width, color });
        self
    }

    pub fn right(mut self, width: f32, color: Color) -> Self {
        self.right = Some(BorderSide { width, color });
        self
    }

    pub fn bottom(mut self, width: f32, color: Color) -> Self {
        self.bottom = Some(BorderSide { width, color });
        self
    }

    pub fn left(mut self, width: f32, color: Color) -> Self {
        self.left = Some(BorderSide { width, color });
        self
    }
}

impl Default for ComputedElementStyle {
    fn default() -> Self {
        Self {
            display: Display::Block,
            width: None,
            height: None,
            background_color: Color::new(0.0, 0.0, 0.0, 1.0), // Black background
            margin: BoxModelValues::default(),
            padding: BoxModelValues::default(),
            border: BorderStyle::default(), // Default is no border on any side
        }
    }
}

impl Default for ElementStyle {
    fn default() -> Self {
        Self {
            display: None,
            width: None,
            height: None,
            background_color: None, // Will default to black in computed style
            color: None,            // Will default to white in computed text style
            margin: None,
            padding: None,
            border: None,
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

impl Dom {
    pub fn compute_styles(&mut self, viewport_width: f32, viewport_height: f32) {
        if let Some(root_id) = self.root {
            // Use our new depth-first collection method instead of indextree's descendants
            let all_nodes = self.collect_nodes_depth_first(root_id);

            for node_id in all_nodes {
                let key: slotmap::DefaultKey = node_id.into();

                match self.content.get(key) {
                    Some(NodeContent::Element(_)) => {
                        let user_style = self.element_styles.get(key).unwrap();
                        // Get parent style using our new parent method
                        let parent_style = self.parent(node_id).and_then(|parent_id| {
                            let parent_key: slotmap::DefaultKey = parent_id.into();
                            self.computed_element_styles.get(parent_key)
                        });

                        let computed_style = Self::resolve_element_style(&user_style, parent_style);
                        self.computed_element_styles.insert(key, computed_style);
                    }
                    Some(NodeContent::Text(_)) => {
                        let user_style = self.text_styles.get(key).unwrap();
                        // Get parent style using our new parent method
                        let parent_style = self.parent(node_id).and_then(|parent_id| {
                            let parent_key: slotmap::DefaultKey = parent_id.into();
                            self.computed_element_styles.get(parent_key)
                        });

                        let computed_style = Self::resolve_text_style(&user_style, parent_style);
                        self.computed_text_styles.insert(key, computed_style);
                    }
                    None => {}
                }
            }
        }
    }

    fn resolve_element_style(style: &ElementStyle, parent_style: Option<&ComputedElementStyle>) -> ComputedElementStyle {
        // Start with dark theme defaults
        let mut computed = ComputedElementStyle::default();

        if let Some(display) = style.display {
            computed.display = display;
        } else if let Some(parent) = parent_style {
            computed.display = parent.display;
        }

        // Apply element-specific styling
        if let Some(bg_color) = style.background_color {
            computed.background_color = bg_color;
        }

        if let Some(margin) = style.margin {
            computed.margin = margin;
        }

        if let Some(padding) = style.padding {
            computed.padding = padding;
        }

        if let Some(border) = style.border {
            computed.border = border;
        }

        computed
    }

    fn resolve_text_style(style: &TextStyle, parent_style: Option<&ComputedElementStyle>) -> ComputedTextStyle {
        // Start with defaults
        let mut computed = ComputedTextStyle {
            color: Color::new(1.0, 1.0, 1.0, 1.0), // White text by default
            font_size: 16.0,
            font_family: "Arial".to_string(),
        };

        // Inherit from parent if available
        if let Some(parent) = parent_style {
            // Text could inherit color from parent element
            computed.color = parent.background_color;
        }

        // Apply text-specific styling
        if let Some(color) = style.color {
            computed.color = color;
        }

        if let Some(font_size) = style.font_size {
            computed.font_size = font_size;
        }

        if let Some(font_family) = &style.font_family {
            computed.font_family = font_family.clone();
        }

        computed
    }
}
