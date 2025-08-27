use indextree::NodeId;

use crate::dom::{dom::Dom, element::{Color, Element}, text::Text};



pub fn create_view(dom: &mut Dom, parent_id: NodeId){
    let child = Element::new(Color::new(0.0, 0.0, 1.0, 0.3));
    let child_node_id = dom.create_element(child);
    dom.append_child(parent_id, child_node_id);

    let child2 = Element::new(Color::new(1.0, 0.0, 1.0, 0.3));
    let child2_node_id = dom.create_element(child2);
    dom.append_child(parent_id, child2_node_id);

    let child3 = Text::new("h test\nme g", Color::new(1.0, 1.0, 1.0, 1.0));

    let child3_node_id = dom.create_text(child3);

    dom.append_child(parent_id, child3_node_id);
    
}