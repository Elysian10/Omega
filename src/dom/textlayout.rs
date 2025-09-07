use skia_safe::{Font, FontStyle};

use crate::dom::{fontmanager::get_thread_local_font_mgr, layoutengine::TextInfo, Dom};

impl Dom{
fn calculate_line_height(font: &Font, font_size: f32) -> f32 {
        font_size * 9.0 / 8.0
    }

    pub fn measure_text(content: &str, font_family: String, font_size: f32, max_width: f32) -> (f32, f32, TextInfo) {
        let font_mgr = get_thread_local_font_mgr();
        let typeface = font_mgr
            .match_family_style(font_family, FontStyle::normal())
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