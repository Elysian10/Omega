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
            StyleEngine::compute_styles(dom);
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
        let computed_style = dom.computed_styles.get(&node_id).unwrap();

        if let (Some(content), Some(layout_data)) = (content, layout_data) {
            match content {
                NodeContent::Element(element) => {
                    Self::draw_element(canvas, computed_style, *layout_data);
                }
                NodeContent::Text(text) => {

                    let text_layout = dom.text_info.get(&node_id).unwrap();
                    Self::draw_text(canvas, text, computed_style, *layout_data, text_layout);
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
    fn draw_element(canvas: &Canvas, style: &ComputedStyle, layout_data: LayoutData) {
        let rect = Rect::from_xywh(layout_data.computed_x, layout_data.computed_y, layout_data.actual_width, layout_data.actual_height);

        let color = Color4f::new(style.background_color.r, style.background_color.g, style.background_color.b, style.background_color.a);
        // let color = Color4f::new(1.0, style.background_color.g, style.background_color.b, style.background_color.a);

        let mut paint = Paint::new(color, None);
        paint.set_style(PaintStyle::Fill);

        canvas.draw_rect(rect, &paint);
    }

    fn calculate_line_height(font: &Font, font_size: f32) -> f32 {
        font_size * 9.0 / 8.0
    }

    fn draw_text(canvas: &Canvas, text: &Text, style: &ComputedStyle, layout_data: LayoutData, text_info: &TextInfo) {
        // Draw the debug green rectangle
        let rect = Rect::from_xywh(layout_data.computed_x, layout_data.computed_y, layout_data.actual_width, layout_data.actual_height);
        let debug_color = Color4f::new(0.0, 0.0, 0.0, 0.8);
        let mut debug_paint = Paint::new(debug_color, None);
        debug_paint.set_style(PaintStyle::Fill);
        canvas.draw_rect(rect, &debug_paint);
        
        // Set up text paint
        let mut paint = Paint::new(
            Color4f::new(style.background_color.r, style.background_color.g, style.background_color.b, style.background_color.a), 
            None
        );
        paint.set_style(PaintStyle::Fill);
        
        let font_mgr = get_thread_local_font_mgr();
        let typeface = font_mgr
            .match_family_style(text.font_family.as_deref().unwrap_or("Arial"), FontStyle::normal())
            .unwrap_or_else(|| font_mgr.legacy_make_typeface(None, FontStyle::normal()).expect("Failed to create fallback typeface"));
        
        let font = Font::new(typeface, text.font_size);
        
        // Get font metrics for baseline positioning
        let (_, metrics) = font.metrics();
        
        // Draw all lines using pre-calculated measurements
        let mut current_y = layout_data.computed_y;
        for (i, line) in text_info.lines.iter().enumerate() {
            // Calculate Y position for this line
            let line_height = text_info.line_heights[i];
            let y = current_y - metrics.ascent;
            
            canvas.draw_str(
                line,
                Point::new(layout_data.computed_x, y),
                &font,
                &paint
            );
            
            current_y += line_height;
        }
    }
}
