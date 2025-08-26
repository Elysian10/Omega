use std::collections::HashMap;

use indextree::NodeId;
use skia_safe::{Font, FontMgr, FontStyle};

use crate::dom::{dom::Dom, node::NodeContent};

// We'll use a Rect for all layout calculations to better represent the box model.
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

// Your original LayoutData, which we'll use at the very end.
#[derive(Debug, Clone, Copy)]
pub struct LayoutData {
    pub computed_x: f32,
    pub computed_y: f32,
    pub actual_width: f32,
    pub actual_height: f32,
}

pub struct LayoutEngine;

impl LayoutEngine {
    /// The main entry point for the layout process.
    pub fn compute_layout(dom: &mut Dom, viewport_width: f32, viewport_height: f32) {
        if let Some(root_id) = dom.root {
            // This map will store the computed layout box for each node.
            let mut layout_map: HashMap<NodeId, Rect> = HashMap::new();

            // --- PASS 1: Calculate Size (Measure) ---
            // Traverses the tree from the bottom-up to determine the intrinsic size of each node.
            // It calculates 'width' and 'height' but leaves 'x' and 'y' as 0 for now.
            Self::calculate_size(dom, root_id, viewport_width, &mut layout_map);

            // --- PASS 2: Arrange Position (Layout) ---
            // Traverses the tree from the top-down, using the pre-calculated sizes
            // to determine the final (x, y) coordinates for each node.
            let viewport_rect = Rect {
                x: 0.0,
                y: 0.0,
                width: viewport_width,
                height: viewport_height,
            };
            Self::arrange_nodes(dom, root_id, viewport_rect, &mut layout_map);

            // --- PASS 3: Apply Results ---
            // Commits the final computed layout data to the nodes in the DOM tree.
            Self::apply_layouts(dom, layout_map);
        }
    }

    /// PASS 1: Recursively calculates the size of a node based on its children or content.
    /// Returns the computed size (width, height) of the current node.
    fn calculate_size(
    dom: &Dom,
    node_id: NodeId,
    available_width: f32,
    layout_map: &mut HashMap<NodeId, Rect>,
) -> Rect {
    let node = dom.arena.get(node_id).unwrap().get();
    // In a real engine, this would come from the node's CSS styles.
    let padding = (2.0, 2.0); // (horizontal, vertical) padding on each side

    // The space available for our content or for our children.
    let content_width = available_width - padding.0 * 2.0;
    let mut content_height = 0.0;

    match &node.content {
        // --- HANDLE ELEMENT NODES ---
        NodeContent::Element(_element) => {
            let child_ids: Vec<NodeId> = node_id.children(&dom.arena).collect();
            if !child_ids.is_empty() {
                // If the element has children, its height is determined by them.
                for child_id in child_ids {
                    let child_rect = Self::calculate_size(dom, child_id, content_width, layout_map);
                    content_height += child_rect.height;
                }
            } else {
                // This is an empty element (like <div></div>). Give it a default height.
                content_height = 30.0;
            }
        }
        // --- HANDLE TEXT NODES (THE NEW LOGIC) ---
        NodeContent::Text(text) => {
            // A text node is a leaf. Its size is determined by its content.
            // We call your measure_text function to get its dimensions.
            let (_measured_width, measured_height) = Self::measure_text(
                &text.content,
                text.font_family.as_deref(),
                text.font_size,
                content_width, // The text can wrap within the available content width.
            );
            content_height = measured_height;
        }
    }

    // The node's final size includes its own padding.
    let final_rect = Rect {
        x: 0.0, // Position is calculated in the arrange pass.
        y: 0.0,
        width: available_width, // Block elements take full available width.
        height: content_height + padding.1 * 2.0,
    };

    layout_map.insert(node_id, final_rect);
    final_rect
}

    /// PASS 2: Recursively sets the final (x, y) position of a node and its children.
    fn arrange_nodes(dom: &Dom, node_id: NodeId, parent_content_box: Rect, layout_map: &mut HashMap<NodeId, Rect>) {
        // Update this node's position based on the parent's content area.
        if let Some(node_layout) = layout_map.get_mut(&node_id) {
            node_layout.x = parent_content_box.x;
            node_layout.y = parent_content_box.y;
        }

        // Determine this node's content box to position its children within it.
        let padding = (2.0, 2.0);
        let node_layout = layout_map[&node_id];
        let mut current_child_box = Rect {
            x: node_layout.x + padding.0,
            y: node_layout.y + padding.1,
            width: node_layout.width - padding.0 * 2.0,
            height: 0.0, // Height is irrelevant here, we only need the starting y.
        };

        // Arrange each child, stacking them vertically within our content box.
        for child_id in node_id.children(&dom.arena) {
            Self::arrange_nodes(dom, child_id, current_child_box, layout_map);
            // Move the "cursor" down by the height of the child we just placed.
            current_child_box.y += layout_map[&child_id].height;
        }
    }

    /// PASS 3: Applies the final calculated layouts back to the DOM nodes.
    fn apply_layouts(dom: &mut Dom, layout_map: HashMap<NodeId, Rect>) {
        for (node_id, rect) in layout_map {
            if let Some(node) = dom.arena.get_mut(node_id) {
                node.get_mut().layout_data = Some(LayoutData {
                    computed_x: rect.x,
                    computed_y: rect.y,
                    actual_width: rect.width,
                    actual_height: rect.height,
                });
            }
        }
    }

    // Your measure_text function remains unchanged. It will be called from calculate_size.
    fn measure_text(content: &str, font_family: Option<&str>, font_size: f32, max_width: f32) -> (f32, f32) {
        // ... implementation is the same ...
        let font_mgr = FontMgr::new();
        let typeface = font_mgr
            .match_family_style(font_family.unwrap_or("Arial"), FontStyle::normal())
            .unwrap_or_else(|| font_mgr.legacy_make_typeface(None, FontStyle::normal()).unwrap());

        let font = Font::new(typeface, font_size);
        let (text_width, text_bounds) = font.measure_str(content, None);
        if text_width <= max_width {
            (text_width, text_bounds.height())
        } else {
            let words: Vec<&str> = content.split_whitespace().collect();
            let space_width = font.measure_str(" ", None).0;
            let mut lines = Vec::new();
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
                    lines.push(current_line);
                    current_line = word.to_string();
                    current_line_width = word_width;
                }
            }
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            let max_line_width = lines.iter().map(|line| font.measure_str(line, None).0).fold(0.0, f32::max).min(max_width);
            let line_height = font.measure_str("A", None).1.height() * 1.2;
            let total_height = lines.len() as f32 * line_height;
            (max_line_width, total_height)
        }
    }
}
