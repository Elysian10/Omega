// view.rs
use crate::dom::{
    dom::NodeId, element::Element, styleengine::{BorderStyle, BoxModelValues, Color, ElementStyle, TextStyle}, text::Text, Dom
};

pub fn create_view(dom: &mut Dom, root_node_id: NodeId) {
    let parent_id = dom.create_element(Element::new());
    dom.append_child(root_node_id, parent_id);
    
    // Create a border with different colors on each side
    let border_style = BorderStyle::default()
        .top(5.0, Color::new(0.0, 0.0, 1.0, 1.0))    // Blue top
        .right(5.0, Color::new(1.0, 0.0, 0.0, 1.0))  // Red right
        .bottom(5.0, Color::new(0.0, 1.0, 0.0, 1.0)) // Green bottom
        .left(5.0, Color::new(1.0, 1.0, 0.0, 1.0));  // Yellow left
    
    dom.set_element_style(
        parent_id,
        ElementStyle {
            background_color: Some(Color::new(0.1, 0.1, 0.1, 1.0)),
            padding: Some(BoxModelValues{top: 20.0, right: 20.0, bottom: 20.0, left: 20.0}),
            border: Some(border_style),
            ..Default::default()
        },
    );

    // Create a uniform border (all sides same)
    let uniform_border = BorderStyle::all(1.0, Color::new(0.5, 0.5, 0.5, 1.0));
    
    let child_id = dom.create_element(Element::new());
    dom.append_child(parent_id, child_id);
    
    dom.set_element_style(
        child_id,
        ElementStyle {
            background_color: Some(Color::new(0.2, 0.2, 0.2, 1.0)),
            border: Some(uniform_border),
            ..Default::default()
        },
    );

    // Create text node
    let text_node_id = dom.create_text(Text::new("Test content"));
    dom.append_child(child_id, text_node_id);
}