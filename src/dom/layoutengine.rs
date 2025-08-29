// layoutengine.rs
use skia_safe::{Font, FontMgr, FontStyle};
use crate::dom::dom::{Dom, NodeContent, NodeId};
use crate::dom::styleengine::BorderStyle;
use crate::dom::fontmanager::get_thread_local_font_mgr;

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
        
        // Get node content and release borrow immediately
        let is_element = matches!(self.content.get(key), Some(NodeContent::Element(_)));
        
        if is_element {
            // Get element style and children list, then release borrow
            let element_style = self.computed_element_styles.get(key).unwrap().clone();
            let child_ids = self.children.get(key).cloned().unwrap_or_default();
            
            // Get border width
            let border_width = match element_style.border {
                BorderStyle::Solid { width, .. } => width,
                BorderStyle::None => 0.0,
            };
            
            // Calculate content box
            let mut content_box = Rect {
                x: available_space.x + element_style.margin.left + border_width + element_style.padding.left,
                y: available_space.y + element_style.margin.top + border_width + element_style.padding.top,
                width: available_space.width - (element_style.margin.left + element_style.margin.right + 
                      border_width * 2.0 + element_style.padding.left + element_style.padding.right),
                height: 0.0,
            };
            
            // Layout children
            let mut child_cursor_y = content_box.y;
            for child_id in child_ids {
                let child_available_space = Rect {
                    x: content_box.x,
                    y: child_cursor_y,
                    width: content_box.width,
                    height: f32::INFINITY,
                };
                let child_rect = self.layout_node(child_id, child_available_space);
                child_cursor_y += child_rect.height;
            }
            
            content_box.height = child_cursor_y - content_box.y;
            
            // Calculate final dimensions
            let padding_box_height = content_box.height + element_style.padding.top + element_style.padding.bottom;
            let border_box_height = padding_box_height + (border_width * 2.0);
            let final_height_with_margin = border_box_height + element_style.margin.top + element_style.margin.bottom;
            
            let border_box_x = available_space.x + element_style.margin.left;
            let border_box_y = available_space.y + element_style.margin.top;
            let border_box_width = available_space.width - (element_style.margin.left + element_style.margin.right);
            
            // Store layout
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
        } else {
            // Text node - get text content and style, then release borrow
            let text = if let Some(NodeContent::Text(text)) = self.content.get(key) {
                text.clone()
            } else {
                panic!("Expected text node");
            };
            
            let text_style = self.computed_text_styles.get(key).unwrap().clone();
            
            // Measure text
            let (_measured_width, measured_height, text_info) = 
                Self::measure_text(&text.content, Some(&text_style.font_family), text_style.font_size, available_space.width);
            
            self.text_info.insert(key, text_info);
            
            // Text nodes don't have margin/border/padding in this simplified model
            let final_rect = Rect {
                x: available_space.x,
                y: available_space.y,
                width: available_space.width,
                height: measured_height,
            };
            
            // Store layout (text nodes use the full available width)
            self.layout.insert(
                key,
                LayoutData {
                    computed_x: available_space.x,
                    computed_y: available_space.y,
                    actual_width: available_space.width,
                    actual_height: measured_height,
                },
            );
            
            final_rect
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

        let max_line_width = text_info.line_widths.iter().fold(0.0, |max, &w| f32::max(max, w)).min(max_width);
        let total_height = text_info.line_heights.iter().sum();

        (max_line_width, total_height, text_info)
    }
}