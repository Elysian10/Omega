// /src/dom/text.rs

use crate::dom::styleengine::Color;

#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}