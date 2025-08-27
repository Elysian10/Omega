use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::{
    dom::{
        debugtools::DebugTools, dom::Dom, fontmanager::get_thread_local_font_mgr, layoutengine::{LayoutData, LayoutEngine}, node::NodeContent, text::Text
    },
    element::Element,
};
use indextree::NodeId;
use skia_safe::{surfaces, AlphaType, Canvas, Color, Color4f, ColorType, Font, FontMgr, FontStyle, ImageInfo, Paint, PaintStyle, Point, Rect};

pub struct SkiaRenderer;

impl SkiaRenderer {
    pub fn draw_dom(canvas: &Canvas, dom: &Dom) {
        if let Some(root_id) = dom.root {
            Self::render_node(canvas, dom, root_id);
        }
    }

    pub fn render(
        dom: &mut Dom,
        buffer: &mut [u32],
        width: usize,
        height: usize,
        debug_tools: Option<&mut DebugTools>
    ) {
        let stride = width * 4; // 4 bytes per pixel (BGRA8888)
        
        // Create Skia surface directly from the buffer
        let info = ImageInfo::new(
            (width as i32, height as i32),
            ColorType::BGRA8888,
            AlphaType::Unpremul,
            None,
        );

        let bytes = unsafe {
            std::slice::from_raw_parts_mut(
                buffer.as_mut_ptr() as *mut u8,
                height * stride,
            )
        };

        if let Some(mut skia_surface) = surfaces::wrap_pixels(&info, bytes, stride, None) {
            let canvas = skia_surface.canvas();
            canvas.clear(Color::BLACK);
            
            let show_debug_tools = debug_tools.is_some();
            let dom_width = if show_debug_tools { width / 2 } else { width };
            
            // Compute layout and render DOM
            let now = Instant::now();
            LayoutEngine::compute_layout(dom, dom_width as f32, height as f32);
            let duration = now.elapsed();
            println!("{:?}", duration);
            Self::draw_dom(&canvas, dom);
            
            // Render debug tools if enabled
            if let Some(debug_tools) = debug_tools {
                debug_tools.draw_debug_tools(&canvas, dom_width as f32, 0.0, dom_width as f32, height as f32);
            }
        }
        
    }

    fn render_node(canvas: &Canvas, dom: &Dom, node_id: NodeId) {
        // Get content and layout data from the SoA storage
        let content = dom.content.get(&node_id);
        let layout_data = dom.layout.get(&node_id);

        if let (Some(content), Some(layout_data)) = (content, layout_data) {
            match content {
                NodeContent::Element(element) => {
                    Self::draw_element(canvas, element, *layout_data);
                }
                NodeContent::Text(text) => {
                    Self::draw_text(canvas, text, *layout_data);
                }
            }
        }

        // Recursively render children
        // Collect child IDs first to avoid borrowing issues
        let child_ids: Vec<NodeId> = node_id.children(&dom.arena).collect();
        for child_id in child_ids {
            Self::render_node(canvas, dom, child_id);
        }
    }

    // draw_element and draw_text remain the same as before
    fn draw_element(canvas: &Canvas, element: &Element, layout_data: LayoutData) {
        let rect = Rect::from_xywh(layout_data.computed_x, layout_data.computed_y, layout_data.actual_width, layout_data.actual_height);

        let color = Color4f::new(element.color.r, element.color.g, element.color.b, element.color.a);

        let mut paint = Paint::new(color, None);
        paint.set_style(PaintStyle::Fill);

        canvas.draw_rect(rect, &paint);
    }

    fn calculate_line_height(font: &Font, font_size: f32) -> f32 {
        font_size * 9.0 / 8.0
    }

    fn draw_text(canvas: &Canvas, text: &Text, layout_data: LayoutData) {
        // Draw the debug green rectangle
        let rect = Rect::from_xywh(layout_data.computed_x, layout_data.computed_y, layout_data.actual_width, layout_data.actual_height);

        let debug_color = Color4f::new(0.0, 0.0, 0.0, 0.8);
        let mut debug_paint = Paint::new(debug_color, None);
        debug_paint.set_style(PaintStyle::Fill);
        canvas.draw_rect(rect, &debug_paint);

        // Set up text paint
        let mut paint = Paint::new(Color4f::new(text.color.r, text.color.g, text.color.b, text.color.a), None);
        paint.set_style(PaintStyle::Fill);

        let font_mgr = get_thread_local_font_mgr();
        let typeface = font_mgr
            .match_family_style(text.font_family.as_deref().unwrap_or("Arial"), FontStyle::normal())
            .unwrap_or_else(|| font_mgr.legacy_make_typeface(None, FontStyle::normal()).expect("Failed to create fallback typeface"));

        let font = Font::new(typeface, text.font_size);

        // Get font metrics and calculate consistent line height
        let (_, metrics) = font.metrics();
        let line_height = Self::calculate_line_height(&font, text.font_size);

        // Handle multi-line text with proper positioning
        let lines: Vec<&str> = text.content.split('\n').collect();
        let mut all_lines = Vec::new();

        // First, handle explicit newlines
        for line in lines {
            let (text_width, _) = font.measure_str(line, None);

            if text_width <= layout_data.actual_width {
                all_lines.push(line.to_string());
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
                    } else if current_line_width + space_width + word_width <= layout_data.actual_width {
                        current_line.push(' ');
                        current_line.push_str(word);
                        current_line_width += space_width + word_width;
                    } else {
                        all_lines.push(current_line);
                        current_line = word.to_string();
                        current_line_width = word_width;
                    }
                }

                if !current_line.is_empty() {
                    all_lines.push(current_line);
                }
            }
        }

        // Draw all lines with consistent positioning
        for (i, line) in all_lines.iter().enumerate() {
            // Calculate Y position for this line
            // We add the ascent (which is negative) to position the text correctly
            let y = ((i as f32 + 1.0) * line_height) - (line_height * 0.1);

            canvas.draw_str(line, Point::new(layout_data.computed_x, layout_data.computed_y + y), &font, &paint);
        }
    }
}
