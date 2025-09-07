use slotmap::SecondaryMap;

use crate::dom::{
    dom::{NodeContent, NodeId}, element::{Element}, layoutengine::TextInfo, styleengine::{BorderStyle, BoxModelValues, BoxSizing, Color, Display, Float, Font, Size, Style}, text::Text, Dom
};

#[derive(Debug)]
pub struct StyleManager {
    pub element_styles: SecondaryMap<slotmap::DefaultKey, Style>,
}

impl StyleManager {
    pub fn new() -> Self {
        Self { element_styles: SecondaryMap::new() }
    }

    fn set_display(&mut self, node_id: NodeId, display: Display) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.display = Some(display);
        }
    }

    fn set_width(&mut self, node_id: NodeId, width: f32) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.width = Some(Size::Points(width));
        }
    }

    fn set_height(&mut self, node_id: NodeId, height: f32) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.height = Some(Size::Points(height));
        }
    }

    fn set_bg_color(&mut self, node_id: NodeId, color: Color) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.bg_color = Some(color);
        }
    }

    fn set_margin(&mut self, node_id: NodeId, margin: BoxModelValues) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.margin = Some(margin);
        }
    }

    fn set_padding(&mut self, node_id: NodeId, padding: BoxModelValues) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.padding = Some(padding);
        }
    }

    fn set_border(&mut self, node_id: NodeId, border: BorderStyle) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.border = Some(border);
        }
    }

    fn set_box_sizing(&mut self, node_id: NodeId, box_sizing: BoxSizing) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.box_sizing = Some(box_sizing);
        }
    }

    fn set_float(&mut self, node_id: NodeId, float: Float) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            style.float = Some(float);
        }
    }

    fn set_margin_top(&mut self, node_id: NodeId, value: f32) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            let mut margin = style.margin.unwrap_or_default();
            margin.top = Some(value);
            style.margin = Some(margin);
        }
    }

    fn set_margin_right(&mut self, node_id: NodeId, value: f32) {
        if let Some(style) = self.element_styles.get_mut(node_id.into()) {
            let mut margin = style.margin.unwrap_or_default();
            margin.right = Some(value);
            style.margin = Some(margin);
        }
    }

    fn set_element_style(&mut self, node_id: NodeId, style: Style) {
        let key: slotmap::DefaultKey = node_id.into();

        let mut current_style = self.element_styles.get(key).cloned().unwrap_or_default();
        current_style.apply(&style);

        self.element_styles.insert(key, current_style);
    }
}

impl Dom {
    pub fn set_display(&mut self, node_id: NodeId, display: Display) {
        self.style_manager.set_display(node_id, display);
        self.set_dirty(node_id, true);
    }

    fn set_dirty(&self, node_id: NodeId, is_dirty: bool) {
        self.dirty.borrow_mut().insert(node_id.into(), is_dirty);
    }

    pub fn set_width(&mut self, node_id: NodeId, width: f32) {
        if let Some(style) = self.styles.get_mut(node_id.into()) {
            style.width = Some(Size::Points(width));
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_height(&mut self, node_id: NodeId, height: f32) {
        if let Some(style) = self.styles.get_mut(node_id.into()) {
            style.height = Some(Size::Points(height));
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_bg_color(&mut self, node_id: NodeId, color: Color) {
        if let Some(style) = self.styles.get_mut(node_id.into()) {
            style.bg_color = Some(color);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_margin(&mut self, node_id: NodeId, margin: BoxModelValues) {
        if let Some(style) = self.styles.get_mut(node_id.into()) {
            style.margin = Some(margin);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_padding(&mut self, node_id: NodeId, padding: BoxModelValues) {
        if let Some(style) = self.styles.get_mut(node_id.into()) {
            style.padding = Some(padding);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_border(&mut self, node_id: NodeId, border: BorderStyle) {
        if let Some(style) = self.styles.get_mut(node_id.into()) {
            style.border = Some(border);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_box_sizing(&mut self, node_id: NodeId, box_sizing: BoxSizing) {
        if let Some(style) = self.styles.get_mut(node_id.into()) {
            style.box_sizing = Some(box_sizing);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_float(&mut self, node_id: NodeId, float: Float) {
        if let Some(style) = self.styles.get_mut(node_id.into()) {
            style.float = Some(float);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_margin_top(&mut self, node_id: NodeId, value: f32) {
        if let Some(style) = self.styles.get_mut(node_id.into()) {
            let mut margin = style.margin.unwrap_or_default();
            margin.top = Some(value);
            style.margin = Some(margin);
            self.set_dirty(node_id, true);
        }
    }

    pub fn set_margin_right(&mut self, node_id: NodeId, value: f32) {
        if let Some(style) = self.styles.get_mut(node_id.into()) {
            let mut margin = style.margin.unwrap_or_default();
            margin.right = Some(value);
            style.margin = Some(margin);
            self.set_dirty(node_id, true);
        }
    }

    pub fn is_dirty(&self, node_id: NodeId) -> bool {
        *self.dirty.borrow().get(node_id.into()).unwrap_or(&false)
    }

    pub fn clear_dirty(&self, node_id: NodeId) {
        self.dirty.borrow_mut().remove(node_id.into());
    }

    pub fn set_style(&mut self, node_id: NodeId, style: Style) {
        let key: slotmap::DefaultKey = node_id.into();

        // Get the current style or create a default one
        let mut current_style = self.styles.get(key).cloned().unwrap_or_default();

        // Apply the new style properties to the current style
        current_style.apply(&style);

        self.styles.insert(key, current_style);
        self.dirty.borrow_mut().insert(node_id.into(), true);
    }

    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        let parent_key: slotmap::DefaultKey = parent_id.into();
        let child_key: slotmap::DefaultKey = child_id.into();

        // Add child to parent's children list
        if let Some(children) = self.children.get_mut(parent_key) {
            children.push(child_id);
        }
        

        self.parents.insert(child_key, Some(parent_id));
    }

    pub fn is_element(&self, id: NodeId) -> bool {
        let key: slotmap::DefaultKey = id.into();
        match self.content.get(key) {
            Some(NodeContent::Element(_)) => true,
            _ => false,
        }
    }

    pub fn set_inner_text(&mut self, element_id: NodeId, text: String) {
        let element_key: slotmap::DefaultKey = element_id.into();
        
        // Verify the node exists and is an element
        if !self.nodes.contains_key(element_key) {
            return;
        }
        
        match self.content.get(element_key) {
            Some(NodeContent::Element(_)) => {
                // Remove all existing children
                self.remove_all_children(element_id);
                
                // Create and append a new text node
                let text_node_id = self.create_text_node(text);
                self.append_child(element_id, text_node_id);
                
            }
            Some(NodeContent::Text(_)) => {},
            None => {},
        }
    }

    pub fn get_inner_text(&self, element_id: NodeId) -> Option<String> {
        let element_key: slotmap::DefaultKey = element_id.into();
        
        
        match self.content.get(element_key) {
            Some(NodeContent::Element(_)) => {
                let mut text_parts = Vec::new();
                
                // Collect text from all child text nodes
                if let Some(children) = self.children.get(element_key) {
                    for &child_id in children {
                        let child_key: slotmap::DefaultKey = child_id.into();
                        if let Some(NodeContent::Text(text_node)) = self.content.get(child_key) {
                            text_parts.push(text_node.content.clone());
                        }
                    }
                }
                
                Some(text_parts.join(""))
            },
            Some(Text)=> None,
            None => None
        }
    }

    fn remove_all_children(&mut self, parent_id: NodeId) {
        let parent_key: slotmap::DefaultKey = parent_id.into();
        
        if let Some(children) = self.children.get(parent_key).cloned() {
            for child_id in children {
                self.remove_node(child_id);
            }
        }
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        let key: slotmap::DefaultKey = node_id.into();
        
        // Check if node exists
        if !self.nodes.contains_key(key) {
            return;
        }
        
        // Remove from parent's children list
        if let Some(Some(parent_id)) = self.parents.get(key).cloned() {
            if let Some(children) = self.children.get_mut(parent_id.into()) {
                children.retain(|&id| id != node_id);
            }
            self.set_dirty(parent_id, true);
        }
        
        // Remove all children recursively
        if let Some(children) = self.children.get(key).cloned() {
            for child_id in children {
                self.remove_node(child_id);
            }
        }
        
        // Remove from storage
        self.nodes.remove(key);
        self.content.remove(key);
        self.styles.remove(key);
        self.computed_styles.remove(key);
        self.text_info.remove(key);
        self.layout.remove(key);
        self.dirty.borrow_mut().remove(key);
        self.children.remove(key);
        self.parents.remove(key);
        
    }

    pub fn create_text_node(&mut self, text: String) -> NodeId {
        let key = self.nodes.insert(());
        let node_id = NodeId(key);

        self.content.insert(key, NodeContent::Text(Text::new(text)));
        self.children.insert(key, Vec::new());
        self.parents.insert(key, None);
        self.set_dirty(node_id, true);

        node_id
    }
}
