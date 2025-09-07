// /src/dom/fragment.rs
use slotmap::{SecondaryMap, SlotMap};
use crate::dom::{
    dom::{NodeContent, NodeId}, element::Element, styleengine::{Style, Font}, text::Text, Dom
};

#[derive(Debug)]
pub struct DocumentFragment {
    // Similar structure to Dom but for a subtree
    nodes: SlotMap<slotmap::DefaultKey, ()>,
    pub children: SecondaryMap<slotmap::DefaultKey, Vec<NodeId>>,
    pub content: SecondaryMap<slotmap::DefaultKey, NodeContent>,
    pub element_styles: SecondaryMap<slotmap::DefaultKey, Style>,
    pub text_styles: SecondaryMap<slotmap::DefaultKey, Font>,
    pub root: Option<NodeId>,
}

impl DocumentFragment {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            children: SecondaryMap::new(),
            content: SecondaryMap::new(),
            element_styles: SecondaryMap::new(),
            text_styles: SecondaryMap::new(),
            root: None,
        }
    }

    pub fn create_element(&mut self, element: Element) -> NodeId {
        let key = self.nodes.insert(());
        let node_id = NodeId(key);
        
        self.content.insert(key, NodeContent::Element(element));
        self.element_styles.insert(key, Style::default());
        self.children.insert(key, Vec::new());
        
        node_id
    }

    pub fn create_text(&mut self, text: Text) -> NodeId {
        let key = self.nodes.insert(());
        let node_id = NodeId(key);
        
        self.content.insert(key, NodeContent::Text(text));
        self.text_styles.insert(key, Font::default());
        self.children.insert(key, Vec::new());
        
        node_id
    }

    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        if let Some(children) = self.children.get_mut(parent_id.into()) {
            children.push(child_id);
        }
    }

    // Method to merge fragment into main DOM
    pub fn merge_into(self, dom: &mut Dom, parent_id: NodeId) -> Vec<NodeId> {
        let mut id_mapping = SecondaryMap::new();
        let mut new_node_ids = Vec::new();
        
        // First pass: create all nodes in the DOM
        for (old_key, content) in self.content {
            let new_node_id = match content {
                NodeContent::Element(el) => dom.create_element(el),
                NodeContent::Text(text) => dom.create_text(text),
            };
            
            // Copy styles if they exist
            if let Some(style) = self.element_styles.get(old_key) {
                dom.set_style(new_node_id, style.clone());
            }
            
            id_mapping.insert(old_key, new_node_id);
            new_node_ids.push(new_node_id);
        }
        
        // Second pass: rebuild parent-child relationships
        for (old_key, children) in self.children {
            if let Some(&new_parent_id) = id_mapping.get(old_key) {
                for &old_child_key in &children {
                    if let Some(&new_child_id) = id_mapping.get(old_child_key.into()) {
                        dom.append_child(new_parent_id, new_child_id);
                    }
                }
            }
        }
        
        // Attach fragment's root children to the parent in the main DOM
        if let Some(fragment_root_id) = self.root {
            if let Some(&new_root_id) = id_mapping.get(fragment_root_id.into()) {
                dom.append_child(parent_id, new_root_id);
            }
        }
        
        new_node_ids
    }
}