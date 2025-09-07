// /src/dom/styleengine.rs

use serde::{Serialize, Serializer};
use serde_json;
use skia_safe::Color4f;

use crate::dom::dom::{Dom, NodeContent};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Flex,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Size {
    Points(f32),
    Percent(f32),
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct BoxModelValues {
     pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}


// An enum to represent different border styles.
#[derive(Debug, Clone, Copy, Default)]
pub struct BorderStyle {
    pub top: Option<BorderSide>,
    pub right: Option<BorderSide>,
    pub bottom: Option<BorderSide>,
    pub left: Option<BorderSide>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
    Inherit,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Float {
    Left,
    Right,
    None,
    Inherit,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct PositionOffsets {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

#[derive(Debug, Clone, Copy, Serialize, Default)]
pub struct ComputedPositionOffsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct BorderSide {
    pub width: f32,
    pub color: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct Style {
    pub display: Option<Display>,
    pub width: Option<Size>,  // Changed from f32
    pub height: Option<Size>, // Changed from f32
    pub bg_color: Option<Color>,
    pub color: Option<Color>,
    pub margin: Option<BoxModelValues>,
    pub padding: Option<BoxModelValues>,
    pub border: Option<BorderStyle>,
    pub box_sizing: Option<BoxSizing>,
    pub position: Option<Position>,
    pub position_offsets: Option<ComputedPositionOffsets>,
    pub float: Option<Float>,
    pub font: Option<Font>,

    // NEW Flex container properties
    pub flex_direction: Option<FlexDirection>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub align_content: Option<AlignContent>,
    pub flex_wrap: Option<FlexWrap>,

    // NEW Flex item properties
    pub align_self: Option<AlignSelf>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    // flex_basis can be added later for more complexity
}

#[derive(Debug, Clone, Serialize)]
pub struct Font {
    pub color: Option<Color>,
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComputedStyle {
    pub display: Display,
    pub width: Option<Size>,
    pub height: Option<Size>,
    pub bg_color: Color,
    pub margin: BoxModelValues,
    pub padding: BoxModelValues,
    pub border: BorderStyle,
    pub position: Position,
    pub box_sizing: BoxSizing,
    pub position_offsets: PositionOffsets,
    pub float: Float,

    
    pub color: Color,
    pub font_size: f32,
    pub font_family: String,
    // NEW Flex container properties
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub flex_wrap: FlexWrap,

    // Flex item properties
    pub align_self: AlignSelf,
    pub flex_grow: f32, // Default: 0.0
    pub flex_shrink: f32, // Default: 1.0
                        // flex_basis can be added later for more complexity
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    // Sticky can be added later
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

impl Style {
    pub fn apply(&mut self, other: &Style) {
        if other.display.is_some() {
            self.display = other.display;
        }
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.bg_color.is_some() {
            self.bg_color = other.bg_color;
        }
        if other.color.is_some() {
            self.color = other.color;
        }
        if other.margin.is_some() {
            self.margin = other.margin;
        }
        if other.padding.is_some() {
            self.padding = other.padding;
        }
        if other.border.is_some() {
            self.border = other.border;
        }
        if other.box_sizing.is_some() {
            self.box_sizing = other.box_sizing;
        }
        if other.float.is_some() {
            self.float = other.float;
        }
        if other.position.is_some() {
            self.position = other.position;
        }
        if other.position_offsets.is_some() {
            self.position_offsets = other.position_offsets;
        }
    }
}

impl Dom {
    pub fn compute_styles(&mut self, viewport_width: f32, viewport_height: f32) {
        let all_nodes = self.collect_nodes_depth_first(self.root);

        for node_id in all_nodes {
            let key: slotmap::DefaultKey = node_id.into();

            match self.content.get(key) {
                Some(NodeContent::Element(_)) => {
                    let user_style = self.styles.get(key).unwrap();
                    // Get parent style using our new parent method
                    let parent_style = self.parent(node_id).and_then(|parent_id| {
                        let parent_key: slotmap::DefaultKey = parent_id.into();
                        self.computed_styles.get(parent_key)
                    });

                    let computed_style = Self::resolve_style(&user_style, parent_style);
                    self.computed_styles.insert(key, computed_style);
                }
                Some(NodeContent::Text(_)) => {
                }
                None => {}
            }
        }
    }

    fn resolve_style(style: &Style, parent_style: Option<&ComputedStyle>) -> ComputedStyle {
        // Start with dark theme defaults
        let mut computed = ComputedStyle::default();

        if let Some(display) = style.display {
            computed.display = display;
        } else if let Some(parent) = parent_style {
            computed.display = parent.display;
        }

        if let Some(box_sizing) = style.box_sizing {
            computed.box_sizing = match box_sizing {
                BoxSizing::Inherit => {
                    if let Some(parent) = parent_style {
                        parent.box_sizing
                    } else {
                        BoxSizing::ContentBox
                    }
                }
                _ => box_sizing,
            };
        }

        if let Some(float) = style.float {
            computed.float = match float {
                Float::Inherit => {
                    if let Some(parent) = parent_style {
                        parent.float
                    } else {
                        Float::None
                    }
                }
                _ => float,
            };
        }

        // Apply element-specific styling
        if let Some(bg_color) = style.bg_color {
            computed.bg_color = bg_color;
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

        if let Some(position) = style.position {
            computed.position = position;
        }

        computed
    }

}
