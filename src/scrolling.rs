use bevy::prelude::*;

/// Component that enables scrolling on an element
#[derive(Component, Default)]
pub struct Scrolling {
    /// Whether scrolling is enabled along the X-axis
    pub enable_x: bool,

    /// Whether scrolling is enabled along the Y-axis
    pub enable_y: bool,

    /// Scroll amount along X-axis
    pub scroll_left: f32,

    /// Scroll amount along Y-axis
    pub scroll_top: f32,

    /// Size of scrolling content along X-axis
    pub scroll_width: f32,

    /// Size of scrolling content along Y-axis
    pub scroll_height: f32,

    /// Scrollbar entity for x-axis
    pub scrollbar_x: Option<Entity>,

    /// Scrollbar entity for y-axis
    pub scrollbar_y: Option<Entity>,
}

#[derive(Component, Default)]
pub struct ScrollContent;

pub fn scroll_system(
    mut query: Query<(&Node, &mut Scrolling, &mut Transform, &GlobalTransform)>,
    mut content_query: Query<(
        &Node,
        &mut ScrollContent,
        &mut Transform,
        &GlobalTransform,
        &Parent,
    )>,
) {
    for (node, mut scrolling, mut transform, gt) in query.iter_mut() {
        // TODO: We need a separate "ScrollContent" element.
        // Measure size and update scroll width and height
        let scroll_size = node.logical_rect(gt);
        if scrolling.enable_x {
            scrolling.scroll_width = scroll_size.width();
        }

        if scrolling.enable_y {
            scrolling.scroll_height = scroll_size.width();
        }

        print!(
            "Scrolling: {} {}",
            scrolling.scroll_width, scrolling.scroll_height
        );
        // width.value = ev.value.clamp(100., node_width - 100.);
    }
    //     clamp scroll position
    //     adjust transform
    //     adjust scrollbar(s)
}
