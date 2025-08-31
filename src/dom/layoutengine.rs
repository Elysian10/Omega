// /src/dom/layoutengine.rs
use crate::dom::dom::{Dom, NodeContent, NodeId};
use crate::dom::styleengine::{BorderStyle, ComputedElementStyle, Display};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct LayoutData {
    pub computed_x: f32,
    pub computed_y: f32,
    pub actual_width: f32,
    pub actual_height: f32,
}

#[derive(Debug, Clone)]
pub struct TextInfo {
    pub lines: Vec<String>,
    pub line_heights: Vec<f32>,
    pub line_widths: Vec<f32>,
}

impl Dom {
    pub fn compute_layout(&mut self, viewport_width: f32, viewport_height: f32) {
        if let Some(root_id) = self.root {
            let available_space = Rect {
                x: 0.0,
                y: 0.0,
                width: viewport_width,
                height: viewport_height,
            };
            self.layout_node(root_id, available_space);
        }
    }

    fn layout_node(&mut self, node_id: NodeId, available_space: Rect) -> Rect {
        let key: slotmap::DefaultKey = node_id.into();

        // If display is 'none', return a zero-sized rect and do nothing.
        if let Some(style) = self.computed_element_styles.get(key) {
            if style.display == Display::None {
                self.layout.insert(
                    key,
                    LayoutData {
                        computed_x: available_space.x,
                        computed_y: available_space.y,
                        actual_width: 0.0,
                        actual_height: 0.0,
                    },
                );
                return Rect {
                    x: available_space.x,
                    y: available_space.y,
                    width: 0.0,
                    height: 0.0,
                };
            }
        }

        let node_content_type = self.content.get(key).cloned();

        match node_content_type {
            Some(NodeContent::Element(_)) => {
                let element_style = self.computed_element_styles.get(key).unwrap().clone();
                let child_ids = self.children.get(key).cloned().unwrap_or_default();

                match element_style.display {
                    Display::Block => self.layout_block_node(node_id, available_space, &element_style, &child_ids),
                    Display::Inline | Display::InlineBlock => self.layout_inline_node(node_id, available_space, &element_style, &child_ids),
                    Display::None => unreachable!(), // Handled above
                }
            }
            Some(NodeContent::Text(text)) => {
                let text_style = self.computed_text_styles.get(key).unwrap().clone();
                let (_measured_width, measured_height, text_info) = Self::measure_text(&text.content, Some(&text_style.font_family), text_style.font_size, available_space.width);
                self.text_info.insert(key, text_info);

                let text_rect = Rect {
                    x: available_space.x,
                    y: available_space.y,
                    width: available_space.width, // Text takes available width initially
                    height: measured_height,
                };

                self.layout.insert(
                    key,
                    LayoutData {
                        computed_x: available_space.x,
                        computed_y: available_space.y,
                        actual_width: available_space.width, // Will be constrained by parent
                        actual_height: measured_height,
                    },
                );
                text_rect
            }
            None => Rect::default(),
        }
    }

    /// Lays out a block-level element.
    /// It calculates its content box and then uses `layout_inline_children` to position child nodes.
    fn layout_block_node(&mut self, node_id: NodeId, available_space: Rect, element_style: &ComputedElementStyle, child_ids: &[NodeId]) -> Rect {
        let key: slotmap::DefaultKey = node_id.into();

        let border_left = element_style.border.left.map(|b| b.width).unwrap_or(0.0);
        let border_right = element_style.border.right.map(|b| b.width).unwrap_or(0.0);
        let border_top = element_style.border.top.map(|b| b.width).unwrap_or(0.0);
        let border_bottom = element_style.border.bottom.map(|b| b.width).unwrap_or(0.0);

        let content_x = available_space.x + element_style.margin.left + border_left + element_style.padding.left;
        let content_y = available_space.y + element_style.margin.top + border_top + element_style.padding.top;
        let content_width = available_space.width - (element_style.margin.left + element_style.margin.right + border_left + border_right + element_style.padding.left + element_style.padding.right);

        // **CHANGE**: Instead of a simple vertical loop, we now use the powerful inline layout function for all children.
        let content_box = Rect { x: content_x, y: content_y, width: content_width, height: f32::INFINITY };
        let (_used_width, used_height) = self.layout_inline_children(child_ids, content_box);

        let content_height = used_height;

        let padding_box_height = content_height + element_style.padding.top + element_style.padding.bottom;
        let border_box_height = padding_box_height + border_top + border_bottom;
        let final_height_with_margin = border_box_height + element_style.margin.top + element_style.margin.bottom;

        let border_box_x = available_space.x + element_style.margin.left;
        let border_box_y = available_space.y + element_style.margin.top;
        let border_box_width = available_space.width - (element_style.margin.left + element_style.margin.right);

        self.layout.insert(
            key,
            LayoutData {
                computed_x: border_box_x,
                computed_y: border_box_y,
                actual_width: border_box_width,
                actual_height: border_box_height,
            },
        );

        Rect {
            x: available_space.x,
            y: available_space.y,
            width: available_space.width,
            height: final_height_with_margin,
        }
    }

    /// Lays out an inline or inline-block element.
    fn layout_inline_node(&mut self, node_id: NodeId, available_space: Rect, element_style: &ComputedElementStyle, child_ids: &[NodeId]) -> Rect {
        let key: slotmap::DefaultKey = node_id.into();

        let border_left = element_style.border.left.map(|b| b.width).unwrap_or(0.0);
        let border_right = element_style.border.right.map(|b| b.width).unwrap_or(0.0);
        let border_top = element_style.border.top.map(|b| b.width).unwrap_or(0.0);
        let border_bottom = element_style.border.bottom.map(|b| b.width).unwrap_or(0.0);

        let content_x = available_space.x + element_style.margin.left + border_left + element_style.padding.left;
        let content_y = available_space.y + element_style.margin.top + border_top + element_style.padding.top;
        let content_width = available_space.width - (element_style.margin.left + element_style.margin.right + border_left + border_right + element_style.padding.left + element_style.padding.right);
        
        // **CHANGE**: Layout children using the dedicated inline layout function.
        let content_box = Rect { x: content_x, y: content_y, width: content_width, height: f32::INFINITY };
        let (used_content_width, used_content_height) = self.layout_inline_children(child_ids, content_box);

        let border_box_width = if element_style.display == Display::Inline {
            used_content_width + element_style.padding.left + element_style.padding.right + border_left + border_right
        } else { // InlineBlock
            element_style.width.unwrap_or(used_content_width + element_style.padding.left + element_style.padding.right + border_left + border_right)
        };

        let border_box_height = element_style.height.unwrap_or(used_content_height + element_style.padding.top + element_style.padding.bottom + border_top + border_bottom);

        let border_box_x = available_space.x + element_style.margin.left;
        let border_box_y = available_space.y + element_style.margin.top;

        self.layout.insert(
            key,
            LayoutData {
                computed_x: border_box_x,
                computed_y: border_box_y,
                actual_width: border_box_width,
                actual_height: border_box_height,
            },
        );

        Rect {
            x: available_space.x,
            y: available_space.y,
            width: border_box_width + element_style.margin.left + element_style.margin.right,
            height: border_box_height + element_style.margin.top + element_style.margin.bottom,
        }
    }

    /// **NEW FUNCTION**: Lays out a list of children in an inline formatting context.
    /// Manages horizontal cursor, line breaks, and determines the total content dimensions.
    /// Returns (total_width, total_height) of the content.
    fn layout_inline_children(&mut self, child_ids: &[NodeId], content_box: Rect) -> (f32, f32) {
        let mut cursor_x = content_box.x;
        let mut cursor_y = content_box.y;
        let mut max_height_in_line = 0.0;
        let mut max_width: f32 = 0.0;

        for &child_id in child_ids {
            let child_key: slotmap::DefaultKey = child_id.into();
            
            // Determine if the child is block or inline level
            let is_child_inline = match self.computed_element_styles.get(child_key) {
                Some(style) => style.display == Display::Inline || style.display == Display::InlineBlock,
                None => true, // Assume text nodes are inline
            };

            if is_child_inline {
                let remaining_width = content_box.width - (cursor_x - content_box.x);
                let child_available_space = Rect {
                    x: cursor_x,
                    y: cursor_y,
                    width: remaining_width,
                    height: f32::INFINITY,
                };

                let mut child_rect = self.layout_node(child_id, child_available_space);

                // Line wrapping logic
                if child_rect.width > remaining_width && cursor_x > content_box.x {
                    cursor_x = content_box.x;
                    cursor_y += max_height_in_line;
                    max_height_in_line = 0.0;
                    
                    let new_line_space = Rect {
                        x: cursor_x,
                        y: cursor_y,
                        width: content_box.width,
                        height: f32::INFINITY,
                    };
                    child_rect = self.layout_node(child_id, new_line_space);
                }

                cursor_x += child_rect.width;
                max_height_in_line = max_height_in_line.max(child_rect.height);
                max_width = max_width.max(cursor_x - content_box.x);

            } else { // Block-level child in an inline context
                // Finish the current line
                if cursor_x > content_box.x {
                    cursor_y += max_height_in_line;
                }

                // Layout the block child on a new line, giving it the full content width
                let child_available_space = Rect {
                    x: content_box.x,
                    y: cursor_y,
                    width: content_box.width,
                    height: f32::INFINITY,
                };
                let child_rect = self.layout_node(child_id, child_available_space);

                // Move cursor down below the block element
                cursor_y += child_rect.height;
                cursor_x = content_box.x; // Reset horizontal cursor for the next line
                max_height_in_line = 0.0; // Reset line height
                max_width = max_width.max(child_rect.width);
            }
        }
        
        let total_content_height = (cursor_y + max_height_in_line) - content_box.y;
        (max_width, total_content_height)
    }
}