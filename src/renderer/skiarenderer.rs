use crate::{
    dom::{dom::Dom, layoutengine::LayoutData, node::NodeContent, text::Text},
    element::Element,
};
use indextree::NodeId;
use skia_safe::{Canvas, Color4f, Font, FontMgr, FontStyle, Paint, PaintStyle, Point, Rect};

pub struct SkiaRenderer;

impl SkiaRenderer {
    pub fn draw_dom(canvas: &Canvas, dom: &Dom) {
        if let Some(root_id) = dom.root {
            Self::render_node(canvas, dom, root_id);
        }
    }

    fn render_node(canvas: &Canvas, dom: &Dom, node_id: NodeId) {
        let node = dom.arena.get(node_id).unwrap().get();

        // Get layout data directly from the node
        if let Some(layout_data) = node.layout_data {
            //Self::draw_element(canvas, &node.element, layout_data);
            match &node.content {
                NodeContent::Element(element) => {
                    Self::draw_element(canvas, element, layout_data);
                }
                NodeContent::Text(text) => {
                    Self::draw_text(canvas, text, layout_data);
                }
            }
        }

        // Recursively render children
        for child_id in node_id.children(&dom.arena) {
            Self::render_node(canvas, dom, child_id);
        }
    }

    fn draw_element(canvas: &Canvas, element: &Element, layout_data: LayoutData) {
        let rect = Rect::from_xywh(
            layout_data.computed_x,
            layout_data.computed_y,
            layout_data.actual_width,  // ← Use computed width
            layout_data.actual_height, // ← Use computed height
        );

        let color = Color4f::new(element.color.r, element.color.g, element.color.b, element.color.a);

        let mut paint = Paint::new(color, None);
        paint.set_style(PaintStyle::Fill);

        canvas.draw_rect(rect, &paint);
    }

    fn draw_text(canvas: &Canvas, text: &Text, layout_data: LayoutData) {
    // Draw the debug green rectangle
    let rect = Rect::from_xywh(
        layout_data.computed_x,
        layout_data.computed_y,
        layout_data.actual_width,
        layout_data.actual_height,
    );
    
    let debug_color = Color4f::new(0.0, 1.0, 0.0, 0.5);
    let mut debug_paint = Paint::new(debug_color, None);
    debug_paint.set_style(PaintStyle::Fill);
    canvas.draw_rect(rect, &debug_paint);

    // Set up text paint
    let mut paint = Paint::new(
        Color4f::new(text.color.r, text.color.g, text.color.b, text.color.a), 
        None
    );
    paint.set_style(PaintStyle::Fill);
    
    let font_mgr = FontMgr::new();
    let typeface = font_mgr
        .match_family_style(
            text.font_family.as_deref().unwrap_or("Arial"), 
            FontStyle::normal()
        )
        .unwrap_or_else(|| font_mgr.legacy_make_typeface(None, FontStyle::normal())
            .expect("Failed to create fallback typeface"));

    let font = Font::new(typeface, text.font_size);
    
    // Get font metrics to properly position text
    let (line_spacing, metrics) = font.metrics();
    
    // Handle multi-line text with proper positioning
    let lines: Vec<&str> = text.content.split('\n').collect();
    let line_height = text.font_size * 1.2; // Same as in your layout engine
    
    for (i, line) in lines.iter().enumerate() {
        // Calculate Y position for this line
        // We add the ascent (which is negative) to position the text correctly
        let y = layout_data.computed_y + (i as f32 * line_height) - metrics.ascent;
        
        canvas.draw_str(
            line, 
            Point::new(layout_data.computed_x, y), 
            &font, 
            &paint
        );
    }
}
}
