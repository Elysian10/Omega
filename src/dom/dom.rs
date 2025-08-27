use indextree::{Arena, NodeId};
use std::collections::HashMap;

use crate::dom::{element::Element, layoutengine::{LayoutData, TextInfo}, node::NodeContent, styleengine::{ComputedStyle, Style}, text::Text};


#[derive(Debug)]
pub struct Dom {
    pub arena: Arena<()>, 
    pub root: Option<NodeId>,

    // SoA Data Collections
    pub content: HashMap<NodeId, NodeContent>,
    pub layout: HashMap<NodeId, LayoutData>,
    pub text_info: HashMap<NodeId, TextInfo>,
    pub dirty: HashMap<NodeId, bool>,
    pub styles: HashMap<NodeId, Style>,
    pub computed_styles: HashMap<NodeId, ComputedStyle>
}

impl Dom {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            root: None,
            content: HashMap::new(),
            layout: HashMap::new(),
            text_info: HashMap::new(),
            dirty: HashMap::new(),
            styles: HashMap::new(),
            computed_styles: HashMap::new(),
        }
    }

    pub fn set_style(&mut self, node_id: NodeId, style: Style) {
        self.styles.insert(node_id, style);
        // Mark as dirty, since styles affect layout/rendering
        self.dirty.insert(node_id, true); 
    }

    pub fn set_text_info(&mut self, node_id: NodeId, text_info: TextInfo) {
        self.text_info.insert(node_id, text_info);
    }
    
    // Node creation methods now update the relevant HashMaps
    pub fn create_element(&mut self, element: Element) -> NodeId {
        let node_id = self.arena.new_node(());
        self.content.insert(node_id, NodeContent::Element(element));
        self.dirty.insert(node_id, true);
        // We also give it a default style entry
        self.styles.insert(node_id, Style::default());
        node_id
    }

    pub fn create_text(&mut self, text: Text) -> NodeId {
        let node_id = self.arena.new_node(());
        self.content.insert(node_id, NodeContent::Text(text));
        self.dirty.insert(node_id, true);
        self.styles.insert(node_id, Style::default());
        node_id
    }
    
    // Other methods like set_root and append_child remain largely the same
    pub fn set_root(&mut self, node_id: NodeId) {
        self.root = Some(node_id);
    }
    
    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        parent_id.append(child_id, &mut self.arena);
    }
}