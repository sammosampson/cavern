use crate::prelude::*;

pub fn create_editor_entities_list() -> EditorGraphNode {
    create_editor_list(EditorItems::EntityId.into())
}

pub fn create_editor_position() -> EditorGraphNode {
    let item = EditorItems::Position.into();
    create_editor_vector(item, "Position")
}

pub fn create_editor_velocity() -> EditorGraphNode {
    let item = EditorItems::Velocity.into();
    create_editor_vector(item, "Velocity")
}

pub fn create_editor_maximum_velocity() -> EditorGraphNode {
    let item = EditorItems::MaximumVelocity.into();
    create_editor_float(item, "Max. Velocity")
}

pub fn create_editor_collision_box() -> EditorGraphNode {
    let item = EditorItems::CollisionBox.into();
    create_editor_float(item, "Collision Box")
}

pub fn create_editor_tab(children: Vec::<EditorGraphNode>) -> EditorGraphNode {
    create_editor_container(vec!(
        create_extract_window_button(),
        create_editor_container(children)
    ))
}

fn create_editor_list(item: EditorGraphDataItem) -> EditorGraphNode {
    create_editor_container(vec!(
        create_editor_separator(),
        create_editor_scroll_area(
            vec!(create_editor_list_items(item))
        ),
        create_editor_separator()
    ))
}

fn create_extract_window_button() -> EditorGraphNode {
    let item = EditorItems::ExtractWindow.into();
    create_button(item, ">>")
}

fn create_editor_container(children: Vec::<EditorGraphNode>) -> EditorGraphNode {
    EditorGraphNode::Container { children }
}

fn create_editor_separator() -> EditorGraphNode {
    EditorGraphNode::Seperator
}

fn create_editor_scroll_area(children: Vec::<EditorGraphNode>) -> EditorGraphNode {
    EditorGraphNode::ScrollArea { children }
}

fn create_editor_list_items(item: EditorGraphDataItem) -> EditorGraphNode {
    EditorGraphNode::EntityListItems {
        item
    }
}

fn create_button(item: EditorGraphDataItem, title: &str) -> EditorGraphNode {
    EditorGraphNode::Button {
        item,
        title: title.to_string()
    }
}

fn create_editor_vector(item: EditorGraphDataItem, title: &str) -> EditorGraphNode {
    EditorGraphNode::EntityVector {
        item,
        title: title.to_string()
    }
}

fn create_editor_dimensions(item: EditorGraphDataItem, title: &str) -> EditorGraphNode {
    EditorGraphNode::EntityDimensions {
        item,
        title: title.to_string()
    }
}

fn create_editor_float(item: EditorGraphDataItem, title: &str) -> EditorGraphNode {
    EditorGraphNode::EntityFloat {
        item,
        title: title.to_string()
    }
}

#[derive(Clone)]
pub enum EditorGraphNode {
    Container { children: Vec<EditorGraphNode> },
    Seperator,
    ScrollArea { children: Vec<EditorGraphNode> },
    Button { item: EditorGraphDataItem, title: String },
    EntityListItems { item: EditorGraphDataItem },
    EntityVector { item: EditorGraphDataItem, title: String },
    EntityDimensions { item: EditorGraphDataItem, title: String },
    EntityFloat { item: EditorGraphDataItem, title: String },
}
