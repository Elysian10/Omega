// /src/events.rs
use crate::dom::dom::{Dom, NodeId};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseEventType {
    Enter,
    Leave,
    Click,
    // Add more event types as needed
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub event_type: MouseEventType,
    pub node_id: NodeId,
    pub x: f32,
    pub y: f32,
}

pub struct EventSystem {
    hovered_nodes: Vec<NodeId>,
    event_listeners: HashMap<String, Vec<Box<dyn Fn(MouseEvent)>>>,
    current_hover: Option<NodeId>, // Track the currently hovered node
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            hovered_nodes: Vec::new(),
            event_listeners: HashMap::new(),
            current_hover: None,
        }
    }
    
    pub fn add_event_listener<F>(&mut self, event_type: &str, callback: F)
    where
        F: Fn(MouseEvent) + 'static,
    {
        self.event_listeners
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }
    
    fn trigger_event(&self, event: MouseEvent) {
        let event_name = match event.event_type {
            MouseEventType::Enter => "mouseenter",
            MouseEventType::Leave => "mouseleave",
            MouseEventType::Click => "click",
        };
        
        if let Some(listeners) = self.event_listeners.get(event_name) {
            for listener in listeners {
                listener(event.clone());
            }
        }
    }
    
    pub fn process_mouse_move(&mut self, dom: &Dom, x: f32, y: f32) {
        // Find the node at the current mouse position
        if let Some(node_id) = dom.find_node_at_position(x, y) {
            // Check if we've entered a new node
            if self.current_hover != Some(node_id) {
                // Trigger leave event for previously hovered node
                if let Some(previous) = self.current_hover {
                    self.trigger_event(MouseEvent {
                        event_type: MouseEventType::Leave,
                        node_id: previous,
                        x,
                        y,
                    });
                }
                
                // Trigger enter event for new node
                self.trigger_event(MouseEvent {
                    event_type: MouseEventType::Enter,
                    node_id,
                    x,
                    y,
                });
                
                // Update current hover
                self.current_hover = Some(node_id);
            }
        } else if self.current_hover.is_some() {
            // Mouse left all nodes
            if let Some(previous) = self.current_hover {
                self.trigger_event(MouseEvent {
                    event_type: MouseEventType::Leave,
                    node_id: previous,
                    x,
                    y,
                });
            }
            self.current_hover = None;
        }
    }
    
    // Add this method to get the currently hovered node
    pub fn get_hovered_node(&self) -> Option<NodeId> {
        self.current_hover
    }
}