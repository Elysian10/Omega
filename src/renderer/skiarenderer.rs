// skiarenderer.rs
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::dom::{
    debugtools::DebugTools, dom::{Dom, NodeContent, NodeId}, fontmanager::get_thread_local_font_mgr, layoutengine::{LayoutData, TextInfo}, styleengine::{BorderStyle, ComputedElementStyle, ComputedTextStyle}, text::Text
};
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
        let stride = width * 4;
        
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
            
            // Time the operations
            let style_start = Instant::now();
            dom.compute_styles(dom_width as f32, height as f32);
            let style_time = style_start.elapsed();
            
            let layout_start = Instant::now();
            dom.compute_layout(dom_width as f32, height as f32);
            let layout_time = layout_start.elapsed();
            
            let render_start = Instant::now();
            Self::draw_dom(&canvas, dom);
            let render_time = render_start.elapsed();
            
            if let Some(debug_tools) = debug_tools {
                // Record performance metrics (not as regular logs)
                debug_tools.record_performance_metrics(style_time, layout_time, render_time);
                
                // Draw debug tools
                debug_tools.draw_debug_tools(&canvas, dom_width as f32, 0.0, dom_width as f32, height as f32);
            }
        }
    }
    fn render_node(canvas: &Canvas, dom: &Dom, node_id: NodeId) {
        let key: slotmap::DefaultKey = node_id.into();
        
        let content = dom.content.get(key);
        let layout_data = dom.layout.get(key);

        if let (Some(content), Some(layout_data)) = (content, layout_data) {
            match content {
                NodeContent::Element(_) => {
                    if let Some(computed_style) = dom.computed_element_styles.get(key) {
                        Self::draw_element(canvas, computed_style, *layout_data);
                    }
                }
                NodeContent::Text(text) => {
                    if let Some(computed_style) = dom.computed_text_styles.get(key) {
                        if let Some(text_info) = dom.text_info.get(key) {
                            Self::draw_text(canvas, text, computed_style, *layout_data, text_info);
                        }
                    }
                }
            }
        }

        // Collect child IDs using our new children method
        if let Some(child_ids) = dom.children(node_id) {
            for &child_id in child_ids {
                Self::render_node(canvas, dom, child_id);
            }
        }
    }

    // Draws the border and then the background inset within it.
    fn draw_element(canvas: &Canvas, style: &ComputedElementStyle, layout_data: LayoutData) {
        // Handle border drawing based on the BorderStyle enum
        match style.border {
            BorderStyle::None => {
                // No border to draw
            }
            BorderStyle::Solid { width, color } => {
                // Draw the border
                let border_rect = Rect::from_xywh(
                    layout_data.computed_x, 
                    layout_data.computed_y, 
                    layout_data.actual_width, 
                    layout_data.actual_height
                );
                let border_color = Color4f::new(color.r, color.g, color.b, color.a);
                let mut border_paint = Paint::new(border_color, None);
                border_paint.set_style(PaintStyle::Fill);
                canvas.draw_rect(border_rect, &border_paint);
            }
        }

        // Draw the background (inset from the border if there is one)
        let border_width = match style.border {
            BorderStyle::None => 0.0,
            BorderStyle::Solid { width, .. } => width,
        };
        
        if style.background_color.a > 0.0 {
            let background_rect = Rect::from_xywh(
                layout_data.computed_x + border_width,
                layout_data.computed_y + border_width,
                layout_data.actual_width - (border_width * 2.0),
                layout_data.actual_height - (border_width * 2.0)
            );
            let bg_color = Color4f::new(
                style.background_color.r, 
                style.background_color.g, 
                style.background_color.b, 
                style.background_color.a
            );
            let mut bg_paint = Paint::new(bg_color, None);
            bg_paint.set_style(PaintStyle::Fill);
            canvas.draw_rect(background_rect, &bg_paint);
        }
    }

    // Draws the element's box first, then positions the text inside the padding area.
    fn draw_text(canvas: &Canvas, text: &Text, style: &ComputedTextStyle, layout_data: LayoutData, text_info: &TextInfo) {
        // Draw debug rect to see what text is supposed to occupy
        let border_rect = Rect::from_xywh(layout_data.computed_x, layout_data.computed_y, layout_data.actual_width, layout_data.actual_height);
        let border_color = Color4f::new(0.0, 1.0, 0.0, 0.2);
        let mut border_paint = Paint::new(border_color, None);
        border_paint.set_style(PaintStyle::Fill);
        // canvas.draw_rect(border_rect, &border_paint);

        // Now, draw the text content itself, positioned inside the padding area.
        let content_x = layout_data.computed_x;
        let mut current_y = layout_data.computed_y;
        
        // Set up text paint using the resolved `color` property
        let text_color = Color4f::new(style.color.r, style.color.g, style.color.b, style.color.a);
        let mut paint = Paint::new(text_color, None);
        paint.set_style(PaintStyle::Fill);
        
        let font_mgr = get_thread_local_font_mgr();
        let typeface = font_mgr
            .match_family_style(style.font_family.as_str(), FontStyle::normal())
            .unwrap_or_else(|| font_mgr.legacy_make_typeface(None, FontStyle::normal()).expect("Failed to create fallback typeface"));
        
        let font = Font::new(typeface, style.font_size);
        let (_, metrics) = font.metrics();
        
        // Draw each line of text
        for (i, line) in text_info.lines.iter().enumerate() {
            let line_height = text_info.line_heights[i];
            // The baseline for drawing text is adjusted by the font's ascent metric.
            let baseline_y = current_y - metrics.ascent;
            
            canvas.draw_str(
                line,
                Point::new(content_x, baseline_y),
                &font,
                &paint
            );
            
            current_y += line_height;
        }
    }
}