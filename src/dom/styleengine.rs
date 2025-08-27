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

pub struct StyleEngine;

impl StyleEngine {
    /// The main entry point for the style computation pass.
    /// Traverses the DOM tree and resolves styles for each node.
    pub fn compute_styles(dom: &mut Dom) {
        if let Some(root_id) = dom.root {
            // We need to collect all nodes first to avoid borrowing issues with `dom`
            let all_nodes: Vec<NodeId> = root_id.descendants(&dom.arena).collect();
            
            for node_id in all_nodes {
                // Get the user-defined style, or a default empty one if not present
                let user_style = dom.styles.get(&node_id).cloned().unwrap_or_default();

                // Get the parent's computed style, if it exists
                let parent_style = node_id.ancestors(&dom.arena).nth(1) // nth(0) is the node itself
                    .and_then(|parent_id| dom.computed_styles.get(&parent_id));

                let computed_style = Self::resolve_style(&user_style, parent_style);
                
                dom.computed_styles.insert(node_id, computed_style);
            }
        }
    }

    /// Resolves a single node's style based on its own style, its parent's, and defaults.
    fn resolve_style(style: &Style, parent_style: Option<&ComputedStyle>) -> ComputedStyle {
        let mut computed = ComputedStyle::default();

        // --- Resolve Background Color ---
        // This property is NOT inherited.
        // 1. Check for a direct value.
        if let Some(color) = style.background_color {
            computed.background_color = color;
        } 
        // 2. No inheritance, so we just stick with the default if no direct value is found.

        // --- Example of an INHERITED property (if we had one) ---
        // if let Some(color) = style.color {
        //     computed.color = color; // 1. Direct value
        // } else if let Some(parent) = parent_style {
        //     computed.color = parent.color; // 2. Inherited value
        // }
        // 3. The default from ComputedStyle::default() is used if neither applies.

        computed
    }
}