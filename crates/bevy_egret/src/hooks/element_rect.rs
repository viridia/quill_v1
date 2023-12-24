use bevy::prelude::*;
use bevy_quill::prelude::*;

/// Trait which adds `use_element_rect` to [`Cx`].
pub trait ElementRectApi {
    fn use_element_rect(&mut self, id: Entity) -> Rect;
}

impl<'w, 'p, Props> ElementRectApi for Cx<'w, 'p, Props> {
    fn use_element_rect(&mut self, id: Entity) -> Rect {
        match (
            self.use_component::<Node>(id),
            self.use_component_untracked::<GlobalTransform>(id),
        ) {
            (Some(ref node), Some(ref transform)) => node.logical_rect(transform),
            _ => Rect::new(0., 0., 0., 0.),
        }
    }
}
