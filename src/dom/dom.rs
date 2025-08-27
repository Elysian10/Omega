use indextree::{Arena, NodeId};
use std::collections::HashMap;

use crate::dom::{element::Element, layoutengine::LayoutData, node::NodeContent, text::Text};


#[derive(Debug)]
pub struct Dom {
    // The arena now only stores the tree structure (parent/child/sibling relationships)
    pub arena: Arena<()>, 
    pub root: Option<NodeId>,

    // SoA Data Collections
    // We use HashMaps here for simplicity, mapping a NodeId to its specific data.
    // For extreme performance, you might use a Vec and map NodeId to an index.
    pub content: HashMap<NodeId, NodeContent>,
    pub layout: HashMap<NodeId, LayoutData>,
    pub dirty: HashMap<NodeId, bool>,
}

impl Dom {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            root: None,
            content: HashMap::new(),
            layout: HashMap::new(),
            dirty: HashMap::new(),
        }
    }
    
    // Node creation methods now update the relevant HashMaps
    pub fn create_element(&mut self, element: Element) -> NodeId {
        let node_id = self.arena.new_node(());
        self.content.insert(node_id, NodeContent::Element(element));
        self.dirty.insert(node_id, true);
        node_id
    }

    pub fn create_text(&mut self, text: Text) -> NodeId {
        let node_id = self.arena.new_node(());
        self.content.insert(node_id, NodeContent::Text(text));
        self.dirty.insert(node_id, true);
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