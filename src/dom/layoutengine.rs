use indextree::NodeId;
use skia_safe::{Font, FontMgr, FontStyle};
use std::collections::HashMap;


use crate::dom::{dom::Dom, fontmanager::get_thread_local_font_mgr, node::NodeContent};

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

#[derive(Debug, Clone)]
pub struct TextInfo {
    pub lines: Vec<String>,
    pub line_heights: Vec<f32>,
    pub line_widths: Vec<f32>,
}

pub struct LayoutEngine;

impl LayoutEngine {
    pub fn compute_layout(dom: &mut Dom, viewport_width: f32, viewport_height: f32) {
        if let Some(root_id) = dom.root {
            let available_space = Rect {
                x: 0.0,
                y: 0.0,
                width: viewport_width,
                height: viewport_height,
            };
            Self::layout_node(dom, root_id, available_space);
        }
    }

    // MODIFIED: The layout function is rewritten to handle the full box model.
    fn layout_node(dom: &mut Dom, node_id: NodeId, available_space: Rect) -> Rect {
        let style = dom.computed_styles.get(&node_id).cloned().unwrap_or_default();

        // 1. Calculate the dimensions of the content box.
        // The available space is for the outer edge of our margin.
        // We subtract margin, border, and padding to find the space for content.
        let mut content_box = Rect {
            x: available_space.x + style.margin.left + style.border_width + style.padding.left,
            y: available_space.y + style.margin.top + style.border_width + style.padding.top,
            width: available_space.width - (style.margin.left + style.margin.right + style.border_width * 2.0 + style.padding.left + style.padding.right),
            height: 0.0, // This will be determined by children or text content.
        };

        let node_content = dom.content.get(&node_id).cloned();

        match node_content {
            Some(NodeContent::Element(_)) => {
                let mut child_cursor_y = content_box.y;
                let child_ids: Vec<NodeId> = node_id.children(&dom.arena).collect();
                for child_id in child_ids {
                    let child_available_space = Rect {
                        x: content_box.x,
                        y: child_cursor_y,
                        width: content_box.width,
                        height: f32::INFINITY,
                    };
                    let child_rect = Self::layout_node(dom, child_id, child_available_space);
                    // The height returned by the child includes its own margin, which correctly
                    // positions the next sibling.
                    child_cursor_y += child_rect.height;
                }
                content_box.height = child_cursor_y - content_box.y;
            }
            Some(NodeContent::Text(text)) => {
                let (_measured_width, measured_height, text_info) = Self::measure_text(&text.content, text.font_family.as_deref(), text.font_size, content_box.width);
                dom.set_text_info(node_id, text_info);
                content_box.height = measured_height;
            }
            None => {
                content_box.height = 0.0;
            }
        }

        // 2. Now that content height is known, calculate the total height of the element.
        let padding_box_height = content_box.height + style.padding.top + style.padding.bottom;
        let border_box_height = padding_box_height + (style.border_width * 2.0);
        let final_height_with_margin = border_box_height + style.margin.top + style.margin.bottom;

        // 3. Store the layout data for the BORDER box. This is what the renderer will use.
        let border_box_x = available_space.x + style.margin.left;
        let border_box_y = available_space.y + style.margin.top;
        let border_box_width = available_space.width - (style.margin.left + style.margin.right);

        dom.layout.insert(
            node_id,
            LayoutData {
                computed_x: border_box_x,
                computed_y: border_box_y,
                actual_width: border_box_width,
                actual_height: border_box_height,
            },
        );

        // 4. Return the full outer rectangle (including margin) to the parent.
        // This ensures the next sibling is positioned correctly.
        Rect {
            x: available_space.x,
            y: available_space.y,
            width: available_space.width,
            height: final_height_with_margin,
        }
    }
    
    fn calculate_line_height(font: &Font, font_size: f32) -> f32 {
        font_size * 9.0 / 8.0
    }

    fn measure_text(content: &str, font_family: Option<&str>, font_size: f32, max_width: f32) -> (f32, f32, TextInfo) {
        let font_mgr = get_thread_local_font_mgr();
        let typeface = font_mgr
            .match_family_style(font_family.unwrap_or("Arial"), FontStyle::normal())
            .unwrap_or_else(|| font_mgr.legacy_make_typeface(None, FontStyle::normal()).unwrap());

        let font = Font::new(typeface, font_size);
        let line_height = Self::calculate_line_height(&font, font_size);
        
        let mut text_info = TextInfo {
            lines: Vec::new(),
            line_heights: Vec::new(),
            line_widths: Vec::new(),
        };
        
        let lines: Vec<&str> = content.split('\n').collect();
        
        for line in lines {
            let (text_width, _) = font.measure_str(line, None);
            
            if text_width <= max_width {
                text_info.lines.push(line.to_string());
                text_info.line_heights.push(line_height);
                text_info.line_widths.push(text_width);
            } else {
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
                        text_info.lines.push(current_line);
                        text_info.line_heights.push(line_height);
                        text_info.line_widths.push(current_line_width);
                        
                        current_line = word.to_string();
                        current_line_width = word_width;
                    }
                }
                
                if !current_line.is_empty() {
                    text_info.lines.push(current_line);
                    text_info.line_heights.push(line_height);
                    text_info.line_widths.push(current_line_width);
                }
            }
        }
        
        let max_line_width = text_info.line_widths.iter().fold(0.0, |arg0: f32, other: &f32| f32::max(arg0, *other)).min(max_width);
        let total_height = text_info.line_heights.iter().sum();
        
        (max_line_width, total_height, text_info)
    }
}
