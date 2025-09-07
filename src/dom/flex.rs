// /src/dom/flex.rs

use super::dom::{Dom, NodeId};
use super::layoutengine::Rect;
use super::styleengine::{
    AlignContent, AlignItems, AlignSelf, ComputedElementStyle, FlexDirection, JustifyContent, Size,
};

/// A temporary structure to hold information about a flex item during layout.
/// This helps avoid borrow-checker issues with the DOM.
struct FlexItem {
    node_id: NodeId,
    style: ComputedElementStyle,
    // The final computed main and cross sizes of the item
    main_size: f32,
    cross_size: f32,
    // The final computed position of the item relative to the container's content box
    x: f32,
    y: f32,
}

/// Represents a single line in a flex container.
struct FlexLine {
    items: Vec<FlexItem>,
    main_size: f32,
    cross_size: f32,
}

impl Dom {
    /// Lays out a node with `display: flex`.
    pub fn layout_flex_node(
        &mut self,
        node_id: NodeId,
        available_space: Rect,
        element_style: &ComputedElementStyle,
    ) -> Rect {
        let key = node_id.into();

        // --- 1. Calculate Container's Content Box ---
        // This is the inner area where flex items will be placed.
        let padding = &element_style.padding;
        let border = &element_style.border;
        let border_left = border.left.map_or(0.0, |b| b.width);
        let border_right = border.right.map_or(0.0, |b| b.width);
        let border_top = border.top.map_or(0.0, |b| b.width);
        let border_bottom = border.bottom.map_or(0.0, |b| b.width);

        let container_width = match element_style.width {
            Some(Size::Points(w)) => w,
            Some(Size::Percent(p)) => available_space.width * p / 100.0,
            _ => available_space.width,
        };

        // The content area for the flex items
        let content_area_width = container_width
            - padding.left
            - padding.right
            - border_left
            - border_right;
        let content_area_height = match element_style.height {
            Some(Size::Points(h)) => h - padding.top - padding.bottom - border_top - border_bottom,
            Some(Size::Percent(p)) => (available_space.height * p / 100.0) - padding.top - padding.bottom - border_top - border_bottom,
            _ => f32::INFINITY, // Auto height initially
        };

        let is_row = matches!(element_style.flex_direction, FlexDirection::Row | FlexDirection::RowReverse);
        let main_axis_size = if is_row { content_area_width } else { content_area_height };

        // --- 2. Collect and Size Flex Items ---
        let child_ids = self.children.get(key).cloned().unwrap_or_default();
        let mut flex_items: Vec<FlexItem> = Vec::new();
        for child_id in child_ids {
            let child_key = child_id.into();
            if let Some(child_style) = self.computed_element_styles.get(child_key).cloned() {
                // To get intrinsic size, we do a preliminary layout pass
                let preliminary_space = Rect {
                    x: 0.0, y: 0.0,
                    width: content_area_width,
                    height: content_area_height,
                };
                let preliminary_rect = self.layout_node(child_id, preliminary_space);

                let item_main_size = if is_row { preliminary_rect.width } else { preliminary_rect.height };
                let item_cross_size = if is_row { preliminary_rect.height } else { preliminary_rect.width };

                flex_items.push(FlexItem {
                    node_id: child_id,
                    style: child_style,
                    main_size: item_main_size,
                    cross_size: item_cross_size,
                    x: 0.0, y: 0.0,
                });
            }
        }

        // Handle flex-direction: *-reverse
        if matches!(element_style.flex_direction, FlexDirection::RowReverse | FlexDirection::ColumnReverse) {
            flex_items.reverse();
        }

        // --- 3. Determine Flex Lines (Wrapping) ---
        let mut flex_lines: Vec<FlexLine> = Vec::new();
        if element_style.flex_wrap == super::styleengine::FlexWrap::NoWrap {
             // All items go on a single line
             let total_main_size: f32 = flex_items.iter().map(|item| item.main_size).sum();
             let max_cross_size: f32 = flex_items.iter().map(|item| item.cross_size).fold(0.0, f32::max);
             flex_lines.push(FlexLine { items: flex_items, main_size: total_main_size, cross_size: max_cross_size });
        } else {
            // Handle wrapping
            let mut current_line = FlexLine { items: vec![], main_size: 0.0, cross_size: 0.0 };
            for item in flex_items {
                if !current_line.items.is_empty() && current_line.main_size + item.main_size > main_axis_size {
                    flex_lines.push(current_line);
                    current_line = FlexLine { items: vec![], main_size: 0.0, cross_size: 0.0 };
                }
                current_line.main_size += item.main_size;
                current_line.cross_size = current_line.cross_size.max(item.cross_size);
                current_line.items.push(item);
            }
            if !current_line.items.is_empty() {
                flex_lines.push(current_line);
            }
        }

        // --- 4. Main Axis Alignment (Justify Content) ---
        for line in &mut flex_lines {
            let free_space = main_axis_size - line.main_size;
            let mut spacing = 0.0;
            let mut offset = 0.0;

            if free_space > 0.0 {
                match element_style.justify_content {
                    JustifyContent::FlexStart => {},
                    JustifyContent::FlexEnd => offset = free_space,
                    JustifyContent::Center => offset = free_space / 2.0,
                    JustifyContent::SpaceBetween => {
                        if line.items.len() > 1 {
                            spacing = free_space / (line.items.len() - 1) as f32;
                        }
                    },
                    JustifyContent::SpaceAround => {
                        spacing = free_space / line.items.len() as f32;
                        offset = spacing / 2.0;
                    },
                    JustifyContent::SpaceEvenly => {
                        spacing = free_space / (line.items.len() + 1) as f32;
                        offset = spacing;
                    }
                }
            }
            
            let mut current_main = offset;
            for item in &mut line.items {
                if is_row { item.x = current_main; } else { item.y = current_main; }
                current_main += item.main_size + spacing;
            }
        }
        
        // --- 5. Cross Axis Alignment (Align Items & Align Content) ---
        let total_cross_size: f32 = flex_lines.iter().map(|line| line.cross_size).sum();
        let mut cross_offset = 0.0;
        
        let free_cross_space = if content_area_height.is_finite() { content_area_height - total_cross_size } else { 0.0 };

        if free_cross_space > 0.0 {
             match element_style.align_content {
                AlignContent::FlexStart => {},
                AlignContent::FlexEnd => cross_offset = free_cross_space,
                AlignContent::Center => cross_offset = free_cross_space / 2.0,
                // Other align-content logic would go here...
                _ => {}
            }
        }


        let mut current_cross = cross_offset;
        for line in &mut flex_lines {
            for item in &mut line.items {
                let align = if item.style.align_self == AlignSelf::Auto { element_style.align_items } else { item.style.align_self.into() };

                let item_cross_pos = match align {
                    AlignItems::FlexStart | AlignItems::Stretch => current_cross,
                    AlignItems::FlexEnd => current_cross + line.cross_size - item.cross_size,
                    AlignItems::Center => current_cross + (line.cross_size - item.cross_size) / 2.0,
                    _ => current_cross, // Baseline not implemented
                };
                
                if is_row { item.y = item_cross_pos; } else { item.x = item_cross_pos; }
            }
            current_cross += line.cross_size;
        }

        // --- 6. Finalize Layout and Update DOM ---
        let content_start_x = available_space.x + element_style.margin.left + border_left + padding.left;
        let content_start_y = available_space.y + element_style.margin.top + border_top + padding.top;
        
        for line in flex_lines {
            for item in line.items {
                self.layout.insert(
                    item.node_id.into(),
                    super::layoutengine::LayoutData {
                        computed_x: content_start_x + item.x,
                        computed_y: content_start_y + item.y,
                        actual_width: if is_row { item.main_size } else { item.cross_size },
                        actual_height: if is_row { item.cross_size } else { item.main_size },
                    },
                );
            }
        }
        
        let final_content_height = if element_style.height.is_some() {
            content_area_height
        } else {
            total_cross_size
        };

        let final_width = container_width;
        let final_height = final_content_height + padding.top + padding.bottom + border_top + border_bottom;

        self.layout.insert(
            key,
            super::layoutengine::LayoutData {
                computed_x: available_space.x + element_style.margin.left,
                computed_y: available_space.y + element_style.margin.top,
                actual_width: final_width,
                actual_height: final_height,
            },
        );

        Rect {
            x: available_space.x,
            y: available_space.y,
            width: final_width + element_style.margin.left + element_style.margin.right,
            height: final_height + element_style.margin.top + element_style.margin.bottom,
        }
    }
}


// Helper to convert AlignSelf to AlignItems
impl From<AlignSelf> for AlignItems {
    fn from(align_self: AlignSelf) -> Self {
        match align_self {
            AlignSelf::FlexStart => AlignItems::FlexStart,
            AlignSelf::FlexEnd => AlignItems::FlexEnd,
            AlignSelf::Center => AlignItems::Center,
            AlignSelf::Stretch => AlignItems::Stretch,
            AlignSelf::Baseline => AlignItems::Baseline,
            AlignSelf::Auto => panic!("Cannot convert AlignSelf::Auto to AlignItems"),
        }
    }
}