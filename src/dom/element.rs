// element.rs
#[derive(Debug, Clone)]
pub struct Element {
    // This is now just a marker struct since all styling is handled
    // through the style system in Taffy's approach
}

impl Element {
    pub fn new() -> Self {
        Self {}
    }
}