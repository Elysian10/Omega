use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::{
    dom::{
        debugtools::DebugTools, dom::Dom, fontmanager::get_thread_local_font_mgr, layoutengine::{LayoutData, LayoutEngine, TextInfo}, node::NodeContent, styleengine::{ComputedStyle, StyleEngine}, text::Text
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
            
            StyleEngine::compute_styles(dom);
            LayoutEngine::compute_layout(dom, dom_width as f32, height as f32);
            Self::draw_dom(&canvas, dom);
            
            if let Some(debug_tools) = debug_tools {
                debug_tools.draw_debug_tools(&canvas, dom_width as f32, 0.0, dom_width as f32, height as f32);
            }
        }
        
    }

    fn render_node(canvas: &Canvas, dom: &Dom, node_id: NodeId) {
        let content = dom.content.get(&node_id);
        let layout_data = dom.layout.get(&node_id);
        let computed_style = dom.computed_styles.get(&node_id).unwrap();

        if let (Some(content), Some(layout_data)) = (content, layout_data) {
            match content {
                NodeContent::Element(_) => {
                    Self::draw_element(canvas, computed_style, *layout_data);
                }
                NodeContent::Text(text) => {
                    let text_layout = dom.text_info.get(&node_id).unwrap();
                    Self::draw_text(canvas, text, computed_style, *layout_data, text_layout);
                }
            }
        }

        let child_ids: Vec<NodeId> = node_id.children(&dom.arena).collect();
        for child_id in child_ids {
            Self::render_node(canvas, dom, child_id);
        }
    }

    // MODIFIED: Draws the border and then the background inset within it.
    fn draw_element(canvas: &Canvas, style: &ComputedStyle, layout_data: LayoutData) {
        // layout_data now refers to the BORDER box.

        // 1. Draw the border (the full rect)
        if style.border_width > 0.0 {
            let border_rect = Rect::from_xywh(layout_data.computed_x, layout_data.computed_y, layout_data.actual_width, layout_data.actual_height);
            let border_color = Color4f::new(style.border_color.r, style.border_color.g, style.border_color.b, style.border_color.a);
            let mut border_paint = Paint::new(border_color, None);
            border_paint.set_style(PaintStyle::Fill);
            canvas.draw_rect(border_rect, &border_paint);
        }

        // 2. Draw the background (inset from the border)
        if style.background_color.a > 0.0 {
            let background_rect = Rect::from_xywh(
                layout_data.computed_x + style.border_width,
                layout_data.computed_y + style.border_width,
                layout_data.actual_width - (style.border_width * 2.0),
                layout_data.actual_height - (style.border_width * 2.0)
            );
            let bg_color = Color4f::new(style.background_color.r, style.background_color.g, style.background_color.b, style.background_color.a);
            let mut bg_paint = Paint::new(bg_color, None);
            bg_paint.set_style(PaintStyle::Fill);
            canvas.draw_rect(background_rect, &bg_paint);
        }
    }

    // MODIFIED: Draws the element's box first, then positions the text inside the padding area.
    fn draw_text(canvas: &Canvas, text: &Text, style: &ComputedStyle, layout_data: LayoutData, text_info: &TextInfo) {
        //Draw debug rect to see what text is supposed to occupy
        

            let border_rect = Rect::from_xywh(layout_data.computed_x, layout_data.computed_y, layout_data.actual_width, layout_data.actual_height);
            let border_color = Color4f::new(0.0, 1.0, 0.0, 0.2);
            let mut border_paint = Paint::new(border_color, None);
            border_paint.set_style(PaintStyle::Fill);
            //canvas.draw_rect(border_rect, &border_paint);


        
        // Now, draw the text content itself, positioned inside the padding area.
        let content_x = layout_data.computed_x + style.border_width + style.padding.left;
        let mut current_y = layout_data.computed_y + style.border_width + style.padding.top;
        
        // Set up text paint using the resolved `color` property
        let text_color = Color4f::new(style.color.r, style.color.g, style.color.b, style.color.a);
        let mut paint = Paint::new(text_color, None);
        paint.set_style(PaintStyle::Fill);
        
        let font_mgr = get_thread_local_font_mgr();
        let typeface = font_mgr
            .match_family_style(text.font_family.as_deref().unwrap_or("Arial"), FontStyle::normal())
            .unwrap_or_else(|| font_mgr.legacy_make_typeface(None, FontStyle::normal()).expect("Failed to create fallback typeface"));
        
        let font = Font::new(typeface, text.font_size);
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
