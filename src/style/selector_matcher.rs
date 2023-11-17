use bevy::prelude::*;
use bevy::{ecs::entity::Entity, utils::HashMap};
use bevy_mod_picking::backend::HitData;
use bevy_mod_picking::pointer::PointerId;

use crate::{ElementClasses, ElementStyles, Selector};

pub struct SelectorMatcher<'w, 's, 'h> {
    query: &'h Query<
        'w,
        's,
        (
            Entity,
            Ref<'static, ElementStyles>,
            Ref<'static, ElementClasses>,
            Option<&'static Parent>,
        ),
    >,
    hover_map: &'h HashMap<PointerId, HashMap<Entity, HitData>>,
}

impl<'w, 's, 'h> SelectorMatcher<'w, 's, 'h> {
    pub(crate) fn new(
        query: &'h Query<
            'w,
            's,
            (
                Entity,
                Ref<'static, ElementStyles>,
                Ref<'static, ElementClasses>,
                Option<&'static Parent>,
            ),
        >,
        hover_map: &'h HashMap<PointerId, HashMap<Entity, HitData>>,
    ) -> Self {
        Self { query, hover_map }
    }

    /// True if the given entity is in the hover map for PointerId::Mouse. This is a separate
    /// method because we need to be able to test hover status apart from selector matching.
    pub(crate) fn is_hovering(&self, e: &Entity) -> bool {
        match self.hover_map.get(&PointerId::Mouse) {
            Some(map) => map.contains_key(e),
            None => false,
        }
    }

    /// Given an array of match params representing the element's ancestor chain, match the
    /// selector expression with the params.
    pub(crate) fn selector_match<'b>(&self, selector: &'b Selector, entity: &Entity) -> bool {
        match selector {
            Selector::Accept => true,
            Selector::Class(cls, next) => match self.query.get(*entity) {
                Ok((_, _, classes, _)) => {
                    classes.0.contains(cls) && self.selector_match(next, entity)
                }
                Err(_) => false,
            },
            Selector::Hover(next) => self.is_hovering(&entity) && self.selector_match(next, entity),
            Selector::Current(next) => self.selector_match(next, entity),
            Selector::Parent(next) => match self.query.get(*entity) {
                Ok((_, _, _, Some(parent))) => self.selector_match(next, &parent.get()),
                _ => false,
            },
            Selector::Either(opts) => opts.iter().any(|next| self.selector_match(next, entity)),
        }
    }
}
