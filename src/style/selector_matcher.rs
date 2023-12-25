use bevy::prelude::*;
use bevy::{ecs::entity::Entity, utils::HashMap};
use bevy_mod_picking::backend::HitData;
use bevy_mod_picking::pointer::PointerId;

use crate::{ElementClasses, Selector};

pub struct SelectorMatcher<'w, 's, 'h> {
    classes_query: &'h Query<'w, 's, Ref<'static, ElementClasses>>,
    parent_query: &'h Query<'w, 's, &'static Parent, (With<Node>, With<Visibility>)>,
    children_query: &'h Query<'w, 's, &'static Children, (With<Node>, With<Visibility>)>,
    hover_map: &'h HashMap<PointerId, HashMap<Entity, HitData>>,
}

impl<'w, 's, 'h> SelectorMatcher<'w, 's, 'h> {
    pub(crate) fn new(
        query: &'h Query<'w, 's, Ref<'static, ElementClasses>>,
        parent_query: &'h Query<'w, 's, &'static Parent, (With<Node>, With<Visibility>)>,
        children_query: &'h Query<'w, 's, &'static Children, (With<Node>, With<Visibility>)>,
        hover_map: &'h HashMap<PointerId, HashMap<Entity, HitData>>,
    ) -> Self {
        Self {
            classes_query: query,
            parent_query,
            children_query,
            hover_map,
        }
    }

    /// True if the given entity, or a descendant of it, is in the hover map for PointerId::Mouse.
    ///
    /// This is used to determine whether to apply the :hover pseudo-class.
    pub fn is_hovering(&self, e: &Entity) -> bool {
        match self.hover_map.get(&PointerId::Mouse) {
            Some(map) => map.iter().any(|(mut ha, _)| loop {
                if ha == e {
                    return true;
                }
                match self.parent_query.get(*ha) {
                    Ok(parent) => ha = parent,
                    _ => return false,
                }
            }),
            None => false,
        }
    }

    /// True if this entity is the first child of its parent.
    pub fn is_first_child(&self, entity: &Entity) -> bool {
        match self.parent_query.get(*entity) {
            Ok(parent) => match self.children_query.get(parent.get()) {
                Ok(children) => children.first() == Some(entity),
                _ => false,
            },
            _ => false,
        }
    }

    /// True if this entity is the last child of its parent.
    pub fn is_last_child(&self, entity: &Entity) -> bool {
        match self.parent_query.get(*entity) {
            Ok(parent) => match self.children_query.get(parent.get()) {
                Ok(children) => children.last() == Some(entity),
                _ => false,
            },
            _ => false,
        }
    }

    /// Given an array of match params representing the element's ancestor chain, match the
    /// selector expression with the params.
    pub(crate) fn selector_match<'b>(&self, selector: &'b Selector, entity: &Entity) -> bool {
        match selector {
            Selector::Accept => true,
            Selector::Class(cls, next) => match self.classes_query.get(*entity) {
                Ok(classes) => classes.0.contains(cls) && self.selector_match(next, entity),
                _ => false,
            },
            Selector::Hover(next) => self.is_hovering(&entity) && self.selector_match(next, entity),
            Selector::FirstChild(next) => {
                self.is_first_child(&entity) && self.selector_match(next, entity)
            }
            Selector::LastChild(next) => {
                self.is_last_child(&entity) && self.selector_match(next, entity)
            }
            Selector::Current(next) => self.selector_match(next, entity),
            Selector::Parent(next) => match self.parent_query.get(*entity) {
                Ok(parent) => self.selector_match(next, &parent.get()),
                _ => false,
            },
            Selector::Either(opts) => opts.iter().any(|next| self.selector_match(next, entity)),
        }
    }
}
