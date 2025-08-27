use indextree::NodeId;

use crate::dom::{
    dom::Dom,
    element::Element,
    styleengine::{Color, Style},
    text::Text,
};

pub fn create_view(dom: &mut Dom, root_node_id: NodeId) {
    // let child = Element::new(Color::new(0.0, 0.0, 1.0, 0.3));
    // let child_node_id = dom.create_element(child);
    // dom.append_child(parent_id, child_node_id);

    // let child2 = Element::new(Color::new(1.0, 0.0, 1.0, 0.3));
    // let child2_node_id = dom.create_element(child2);
    // dom.append_child(parent_id, child2_node_id);

    // let child3 = Text::new("h test\nme g", Color::new(1.0, 1.0, 1.0, 1.0));

    // let child3_node_id = dom.create_text(child3);

    // dom.append_child(parent_id, child3_node_id);
    let parent_id = dom.create_element(Element::new()); // Element is just a "tag" now
    dom.append_child(root_node_id, parent_id);
    dom.set_style(
        parent_id,
        Style {
            background_color: Some(Color::new(1.0, 1.0, 0.0, 1.0)),
            ..Default::default()
        },
    );

    let child1_id = dom.create_element(Element::new());
    dom.append_child(parent_id, child1_id);
    dom.set_style(
        child1_id,
        Style {
            background_color: Some(Color::new(0.0, 0.0, 1.0, 0.3)),
            ..Default::default()
        },
    );

    let child2_id = dom.create_element(Element::new());
    dom.append_child(parent_id, child2_id);
    dom.set_style(
        child2_id,
        Style {
            background_color: Some(Color::new(1.0, 0.0, 0.0, 0.3)),
            ..Default::default()
        },
    );

    // Node 3 (Text)
    let child3_node_id = dom.create_text(Text::new("h test\nme g"));
    dom.append_child(parent_id, child3_node_id);
    // You could set a background color for text, too!
    dom.set_style(
        child3_node_id,
        Style {
            background_color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            ..Default::default()
        },
    );
}
