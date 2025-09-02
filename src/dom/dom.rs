// /src/dom/dom.rs

use std::cell::RefCell;

use crate::dom::{
    element::Element,
    layoutengine::{LayoutData, TextInfo},
    styleengine::{ComputedElementStyle, ComputedTextStyle, Display, ElementStyle, TextStyle},
    text::Text,
};
use slotmap::{SecondaryMap, SlotMap};

#[derive(Debug, Clone)]
pub enum NodeContent {
    Element(Element),
    Text(Text),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub(crate) slotmap::DefaultKey);

// Convert from SlotMap key to NodeId
impl From<slotmap::DefaultKey> for NodeId {
    fn from(key: slotmap::DefaultKey) -> Self {
        NodeId(key)
    }
}

// Convert from NodeId to SlotMap key
impl From<NodeId> for slotmap::DefaultKey {
    fn from(node_id: NodeId) -> Self {
        node_id.0
    }
}

#[derive(Debug)]
pub struct Dom {
    // Tree structure using SlotMap
    nodes: SlotMap<slotmap::DefaultKey, ()>,
    pub children: SecondaryMap<slotmap::DefaultKey, Vec<NodeId>>,
    pub parents: SecondaryMap<slotmap::DefaultKey, Option<NodeId>>,
    pub root: Option<NodeId>,

    // SoA Data Collections
    pub content: SecondaryMap<slotmap::DefaultKey, NodeContent>,
    pub layout: SecondaryMap<slotmap::DefaultKey, LayoutData>,
    pub text_info: SecondaryMap<slotmap::DefaultKey, TextInfo>,
    pub dirty: RefCell<SecondaryMap<slotmap::DefaultKey, bool>>,

    // Separate style storage
    pub element_styles: SecondaryMap<slotmap::DefaultKey, ElementStyle>,
    pub text_styles: SecondaryMap<slotmap::DefaultKey, TextStyle>,
    pub computed_element_styles: SecondaryMap<slotmap::DefaultKey, ComputedElementStyle>,
    pub computed_text_styles: SecondaryMap<slotmap::DefaultKey, ComputedTextStyle>,
}

impl Dom {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            children: SecondaryMap::new(),
            parents: SecondaryMap::new(),
            root: None,
            content: SecondaryMap::new(),
            layout: SecondaryMap::new(),
            text_info: SecondaryMap::new(),
            dirty: RefCell::new(SecondaryMap::new()),
            element_styles: SecondaryMap::new(),
            text_styles: SecondaryMap::new(),
            computed_element_styles: SecondaryMap::new(),
            computed_text_styles: SecondaryMap::new(),
        }
    }

    // Node creation methods
    pub fn create_element(&mut self, element: Element) -> NodeId {
        let key = self.nodes.insert(());
        let node_id = NodeId(key);

        self.content.insert(key, NodeContent::Element(element));
        self.element_styles.insert(key, ElementStyle::default());
        self.dirty.borrow_mut().insert(node_id.into(), true);
        self.children.insert(key, Vec::new());
        self.parents.insert(key, None);

        node_id
    }

    pub fn append_new_element(&mut self, parent_id: NodeId, element: Element) -> NodeId {
        let child_id = self.create_element(element);
        self.append_child(parent_id, child_id);
        child_id
    }

    pub fn append_new_styled_element(&mut self, parent_id: NodeId, element: Element, style: &ElementStyle) -> NodeId {
        let child_id = self.create_element(element);
        self.set_element_style(child_id, style.clone());
        self.append_child(parent_id, child_id);
        child_id
    }

    pub fn create_text(&mut self, text: Text) -> NodeId {
        let key = self.nodes.insert(());
        let node_id = NodeId(key);

        self.content.insert(key, NodeContent::Text(text));
        self.text_styles.insert(key, TextStyle::default());
        self.dirty.borrow_mut().insert(node_id.into(), true);
        self.children.insert(key, Vec::new());
        self.parents.insert(key, None);

        node_id
    }
    

    pub fn children(&self, node_id: NodeId) -> Option<&Vec<NodeId>> {
        self.children.get(node_id.into())
    }

    pub fn parent(&self, node_id: NodeId) -> Option<NodeId> {
        self.parents.get(node_id.into()).and_then(|p| *p)
    }

    pub fn collect_nodes_depth_first(&self, root_id: NodeId) -> Vec<NodeId> {
        let mut nodes = Vec::new();
        let mut stack = vec![root_id];

        while let Some(node_id) = stack.pop() {
            nodes.push(node_id);

            // Push children in reverse order so they're processed in correct order
            if let Some(children) = self.children.get(node_id.into()) {
                for &child_id in children.iter().rev() {
                    stack.push(child_id);
                }
            }
        }
        nodes
    }

    pub fn find_node_at_position(&self, x: f32, y: f32) -> Option<NodeId> {
        if let Some(root_id) = self.root {
            // Collect all nodes in depth-first order (from top to bottom in rendering order)
            let nodes = self.collect_nodes_depth_first(root_id);

            // Check nodes in reverse order (from top-most to bottom-most)
            for node_id in nodes.iter().rev() {
                let key: slotmap::DefaultKey = (*node_id).into();

                if let Some(layout_data) = self.layout.get(key) {
                    // Check if the point is inside the node's bounds
                    if x >= layout_data.computed_x && x <= layout_data.computed_x + layout_data.actual_width && y >= layout_data.computed_y && y <= layout_data.computed_y + layout_data.actual_height {
                        return Some(*node_id);
                    }
                }
            }
            return Some(root_id);
        }
        None
    }
}
