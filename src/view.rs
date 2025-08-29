// view.rs
use crate::dom::{
    dom::NodeId, element::Element, styleengine::{BoxModelValues, Color, ElementStyle, TextStyle}, text::Text, Dom
};

pub fn create_view(dom: &mut Dom, root_node_id: NodeId) {
    let parent_id = dom.create_element(Element::new());
    dom.append_child(root_node_id, parent_id);
    
    // Set element style using the new method
    dom.set_element_style(
        parent_id,
        ElementStyle {
            background_color: Some(Color::new(1.0, 1.0, 0.0, 1.0)),
            padding: Some(BoxModelValues{top: 20.0, right: 20.0, bottom: 20.0, left: 20.0}),
            ..Default::default()
        },
    );

    let child1_id = dom.create_element(Element::new());
    dom.append_child(parent_id, child1_id);
    
    // Set element style using the new method
    dom.set_element_style(
        child1_id,
        ElementStyle {
            background_color: Some(Color::new(0.0, 0.0, 1.0, 1.0)),
            ..Default::default()
        },
    );

    let child2_id = dom.create_element(Element::new());
    dom.append_child(parent_id, child2_id);
    
    // Set element style using the new method
    dom.set_element_style(
        child2_id,
        ElementStyle {
            background_color: Some(Color::new(1.0, 0.0, 0.0, 1.0)),
            ..Default::default()
        },
    );

    // Create text node
    let child3_node_id = dom.create_text(Text::new("h test\nme g"));
    dom.append_child(child2_id, child3_node_id);
    
    // Set text style using the new method (not element style)
    // Text nodes don't have background color in the new system
    dom.set_text_style(
        child3_node_id,
        TextStyle {
            color: Some(Color::new(0.0, 0.0, 0.0, 1.0)), // Black text
            font_size: Some(16.0),
            font_family: Some("Arial".to_string()),
        },
    );
}