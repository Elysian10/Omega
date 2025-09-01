use serde::{de::Error, Serialize, Serializer};
use serde_json::{Map, Value};

use crate::dom::{dom::{NodeContent, NodeId}, styleengine::{BorderSide, BorderStyle, BoxSizing, Color}, Dom};

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

impl Serialize for BoxSizing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            BoxSizing::ContentBox => serializer.serialize_str("content-box"),
            BoxSizing::BorderBox => serializer.serialize_str("border-box"),
            BoxSizing::Inherit => serializer.serialize_str("Inherit"),
        }
    }
}

impl Serialize for BorderStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(None)?;

        if let Some(top) = &self.top {
            map.serialize_entry("top", top)?;
        }
        if let Some(right) = &self.right {
            map.serialize_entry("right", right)?;
        }
        if let Some(bottom) = &self.bottom {
            map.serialize_entry("bottom", bottom)?;
        }
        if let Some(left) = &self.left {
            map.serialize_entry("left", left)?;
        }

        map.end()
    }
}

impl Serialize for BorderSide {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let border_str = format!("width:{:.1}, color:rgba({:.2}, {:.2}, {:.2}, {:.2})", self.width, self.color.r, self.color.g, self.color.b, self.color.a);
        serializer.serialize_str(&border_str)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Format as a compact string: "rgba(r, g, b, a)"
        let color_str = format!("rgba({:.2}, {:.2}, {:.2}, {:.2})", self.r, self.g, self.b, self.a);
        serializer.serialize_str(&color_str)
    }
}