use skia_safe::{Canvas, Color4f, Font, FontMgr, FontStyle, Paint, PaintStyle, Point, Rect};

use crate::dom::fontmanager::get_thread_local_font_mgr;

#[derive(Debug, Clone)]
pub struct DebugTools {
    pub logs: Vec<String>,
    max_logs: usize,
}

impl DebugTools {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            max_logs: 10000, // Show only the most recent 50 logs
        }
    }
    
    pub fn log(&mut self, message: &str) {
        self.logs.push(message.to_string());
        
        // Keep only the most recent logs
        if self.logs.len() > self.max_logs {
            self.logs.remove(0);
        }
    }                                                                                                                                                                                                                                                               
    
    pub fn draw_debug_tools(&self, canvas: &Canvas, x: f32, y: f32, width: f32, height: f32) {
        // Draw a semi-transparent background
        let mut bg_paint = Paint::new(Color4f::new(0.1, 0.1, 0.1, 0.8), None);
        bg_paint.set_style(PaintStyle::Fill);
        
        let bg_rect = Rect::from_xywh(x, y, width, height);
        canvas.draw_rect(bg_rect, &bg_paint);
        
        // Set up text paint
        let mut text_paint = Paint::new(Color4f::new(1.0, 1.0, 1.0, 1.0), None);
        text_paint.set_style(PaintStyle::Fill);
        
        let font_mgr = get_thread_local_font_mgr();
        let typeface = font_mgr
            .legacy_make_typeface(None, FontStyle::normal())
            .expect("Failed to create typeface");
            
        let font = Font::new(typeface, 14.0);
        
        // Draw logs (newest at the top)
        let line_height = 18.0;
        let mut current_y = y + 40.0;
        canvas.draw_str(
                self.logs.len().to_string(),
                Point::new(x + 10.0, 20.0),
                &font,
                &text_paint
            );
        
        for log in self.logs.iter().rev() {
            if current_y > y + height {
                break; // Stop if we've run out of space
            }
            
            canvas.draw_str(
                log,
                Point::new(x + 10.0, current_y),
                &font,
                &text_paint
            );
            
            current_y += line_height;
        }
    }
}