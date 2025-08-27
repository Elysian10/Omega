use indextree::NodeId;
use skia_safe::{Font, FontMgr, FontStyle};
use std::collections::HashMap;


use crate::dom::{dom::Dom, fontmanager::get_thread_local_font_mgr, node::NodeContent}; // SoA-compatible imports

// Rect and LayoutData structs remain the same
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutData {
    pub computed_x: f32,
    pub computed_y: f32,
    pub actual_width: f32,
    pub actual_height: f32,
}

pub struct LayoutEngine;

impl LayoutEngine {
    /// Main entry point, simplified to kick off the recursive DFS.
    pub fn compute_layout(dom: &mut Dom, viewport_width: f32, viewport_height: f32) {
        if let Some(root_id) = dom.root {
            // The available space for the root is the whole viewport.
            let available_space = Rect {
                x: 0.0,
                y: 0.0,
                width: viewport_width,
                height: viewport_height,
            };
            // Start the single-pass DFS from the root.
            Self::layout_node(dom, root_id, available_space);
        }
    }

    /// The heart of the single-pass DFS layout algorithm.
    /// This function computes its own size and layout, arranges its children,
    /// and returns its own computed bounding box.
    fn layout_node(dom: &mut Dom, node_id: NodeId, available_space: Rect) -> Rect {
        // In a real engine, this would come from CSS styles associated with the node.
        let padding = (2.0, 2.0); // (horizontal, vertical) padding on each side

        // The content box is the space inside our padding.
        let mut content_box = Rect {
            x: available_space.x + padding.0,
            y: available_space.y + padding.1,
            width: available_space.width - padding.0 * 2.0,
            height: 0.0, // We will calculate this based on content/children.
        };

        let node_content = dom.content.get(&node_id).cloned();

        match node_content {
            Some(NodeContent::Element(_)) => {
                // This "cursor" tracks where the next child should be placed.
                let mut child_cursor_y = content_box.y;

                // --- RECURSIVE STEP (Pre-order work) ---
                // We iterate over children and lay them out one by one.
                let child_ids: Vec<NodeId> = node_id.children(&dom.arena).collect();
                for child_id in child_ids {
                    let child_available_space = Rect {
                        x: content_box.x,
                        y: child_cursor_y,
                        width: content_box.width,
                        // Height is unconstrained for now in this simple block layout
                        height: f32::INFINITY,
                    };

                    // The magic happens here: recursively call layout on the child.
                    let child_rect = Self::layout_node(dom, child_id, child_available_space);

                    // After the child call returns, we know its size.
                    // Now we can update our cursor for the next sibling.
                    child_cursor_y += child_rect.height;
                }

                // --- POST-ORDER WORK ---
                // All children have been laid out. Our content height is the total height they occupy.
                content_box.height = child_cursor_y - content_box.y;
            }
            Some(NodeContent::Text(text)) => {
                // Leaf node: measure the text to determine content size.
                let (_measured_width, measured_height) = Self::measure_text(&text.content, text.font_family.as_deref(), text.font_size, content_box.width);
                content_box.height = measured_height;
            }
            None => {
                // Empty element, give it a default height
                content_box.height = 30.0;
            }
        }

        // --- FINALIZATION ---
        // The node's final size includes its own padding.
        let final_rect = Rect {
            x: available_space.x,
            y: available_space.y,
            width: available_space.width, // Block elements take full available width.
            height: content_box.height + padding.1 * 2.0,
        };

        // Directly apply the result to the DOM's SoA layout data. No third pass needed!
        dom.layout.insert(
            node_id,
            LayoutData {
                computed_x: final_rect.x,
                computed_y: final_rect.y,
                actual_width: final_rect.width,
                actual_height: final_rect.height,
            },
        );

        // Return our computed rect so our parent can position us.
        final_rect
    }

    // The measure_text function remains exactly the same.
    fn calculate_line_height(font: &Font, font_size: f32) -> f32 {
        font_size * 9.0 / 8.0
    }

    // In measure_text function:
    fn measure_text(content: &str, font_family: Option<&str>, font_size: f32, max_width: f32) -> (f32, f32) {
        let font_mgr = get_thread_local_font_mgr();
        let typeface = font_mgr
            .match_family_style(font_family.unwrap_or("Arial"), FontStyle::normal())
            .unwrap_or_else(|| font_mgr.legacy_make_typeface(None, FontStyle::normal()).unwrap());

        let font = Font::new(typeface, font_size);
        let line_height = Self::calculate_line_height(&font, font_size);

        // Handle multi-line text by splitting on newlines first
        let lines: Vec<&str> = content.split('\n').collect();
        let mut wrapped_lines = Vec::new();

        for line in lines {
            let (text_width, _) = font.measure_str(line, None);

            if text_width <= max_width {
                wrapped_lines.push(line.to_string());
            } else {
                // Word wrapping for lines that are too long
                let words: Vec<&str> = line.split_whitespace().collect();
                let space_width = font.measure_str(" ", None).0;
                let mut current_line = String::new();
                let mut current_line_width = 0.0;

                for word in words {
                    let word_width = font.measure_str(word, None).0;
                    if current_line.is_empty() {
                        current_line = word.to_string();
                        current_line_width = word_width;
                    } else if current_line_width + space_width + word_width <= max_width {
                        current_line.push(' ');
                        current_line.push_str(word);
                        current_line_width += space_width + word_width;
                    } else {
                        wrapped_lines.push(current_line);
                        current_line = word.to_string();
                        current_line_width = word_width;
                    }
                }

                if !current_line.is_empty() {
                    wrapped_lines.push(current_line);
                }
            }
        }

        // Calculate max width and total height
        let max_line_width = wrapped_lines.iter().map(|line| font.measure_str(line, None).0).fold(0.0, f32::max).min(max_width);

        let total_height = wrapped_lines.len() as f32 * line_height;

        (max_line_width, total_height)
    }
}
