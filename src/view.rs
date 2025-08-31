// view.rs
use crate::dom::{
    dom::NodeId, element::Element, styleengine::{BorderStyle, BoxModelValues, Color, Display, ElementStyle, TextStyle}, text::Text, Dom
};

pub fn create_view(dom: &mut Dom, root_node_id: NodeId) {
    // Create a border with different colors on each side
    let border_style = BorderStyle::default()
        .top(5.0, Color::new(0.0, 0.0, 1.0, 1.0)) // Blue top
        .right(5.0, Color::new(1.0, 0.0, 0.0, 1.0)) // Red right
        .bottom(5.0, Color::new(0.0, 1.0, 0.0, 1.0)) // Green bottom
        .left(5.0, Color::new(1.0, 1.0, 0.0, 1.0)); // Yellow left

    let style = ElementStyle {
        background_color: Some(Color::new(0.1, 0.1, 0.1, 1.0)),
        // padding: Some(BoxModelValues {
        //     top: 20.0,
        //     right: 20.0,
        //     bottom: 20.0,
        //     left: 20.0,
        // }),
        border: Some(border_style),
        ..Default::default()
    };

    let child1 = dom.create_element(Element::new());
    let child2 = dom.create_element(Element::new());

    dom.set_display(child1, Display::InlineBlock);
    dom.set_display(child2, Display::InlineBlock);
    dom.append_child(root_node_id, child1);
    dom.append_child(root_node_id, child2);

    dom.set_element_style(
        child1,
        style.clone(),
    );

    dom.set_element_style(
        child2,
        style
    );
}
