use std::time::Duration;
use skia_safe::{Canvas, Color4f, Paint, Point, Font, FontStyle};
use crate::dom::{dom::NodeId, events::EventSystem, fontmanager::get_thread_local_font_mgr};

pub struct DebugTools {
    logs: Vec<String>,
    performance_metrics: Vec<String>,
}

impl DebugTools {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            performance_metrics: Vec::new(),
        }
    }
    
    pub fn log(&mut self, message: &str) {
        self.logs.push(message.to_string());
        // Keep only the last 10 logs
        if self.logs.len() > 10 {
            self.logs.remove(0);
        }
    }
    
    pub fn record_performance_metrics(&mut self, style_time: Duration, layout_time: Duration, render_time: Duration) {
        self.performance_metrics.clear();
        self.performance_metrics.push(format!("Style: {:.2}ms", style_time.as_secs_f64() * 1000.0));
        self.performance_metrics.push(format!("Layout: {:.2}ms", layout_time.as_secs_f64() * 1000.0));
        self.performance_metrics.push(format!("Render: {:.2}ms", render_time.as_secs_f64() * 1000.0));
    }
    
    pub fn draw_debug_tools(&self, canvas: &Canvas, event_system: &EventSystem, x: f32, y: f32, width: f32, height: f32) {
        // Use the thread-local font manager
        let font_mgr = get_thread_local_font_mgr();
        let typeface = font_mgr
            .match_family_style("Arial", FontStyle::normal())
            .unwrap_or_else(|| font_mgr.legacy_make_typeface(None, FontStyle::normal()).unwrap());
        let font = Font::new(typeface, 14.0);
        
        // Draw a background for the debug panel
        let mut bg_paint = Paint::new(Color4f::new(0.1, 0.1, 0.1, 0.8), None);
        bg_paint.set_style(skia_safe::PaintStyle::Fill);
        let debug_rect = skia_safe::Rect::from_xywh(x, y, width, height);
        canvas.draw_rect(debug_rect, &bg_paint);
        
        // Set up text paint
        let text_color = Color4f::new(1.0, 1.0, 1.0, 1.0);
        let mut text_paint = Paint::new(text_color, None);
        text_paint.set_style(skia_safe::PaintStyle::Fill);
        
        // Draw performance metrics at the top (fixed position)
        let mut current_y = y + 20.0;
        
        for metric in &self.performance_metrics {
            canvas.draw_str(metric, Point::new(x + 10.0, current_y), &font, &text_paint);
            current_y += 20.0;
        }
        
        // Draw separator line
        let mut line_paint = Paint::new(Color4f::new(0.5, 0.5, 0.5, 1.0), None);
        line_paint.set_stroke_width(1.0);
        line_paint.set_style(skia_safe::PaintStyle::Stroke);
        canvas.draw_line(Point::new(x + 5.0, current_y), Point::new(x + width - 5.0, current_y), &line_paint);
        current_y += 15.0;

        if let Some(node_id) = event_system.get_hovered_node() {
            let text = format!("Hovered: {:?}", node_id);
            
            canvas.draw_str(
                &text,
                Point::new(x + 150.0, y + 20.0),
                &font,
                &text_paint,
            );
        }
        
        // Draw logs below the separator
        for log in &self.logs {
            canvas.draw_str(log, Point::new(x + 10.0, current_y), &font, &text_paint);
            current_y += 20.0;
        }
        
    }

    
}