use crate::dom::dom::{Dom, NodeId};
use crate::dom::element::Element;
use crate::dom::text::Text;
use crate::dom::styleengine::{Color, BorderStyle, BorderSide, BoxModelValues, ElementStyle};

#[macro_export]
macro_rules! rsx {
    // Base case for empty element
    ($dom:expr, $parent:expr, $tag:ident { }) => {{
        let node = $dom.create_element(Element::new());
        $dom.append_child($parent, node);
        node
    }};
    
    // Element with properties and children
    ($dom:expr, $parent:expr, $tag:ident { $($content:tt)* }) => {{
        let node = $dom.create_element(Element::new());
        
        // Process the content
        rsx!(@process_content $dom, node, $($content)*);
        
        $dom.append_child($parent, node);
        node
    }};
    
    // Text node
    ($dom:expr, $parent:expr, $text:literal) => {{
        let text_node = $dom.create_text(Text { content: $text.to_string() });
        $dom.append_child($parent, text_node);
        text_node
    }};
    
    // Process content - border
    (@process_content $dom:expr, $node:expr, border { $($border_props:tt)* } $($rest:tt)*) => {{
        let border_style = rsx!(@process_border $($border_props)*);
        // Apply border style to element
        if let Some(mut style) = $dom.element_styles.get_mut($node.into()) {
            style.border = Some(border_style);
            $dom.dirty.insert($node.into(), true);
        }
        rsx!(@process_content $dom, $node, $($rest)*);
    }};
    
    // Process border properties
    (@process_border top { width: $width:expr, color: $r:expr, $g:expr, $b:expr, $a:expr } $($rest:tt)*) => {{
        let mut border = rsx!(@process_border $($rest)*);
        border.top = Some(crate::dom::styleengine::BorderSide { 
            width: $width, 
            color: Color::new($r, $g, $b, $a) 
        });
        border
    }};
    
    // Add similar patterns for right, bottom, left
    
    // Process content - padding
    (@process_content $dom:expr, $node:expr, padding: [$top:expr, $right:expr, $bottom:expr, $left:expr] $($rest:tt)*) => {{
        if let Some(mut style) = $dom.element_styles.get_mut($node.into()) {
            style.padding = Some(BoxModelValues {
                top: $top,
                right: $right,
                bottom: $bottom,
                left: $left,
            });
            $dom.dirty.insert($node.into(), true);
        }
        rsx!(@process_content $dom, $node, $($rest)*);
    }};
    
    // Process content - background color
    (@process_content $dom:expr, $node:expr, bg_color: [$r:expr, $g:expr, $b:expr, $a:expr] $($rest:tt)*) => {{
        if let Some(mut style) = $dom.element_styles.get_mut($node.into()) {
            style.bg_color = Some(Color::new($r, $g, $b, $a));
            $dom.dirty.insert($node.into(), true);
        }
        rsx!(@process_content $dom, $node, $($rest)*);
    }};
    
    // Process content - text
    (@process_content $dom:expr, $node:expr, $text:literal $($rest:tt)*) => {{
        let text_node = rsx!($dom, $node, $text);
        rsx!(@process_content $dom, $node, $($rest)*);
    }};
    
    // Process content - child element
    (@process_content $dom:expr, $node:expr, $child_tag:ident { $($child_content:tt)* } $($rest:tt)*) => {{
        let child_node = rsx!($dom, $node, $child_tag { $($child_content)* });
        rsx!(@process_content $dom, $node, $($rest)*);
    }};
    
    // Termination case for process_content
    (@process_content $dom:expr, $node:expr, ) => {};
    
    // Default border
    (@process_border) => {{
        BorderStyle::default()
    }};
}