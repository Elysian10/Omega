// /src/dom/element.rs

#[derive(Debug, Clone)]
pub struct Element {
    pub name: Option<String>
}

impl Element {
    pub fn new() -> Self {
        Self {
            name: None,
        }
    }
    
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}