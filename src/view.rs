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
    let style = ElementStyle {
        bg_color: Some(Color::new(0.1, 0.1, 0.1, 1.0)),
        padding: Some(BoxModelValues {
            top: 20.0,
            right: 20.0,
            bottom: 20.0,
            left: 20.0,
        }),
        border: Some(BorderStyle::default()
        .top(5.0, Color::new(0.0, 0.0, 1.0, 1.0))
        .right(5.0, Color::new(1.0, 0.0, 0.0, 1.0))
        .bottom(5.0, Color::new(0.0, 1.0, 0.0, 1.0))
        .left(5.0, Color::new(1.0, 1.0, 0.0, 1.0))),
        ..Default::default()
    };
    
    let text = dom.create_text(Text { content: "aaaaa aaaaa aaaaa aaaaa aaaaa aaaaa".to_owned(), font_size: 16.0, font_family: Some("Arial".to_owned()) });

    let inlinetest1 = dom.append_new_element(root_node_id, Element::new());
    let inlinetest2 = dom.append_new_element(root_node_id, Element::new());
    let blocktest1 = dom.append_new_element(root_node_id, Element::new());
    let blocktest2 = dom.append_new_element(root_node_id, Element::new());

    dom.append_child(inlinetest1, text);

    dom.set_display(inlinetest1, Display::InlineBlock);
    dom.set_display(inlinetest2, Display::InlineBlock);
    dom.set_element_style(inlinetest1, style.clone());
    dom.set_element_style(inlinetest2, style.clone());

    dom.set_element_style(blocktest1, style.clone());
    dom.set_element_style(blocktest2, style.clone());

    test_float(dom, root_node_id);
}

fn test_float(dom: &mut Dom, root_node_id: NodeId){
let style = ElementStyle {
        bg_color: Some(Color::new(0.1, 0.1, 0.1, 1.0)),
        border: Some(BorderStyle::default()
        .top(5.0, Color::new(0.0, 0.0, 1.0, 1.0))
        .right(5.0, Color::new(1.0, 0.0, 0.0, 1.0))
        .bottom(5.0, Color::new(0.0, 1.0, 0.0, 1.0))
        .left(5.0, Color::new(1.0, 1.0, 0.0, 1.0))),
        ..Default::default()
    };

    let text = dom.create_text(Text { content: "aaaaa aaaaa aaaaa aaaaa aaaaa aaaaa".to_owned(), font_size: 16.0, font_family: Some("Arial".to_owned()) });
    let floatleft1 = dom.append_new_styled_element(root_node_id, Element::new(), &style);
    let floatleft2 = dom.append_new_styled_element(root_node_id, Element::new(), &style);
    // let floatright1 = dom.append_new_styled_element(root_node_id, Element::new(), &style);
    // let floatright2 = dom.append_new_styled_element(root_node_id, Element::new(), &style);
    dom.set_float(floatleft1, Float::Left);
    dom.set_float(floatleft2, Float::Left);
    // dom.set_float(floatright1, Float::Right);
    // dom.set_float(floatright2, Float::Right);
    // dom.append_child(floatleft1, text.clone());
    dom.append_child(floatleft2, text);
    // dom.append_child(floatright1, text);
    // dom.append_child(floatright2, text);

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
