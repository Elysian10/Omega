use crate::dom::{
    Dom,
    dom::{NodeContent, NodeId},
    styleengine::{BorderStyle, BoxModelValues, BoxSizing, Color, Display, Float},
};

impl Dom {
    pub fn set_display(&mut self, node_id: NodeId, display: Display) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.display = Some(display);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_width(&mut self, node_id: NodeId, width: f32) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.width = Some(width);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_height(&mut self, node_id: NodeId, height: f32) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.height = Some(height);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_bg_color(&mut self, node_id: NodeId, color: Color) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.bg_color = Some(color);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_margin(&mut self, node_id: NodeId, margin: BoxModelValues) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.margin = Some(margin);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_padding(&mut self, node_id: NodeId, padding: BoxModelValues) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.padding = Some(padding);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_border(&mut self, node_id: NodeId, border: BorderStyle) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.border = Some(border);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_box_sizing(&mut self, node_id: NodeId, box_sizing: BoxSizing) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.box_sizing = Some(box_sizing);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_float(&mut self, node_id: NodeId, float: Float) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.float = Some(float);
            self.set_dirty(node_id, true);
        }
    }

    // For text styles
    pub fn set_text_color(&mut self, node_id: NodeId, color: Color) {
        if let Some(style) = self.text_styles.get_mut(node_id.into()) {
            style.color = Some(color);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_font_size(&mut self, node_id: NodeId, font_size: f32) {
        if let Some(style) = self.text_styles.get_mut(node_id.into()) {
            style.font_size = Some(font_size);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_font_family(&mut self, node_id: NodeId, font_family: String) {
        if let Some(style) = self.text_styles.get_mut(node_id.into()) {
            style.font_family = Some(font_family);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_margin_top(&mut self, node_id: NodeId, value: f32) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            let mut margin = style.margin.unwrap_or_default();
            margin.top = value;
            style.margin = Some(margin);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_margin_right(&mut self, node_id: NodeId, value: f32) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            let mut margin = style.margin.unwrap_or_default();
            margin.right = value;
            style.margin = Some(margin);
            self.set_dirty(node_id, true);
        }
    }
}
