use indextree::NodeId;

use crate::dom::dom::Dom;

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

// NEW: A struct to hold values for things like padding and margin.
#[derive(Debug, Clone, Copy, Default)]
pub struct BoxModelValues {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

// MODIFIED: User-facing style definition now includes box model properties.
#[derive(Debug, Clone, Default)]
pub struct Style {
    pub background_color: Option<Color>,
    pub color: Option<Color>, // For text
    pub margin: Option<BoxModelValues>,
    pub padding: Option<BoxModelValues>,
    pub border_width: Option<f32>,
    pub border_color: Option<Color>,
}

// MODIFIED: Final, resolved style values now include box model properties.
#[derive(Debug, Clone)]
pub struct ComputedStyle {
    pub background_color: Color,
    pub color: Color, // For text
    pub margin: BoxModelValues,
    pub padding: BoxModelValues,
    pub border_width: f32,
    pub border_color: Color,
}

impl Default for ComputedStyle {
    fn default() -> Self {
        Self {
            background_color: Color::new(0.0, 0.0, 0.0, 0.0), // Transparent
            color: Color::new(1.0, 1.0, 1.0, 1.0),            // White text
            margin: BoxModelValues::default(),
            padding: BoxModelValues::default(),
            border_width: 0.0,
            border_color: Color::new(0.0, 0.0, 0.0, 1.0), // Opaque black
        }
    }
}

pub struct StyleEngine;

impl StyleEngine {
    pub fn compute_styles(dom: &mut Dom) {
        if let Some(root_id) = dom.root {
            let all_nodes: Vec<NodeId> = root_id.descendants(&dom.arena).collect();
            
            for node_id in all_nodes {
                let user_style = dom.styles.get(&node_id).cloned().unwrap_or_default();

                let parent_style = node_id.ancestors(&dom.arena).nth(1)
                    .and_then(|parent_id| dom.computed_styles.get(&parent_id));

                let computed_style = Self::resolve_style(&user_style, parent_style);
                
                dom.computed_styles.insert(node_id, computed_style);
            }
        }
    }

    // MODIFIED: The resolver now handles all the new properties.
    fn resolve_style(style: &Style, parent_style: Option<&ComputedStyle>) -> ComputedStyle {
        let mut computed = parent_style.cloned().unwrap_or_default();

        // --- Resolve Background Color --- (Not inherited)
        if let Some(color) = style.background_color {
            computed.background_color = color;
        }

        // --- Resolve Margin --- (Not inherited)
        if let Some(margin) = style.margin {
            computed.margin = margin;
        }

        // --- Resolve Padding --- (Not inherited)
        if let Some(padding) = style.padding {
            computed.padding = padding;
        }

        // --- Resolve Border Width --- (Not inherited)
        if let Some(width) = style.border_width {
            computed.border_width = width;
        }

        // --- Resolve Border Color --- (Not inherited)
        if let Some(color) = style.border_color {
            computed.border_color = color;
        }

        // --- Resolve Text Color --- (Inherited)
        if let Some(color) = style.color {
            computed.color = color; // 1. Direct value
        } else if let Some(parent) = parent_style {
            computed.color = parent.color; // 2. Inherited value
        }
        // 3. The default is used if neither applies.

        computed
    }
}
