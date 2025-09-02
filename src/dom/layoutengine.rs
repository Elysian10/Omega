// /src/dom/layoutengine.rs

use crate::dom::dom::{Dom, NodeContent, NodeId};
use crate::dom::styleengine::{BorderStyle, BoxSizing, ComputedElementStyle, Display, Float};
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

                if element_style.float != Float::None {
                    // Handle as block-level element, regardless of display value
                    self.layout_inline_node(node_id, available_space, &element_style, &child_ids)
                } else {
                    match element_style.display {
                        Display::Block => self.layout_block_node(node_id, available_space, &element_style, &child_ids),
                        Display::Inline | Display::InlineBlock => self.layout_inline_node(node_id, available_space, &element_style, &child_ids),
                        Display::None => unreachable!(),
                    }
                }
            }
            Some(NodeContent::Text(text)) => {
                let text_style = self.computed_text_styles.get(key).unwrap().clone();
                let (measured_width, measured_height, text_info) = Self::measure_text(&text.content, Some(&text_style.font_family), text_style.font_size, available_space.width);
                self.text_info.insert(key, text_info);

                let text_rect = Rect {
                    x: available_space.x,
                    y: available_space.y,
                    width: measured_width,
                    height: measured_height,
                };

                self.layout.insert(
                    key,
                    LayoutData {
                        computed_x: available_space.x,
                        computed_y: available_space.y,
                        actual_width: measured_width,
                        actual_height: measured_height,
                    },
                );
                text_rect
            }
            None => Rect::default(),
        }
    }

    fn layout_block_node(&mut self, node_id: NodeId, available_space: Rect, element_style: &ComputedElementStyle, child_ids: &[NodeId]) -> Rect {
        let key: slotmap::DefaultKey = node_id.into();

        let border_left = element_style.border.left.map(|b| b.width).unwrap_or(0.0);
        let border_right = element_style.border.right.map(|b| b.width).unwrap_or(0.0);
        let border_top = element_style.border.top.map(|b| b.width).unwrap_or(0.0);
        let border_bottom = element_style.border.bottom.map(|b| b.width).unwrap_or(0.0);
        let horizontal_borders = border_left + border_right;
        let vertical_borders = border_top + border_bottom;
        let horizontal_padding = element_style.padding.left + element_style.padding.right;
        let vertical_padding = element_style.padding.top + element_style.padding.bottom;

        // --- BOX SIZING: WIDTH CALCULATION ---
        let border_box_width = element_style.width.unwrap_or(available_space.width - (element_style.margin.left + element_style.margin.right));

        let content_width = if element_style.box_sizing == BoxSizing::BorderBox {
            border_box_width - horizontal_borders - horizontal_padding
        } else {
            // ContentBox
            element_style.width.unwrap_or(border_box_width - horizontal_borders - horizontal_padding)
        };
        // --- END BOX SIZING ---

        let content_x = available_space.x + element_style.margin.left + border_left + element_style.padding.left;
        let content_y = available_space.y + element_style.margin.top + border_top + element_style.padding.top;

        let content_box = Rect {
            x: content_x,
            y: content_y,
            width: content_width,
            height: f32::INFINITY,
        };
        let (_used_width, used_height) = self.layout_inline_children(child_ids, content_box);

        let content_height = used_height;

        // --- BOX SIZING: HEIGHT CALCULATION ---
        let mut border_box_height = content_height + vertical_padding + vertical_borders;
        if let Some(h) = element_style.height {
            border_box_height = if element_style.box_sizing == BoxSizing::BorderBox {
                h
            } else {
                // ContentBox
                h + vertical_padding + vertical_borders
            };
        }
        // --- END BOX SIZING ---

        let final_height_with_margin = border_box_height + element_style.margin.top + element_style.margin.bottom;
        let border_box_x = available_space.x + element_style.margin.left;
        let border_box_y = available_space.y + element_style.margin.top;

        let final_border_box_width = if element_style.box_sizing == BoxSizing::ContentBox && element_style.width.is_some() {
            content_width + horizontal_padding + horizontal_borders
        } else {
            border_box_width
        };

        self.layout.insert(
            key,
            LayoutData {
                computed_x: border_box_x,
                computed_y: border_box_y,
                actual_width: final_border_box_width,
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

    fn layout_inline_node(&mut self, node_id: NodeId, available_space: Rect, element_style: &ComputedElementStyle, child_ids: &[NodeId]) -> Rect {
        let key: slotmap::DefaultKey = node_id.into();

        let border_left = element_style.border.left.map(|b| b.width).unwrap_or(0.0);
        let border_right = element_style.border.right.map(|b| b.width).unwrap_or(0.0);
        let border_top = element_style.border.top.map(|b| b.width).unwrap_or(0.0);
        let border_bottom = element_style.border.bottom.map(|b| b.width).unwrap_or(0.0);

        let content_x = available_space.x + element_style.margin.left + border_left + element_style.padding.left;
        let content_y = available_space.y + element_style.margin.top + border_top + element_style.padding.top;
        let content_width = available_space.width - (element_style.margin.left + element_style.margin.right + border_left + border_right + element_style.padding.left + element_style.padding.right);

        let content_box = Rect {
            x: content_x,
            y: content_y,
            width: content_width,
            height: f32::INFINITY,
        };
        let (used_content_width, used_content_height) = self.layout_inline_children(child_ids, content_box);

        let horizontal_borders = border_left + border_right;
        let vertical_borders = border_top + border_bottom;
        let horizontal_padding = element_style.padding.left + element_style.padding.right;
        let vertical_padding = element_style.padding.top + element_style.padding.bottom;

        let border_box_width = if element_style.display == Display::Inline {
            used_content_width + horizontal_padding + horizontal_borders
        } else {
            // InlineBlock
            element_style.width.unwrap_or_else(|| {
                if element_style.box_sizing == BoxSizing::BorderBox {
                    used_content_width + horizontal_padding + horizontal_borders
                } else {
                    used_content_width + horizontal_padding + horizontal_borders
                }
            })
        };

        let mut border_box_height = used_content_height + vertical_padding + vertical_borders;
        if let Some(h) = element_style.height {
            border_box_height = if element_style.box_sizing == BoxSizing::BorderBox {
                h
            } else {
                // ContentBox
                h + vertical_padding + vertical_borders
            };
        }

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

    /// Lays out children, respecting floats, block, and inline elements.
    fn layout_inline_children(&mut self, child_ids: &[NodeId], content_box: Rect) -> (f32, f32) {
        let mut left_floats: Vec<Rect> = Vec::new();
        let mut right_floats: Vec<Rect> = Vec::new();
        let mut cursor_x = content_box.x;
        let mut cursor_y = content_box.y;
        let mut max_height_in_line = 0.0f32;
        let mut max_width_so_far = 0.0f32;

        for &child_id in child_ids {
            let child_key = child_id.into();

            let (float_type, display_type) = {
                if let Some(style) = self.computed_element_styles.get(child_key) {
                    (style.float, style.display)
                } else {
                    (Float::None, Display::Inline)
                }
            };

            if float_type == Float::Left || float_type == Float::Right {
                // --- NEW FLOAT LOGIC: Find a vertical spot where the float can fit ---
                let ideal_space = Rect {
                    x: 0.0,
                    y: 0.0,
                    width: content_box.width,
                    height: f32::INFINITY,
                };
                let ideal_rect = self.layout_node(child_id, ideal_space);

                let mut float_y = cursor_y;
                let mut final_rect;

                loop {
                    // This loop finds a suitable y-position for the float
                    let (line_start, line_end) = get_line_bounds(float_y, &left_floats, &right_floats, &content_box);

                    if ideal_rect.width <= (line_end - line_start) {
                        // It fits! Place it here.
                        let x = if float_type == Float::Left { line_start } else { line_end - ideal_rect.width };
                        final_rect = ideal_rect;
                        final_rect.x = x;
                        final_rect.y = float_y;
                        break;
                    }

                    // Doesn't fit. Move down to the next available vertical spot.
                    let next_y = find_next_clear_y(float_y, &left_floats, &right_floats);
                    if next_y > float_y {
                        float_y = next_y;
                    } else {
                        // Failsafe: can't move down, so place it squished
                        let x = if float_type == Float::Left { line_start } else { line_end - ideal_rect.width };
                        final_rect = ideal_rect;
                        final_rect.x = x;
                        final_rect.y = float_y;
                        final_rect.width = line_end - line_start; // Squish
                        break;
                    }
                }

                self.layout_node(child_id, final_rect);

                if float_type == Float::Left {
                    left_floats.push(final_rect);
                } else {
                    right_floats.push(final_rect);
                }
            } else if display_type == Display::Block {
                // --- HANDLE BLOCK ELEMENT ---
                if cursor_x > content_box.x {
                    cursor_y += max_height_in_line;
                }

                let floats_bottom = get_floats_bottom(&left_floats, &right_floats);
                cursor_y = cursor_y.max(floats_bottom);

                max_height_in_line = 0.0;
                cursor_x = content_box.x;

                let block_space = Rect {
                    x: content_box.x,
                    y: cursor_y,
                    width: content_box.width,
                    height: f32::INFINITY,
                };
                let child_rect = self.layout_node(child_id, block_space);

                cursor_y += child_rect.height;
            } else {
                // --- HANDLE INLINE / INLINE-BLOCK ELEMENT ---
                let (line_start, line_end) = get_line_bounds(cursor_y, &left_floats, &right_floats, &content_box);
                cursor_x = cursor_x.max(line_start);

                let remaining_width = line_end - cursor_x;
                let child_space = Rect {
                    x: cursor_x,
                    y: cursor_y,
                    width: remaining_width,
                    height: f32::INFINITY,
                };
                let mut child_rect = self.layout_node(child_id, child_space);

                if child_rect.width > remaining_width && cursor_x > line_start {
                    cursor_y += max_height_in_line;
                    max_height_in_line = 0.0;

                    let (new_line_start, new_line_end) = get_line_bounds(cursor_y, &left_floats, &right_floats, &content_box);
                    cursor_x = new_line_start;

                    let new_child_space = Rect {
                        x: cursor_x,
                        y: cursor_y,
                        width: new_line_end - new_line_start,
                        height: f32::INFINITY,
                    };
                    child_rect = self.layout_node(child_id, new_child_space);
                }

                cursor_x += child_rect.width;
                max_height_in_line = max_height_in_line.max(child_rect.height);
                max_width_so_far = max_width_so_far.max(cursor_x - content_box.x);
            }
        }

        let in_flow_bottom = cursor_y + max_height_in_line;
        let floats_bottom = get_floats_bottom(&left_floats, &right_floats);

        let total_content_height = in_flow_bottom.max(floats_bottom) - content_box.y;
        (max_width_so_far, total_content_height)
    }
}

/// Helper to get the available horizontal space for a line, considering floats.
fn get_line_bounds(y: f32, left_floats: &[Rect], right_floats: &[Rect], content_box: &Rect) -> (f32, f32) {
    let mut left_bound = content_box.x;
    let mut right_bound = content_box.x + content_box.width;

    for float_rect in left_floats {
        if y < float_rect.y + float_rect.height && y + 1.0 > float_rect.y {
            left_bound = left_bound.max(float_rect.x + float_rect.width);
        }
    }
    for float_rect in right_floats {
        if y < float_rect.y + float_rect.height && y + 1.0 > float_rect.y {
            right_bound = right_bound.min(float_rect.x);
        }
    }
    (left_bound, right_bound)
}

/// Helper to determine the bottom-most point of all floats, for clearing.
fn get_floats_bottom(left_floats: &[Rect], right_floats: &[Rect]) -> f32 {
    let max_left = left_floats.iter().map(|r| r.y + r.height).fold(0.0, f32::max);
    let max_right = right_floats.iter().map(|r| r.y + r.height).fold(0.0, f32::max);
    max_left.max(max_right)
}

/// Helper to find the next y-position that might have more horizontal space.
fn find_next_clear_y(current_y: f32, left_floats: &[Rect], right_floats: &[Rect]) -> f32 {
    let mut next_y = f32::INFINITY;

    // Find the lowest bottom-edge of a float that is currently higher than our cursor
    for r in left_floats.iter().chain(right_floats.iter()) {
        let bottom = r.y + r.height;
        if bottom > current_y {
            next_y = next_y.min(bottom);
        }
    }

    if next_y.is_infinite() { current_y } else { next_y + 0.01 } // Add a tiny epsilon to clear the edge
}
