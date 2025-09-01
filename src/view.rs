// view.rs
use crate::{
    dom::{
        Dom,
        dom::NodeId,
        element::Element,
        styleengine::{BorderStyle, BoxModelValues, BoxSizing, Color, Display, ElementStyle, Float, TextStyle},
        text::Text,
    },
    rsx,
};

pub fn create_view(dom: &mut Dom, root_node_id: NodeId) {
    // Create a border with different colors on each side
    let border_style = BorderStyle::default()
        .top(5.0, Color::new(0.0, 0.0, 1.0, 1.0))
        .right(5.0, Color::new(1.0, 0.0, 0.0, 1.0))
        .bottom(5.0, Color::new(0.0, 1.0, 0.0, 1.0))
        .left(5.0, Color::new(1.0, 1.0, 0.0, 1.0));

    let style = ElementStyle {
        bg_color: Some(Color::new(0.1, 0.1, 0.1, 1.0)),
        padding: Some(BoxModelValues {
            top: 20.0,
            right: 20.0,
            bottom: 20.0,
            left: 20.0,
        }),
        border: Some(border_style),
        ..Default::default()
    };

    let blocktest1 = dom.create_element(Element::new());
    let blocktest2 = dom.create_element(Element::new());
    let inlinetest1 = dom.create_element(Element::new());
    let inlinetest2 = dom.create_element(Element::new());

    dom.set_display(inlinetest1, Display::InlineBlock);
    dom.set_display(inlinetest2, Display::InlineBlock);
    dom.append_child(root_node_id, inlinetest1);
    dom.append_child(root_node_id, inlinetest2);
    dom.set_element_style(inlinetest1, style.clone());
    dom.set_element_style(inlinetest2, style.clone());
    let text = dom.create_text(Text { content: "test".to_owned(), font_size: 16.0, font_family: Some("Arial".to_owned()) });
    dom.append_child(inlinetest1, text);

    dom.append_child(root_node_id, blocktest1);
    dom.append_child(root_node_id, blocktest2);
    dom.set_element_style(blocktest1, style.clone());
    dom.set_element_style(blocktest2, style.clone());
}

// pub fn create_view(dom: &mut Dom, root_node_id: NodeId) {
//     rsx! {
//         dom,
//         root_node_id,
//         div {
//             border {
//                 top { width: 5.0, color: 0.0, 0.0, 1.0, 1.0 },
//                 right { width: 5.0, color: 1.0, 0.0, 0.0, 1.0 },
//                 bottom { width: 5.0, color: 0.0, 1.0, 0.0, 1.0 },
//                 left { width: 5.0, color: 1.0, 1.0, 0.0, 1.0 }
//             },
//             padding: [20.0, 20.0, 20.0, 20.0],
//             bg_color: [0.1, 0.1, 0.1, 1.0],
//             "inner text",
//             div {
//                 // Nested element
//                 bg_color: [0.2, 0.2, 0.2, 1.0],
//                 "nested text"
//             }
//         }
//     };
// }
