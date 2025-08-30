// /src/dom/node.rs
use crate::dom::{dom::NodeContent, element::Element, layoutengine::LayoutData, text::Text};

#[derive(Debug, Clone)]
pub struct Node {
    
    pub content: NodeContent,
    pub dirty: bool
}

impl Node {
    pub fn new_element(element: Element) -> Self {
        Self {
            content: NodeContent::Element(element),
            dirty: true
        }
    }

    pub fn new_text(text: Text) -> Self {
        Self {
            content: NodeContent::Text(text),
            dirty: true
        }
    }
}