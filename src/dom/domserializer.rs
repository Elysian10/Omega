use serde::de::Error;
use serde_json::{Map, Value};

use crate::dom::{dom::{NodeContent, NodeId}, Dom};

impl Dom{
    pub fn node_to_json_value(&self, node_id: NodeId) -> Result<Value, serde_json::Error> {
        let key: slotmap::DefaultKey = node_id.into();
        
        let mut result = Map::new();
        result.insert("node_id".to_string(), Value::String(format!("{:?}", node_id.0)));

        // Add content information
        if let Some(content) = self.content.get(key) {
            match content {
                NodeContent::Element(e) => {
                result.insert("type".to_string(), Value::String("element".to_string()));
                // Add element name if available
                if let Some(name) = &e.name {
                    result.insert("name".to_string(), Value::String(name.clone()));
                }
                }
                NodeContent::Text(t) => {
                    result.insert("type".to_string(), Value::String("text".to_string()));
                    result.insert("text_content".to_string(), Value::String(t.content.clone()));
                }
            }
        }

        // Add styles
        if let Some(style) = self.element_styles.get(key) {
            result.insert("element_style".to_string(), serde_json::to_value(style)?);
        }

        if let Some(computed_style) = self.computed_element_styles.get(key) {
            result.insert("computed_element_style".to_string(), serde_json::to_value(computed_style)?);
        }

        if let Some(text_style) = self.text_styles.get(key) {
            result.insert("text_style".to_string(), serde_json::to_value(text_style)?);
        }

        if let Some(computed_text_style) = self.computed_text_styles.get(key) {
            result.insert("computed_text_style".to_string(), serde_json::to_value(computed_text_style)?);
        }

        // Add layout data if available
        if let Some(layout) = self.layout.get(key) {
            result.insert("layout".to_string(), serde_json::to_value(layout)?);
        }

        Ok(Value::Object(result))
    }

    pub fn get_json_at_path(&self, node_id: NodeId, path: &str) -> Result<String, serde_json::Error> {
        let value = self.node_to_json_value(node_id)?;
        
        // Split the path by dots to navigate the JSON structure
        let path_parts: Vec<&str> = path.split('.').collect();
        let mut current = &value;
        
        for part in path_parts {
            if part.is_empty() {
                continue;
            }
            
            if let Some(obj) = current.as_object() {
                if let Some(next) = obj.get(part) {
                    current = next;
                } else {
                    return Err(serde_json::Error::custom(format!("Path '{}' not found", part)));
                }
            } else {
                return Err(serde_json::Error::custom(format!("Cannot access '{}' on non-object", part)));
            }
        }
        
        serde_json::to_string_pretty(current)
    }
}