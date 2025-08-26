use indextree::{Arena, NodeId};

use crate::dom::{element::Element, node::Node, text::Text};

#[derive(Debug)]
pub struct Dom {
    pub arena: Arena<Node>,
    pub root: Option<NodeId>,
}

impl Dom {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            root: None,
        }
    }
    
    pub fn create_element(&mut self, element: Element) -> NodeId {
        self.arena.new_node(Node::new_element(element))
    }

    pub fn create_text(&mut self, element: Text) -> NodeId {
        self.arena.new_node(Node::new_text(element))
    }
    
    pub fn set_root(&mut self, node_id: NodeId) {
        self.root = Some(node_id);
    }
    
    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        child_id.append(child_id, &mut self.arena);
    }

    pub fn try_append_child(&mut self, parent_id: NodeId, child_id: NodeId) -> Result<(), String> {
        if child_id == parent_id {
            return Err("Cannot append a node to itself".to_string());
        }
        
        if child_id.ancestors(&self.arena).any(|id| id == parent_id) {
            return Err("Cannot append an ancestor as a child".to_string());
        }
        
        if self.arena.get(child_id).is_none() || self.arena.get(parent_id).is_none() {
            return Err("Node has been removed from the arena".to_string());
        }
        
        parent_id.append(child_id, &mut self.arena);
        Ok(())
    }
    
    // Add more DOM manipulation methods as needed
}