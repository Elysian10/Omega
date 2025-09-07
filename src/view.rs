use omega::rsx;

// view.rs
use crate::{
    dom::{
        Dom,
        dom::NodeId,
        element::Element,
        styleengine::{BorderStyle, BoxModelValues, BoxSizing, Color, Display, Style, Float, Font},
        text::Text,
    },
};

struct Div;

impl Div {
    fn default() -> Self {
        println!("Creating a default Div instance");
        Div
    }
}

pub fn create_view(dom: &mut Dom) {
    rsx!{div}
    let style = Style {
        bg_color: Some(Color::new(0.1, 0.1, 0.1, 1.0)),
        padding: Some(BoxModelValues::all(20.0)),
        border: Some(BorderStyle::default()
        .top(5.0, Color::BLUE)
        .right(5.0, Color::RED)
        .bottom(5.0, Color::GREEN)
        .left(5.0, Color::YELLOW)),
        ..Default::default()
    };
    
    let text = dom.create_text(Text::new("aaaaa aaaaa aaaaa aaaaa aaaaa aaaaa"));

    let inlinetest1 = dom.append_new_element(dom.root, Element::new());
    let inlinetest2 = dom.append_new_element(dom.root, Element::new());
    let blocktest1 = dom.append_new_element(dom.root, Element::new());
    let blocktest2 = dom.append_new_element(dom.root, Element::new());

    dom.append_child(inlinetest1, text);

    dom.set_display(inlinetest1, Display::InlineBlock);
    dom.set_display(inlinetest2, Display::InlineBlock);
    dom.set_style(inlinetest1, style.clone());
    dom.set_style(inlinetest2, style.clone());

    dom.set_style(blocktest1, style.clone());
    dom.set_style(blocktest2, style.clone());

    test_float(dom);
}

fn test_float(dom: &mut Dom){
let style = Style {
        bg_color: Some(Color::new(0.1, 0.1, 0.1, 1.0)),
        border: Some(BorderStyle::default()
        .top(5.0, Color::new(0.0, 0.0, 1.0, 1.0))
        .right(5.0, Color::new(1.0, 0.0, 0.0, 1.0))
        .bottom(5.0, Color::new(0.0, 1.0, 0.0, 1.0))
        .left(5.0, Color::new(1.0, 1.0, 0.0, 1.0))),
        ..Default::default()
    };

    let text = dom.create_text(Text { content: "aaaaa aaaaa aaaaa aaaaa aaaaa aaaaa".to_owned()});
    let floatleft1 = dom.append_new_styled_element(dom.root, Element::new(), &style);
    let floatleft2 = dom.append_new_styled_element(dom.root, Element::new(), &style);
    // let floatright1 = dom.append_new_styled_element(dom.root, Element::new(), &style);
    // let floatright2 = dom.append_new_styled_element(dom.root, Element::new(), &style);
    dom.set_float(floatleft1, Float::Left);
    dom.set_float(floatleft2, Float::Left);
    // dom.set_float(floatright1, Float::Right);
    // dom.set_float(floatright2, Float::Right);
    // dom.append_child(floatleft1, text.clone());
    dom.append_child(floatleft2, text);
    // dom.append_child(floatright1, text);
    // dom.append_child(floatright2, text);

}

// pub fn create_view(dom: &mut Dom, dom.root: NodeId) {
//     rsx! {
//         dom,
//         dom.root,
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
