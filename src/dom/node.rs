use crate::dom::{element::Element, layoutengine::LayoutData, text::Text};

#[derive(Debug, Clone)]
pub enum NodeContent {
    Element(Element),
    Text(Text),
}
#[derive(Debug, Clone)]
pub struct Node {
    
    pub content: NodeContent,
    pub layout_data: Option<LayoutData>,
    pub dirty: bool
}

impl Node {
    pub fn new_element(element: Element) -> Self {
        Self {
            content: NodeContent::Element(element),
            layout_data: None,
            dirty: true
        }
    }

    pub fn new_text(text: Text) -> Self {
        Self {
            content: NodeContent::Text(text),
            layout_data: None,
            dirty: true
        }
    }
}