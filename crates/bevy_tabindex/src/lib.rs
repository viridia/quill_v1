use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Query, SystemParam},
    },
    hierarchy::{Children, Parent},
    log::*,
    ui::Node,
};

/// A component which indicates that an entity wants to participate in tab navigation.
///
/// The rules of tabbing are derived from the HTML specification, and are as follows:
/// * An index >= 0 means that the entity is tabbable via sequential navigation.
///   The order of tabbing is determined by the index, with lower indices being tabbed first.
///   If two entities have the same index, then the order is determined by the order of
///   the entities in the ECS hierarchy (as determined by Parent/Child).
/// * An index < 0 means that the entity is not focusable via sequential navigation, but
///   can still be focused via direct selection.
///
/// Note that you must also add the [`TabGroup`] component to the entity's ancestor in order
/// for this component to have any effect.
#[derive(Debug, Default, Component, Copy, Clone)]
pub struct TabIndex(pub i32);

/// A component used to mark a tree of entities as containing tabbable elements.
#[derive(Debug, Default, Component, Copy, Clone)]
pub struct TabGroup {
    /// The order of the tab group relative to other tab groups.
    pub order: i32,

    /// Whether this is a 'trapped' group. If true, then tabbing within the group (that is,
    /// if the current focus entity is a child of this group) will cycle through the children
    /// of this group. If false, then tabbing within the group will cycle through all non-trapped
    /// tab groups.
    pub trapped: bool,
}

/// Resource which contains the current list of tabbable entities as a flat vector.
/// If the current focus entity is inside a trapped group, then this list only contains
/// the entities in that group. Otherwise, it contains all tabbable entities that are not
/// in trapped groups.
// #[derive(Debug, Default, Resource)]
// pub struct FocusableEntities {
//     /// The list of tabbable entities.
//     pub entities: Vec<Entity>,
// }

/// An injectable object that provides tab navigation functionality.
#[doc(hidden)]
#[derive(SystemParam)]
#[allow(clippy::type_complexity)]
pub struct TabNavigation<'w, 's> {
    // Query for tab groups.
    tabgroup: Query<'w, 's, (Entity, &'static TabGroup, &'static Children)>,
    // Query for tab indices.
    tabindex: Query<
        'w,
        's,
        (Entity, Option<&'static TabIndex>, Option<&'static Children>),
        (With<Node>, Without<TabGroup>),
    >,
    // Query for parents.
    parent: Query<'w, 's, &'static Parent, With<Node>>,
}

impl TabNavigation<'_, '_> {
    /// Navigate to the next focusable entity.
    ///
    /// Arguments:
    /// * `focus`: The current focus entity. If `None`, then the first focusable entity is returned,
    ///    unless `reverse` is true, in which case the last focusable entity is returned.
    /// * `reverse`: Whether to navigate in reverse order.
    pub fn navigate(&self, focus: Option<Entity>, reverse: bool) -> Option<Entity> {
        // If there are no tab groups, then there are no focusable entities.
        if self.tabgroup.is_empty() {
            warn!("No tab groups found");
            return None;
        }

        // Start by identifying which tab group we are in. Mainly what we want to know is if
        // we're in a trapped group.
        let mut tabgroup: Option<(Entity, &TabGroup)> = None;
        let mut entity = focus;
        while let Some(ent) = entity {
            if let Ok((tg_entity, tg, _)) = self.tabgroup.get(ent) {
                tabgroup = Some((tg_entity, tg));
            }
            // Search up
            entity = self.parent.get(ent).ok().map(|parent| parent.get());
        }

        // List of all focusable entities found.
        let mut focusable: Vec<(Entity, TabIndex)> = Vec::with_capacity(self.tabindex.iter().len());

        match tabgroup {
            Some((tg_entity, tg)) if tg.trapped => {
                // We're in a trapped tab group, then gather all tab indices in that group.
                self.gather_focusable(&mut focusable, tg_entity);
            }
            _ => {
                // Otherwise, gather all tab indices in all non-trapped tab groups.
                let mut tab_groups: Vec<(Entity, TabGroup)> = self
                    .tabgroup
                    .iter()
                    .filter(|(_, tg, _)| !tg.trapped)
                    .map(|(e, tg, _)| (e, *tg))
                    .collect();
                // Stable sort by group order
                tab_groups.sort_by(compare_tab_groups);

                // Search group descendants
                tab_groups.iter().for_each(|(tg_entity, _)| {
                    self.gather_focusable(&mut focusable, *tg_entity);
                })
            }
        }

        if focusable.is_empty() {
            warn!("No focusable entities found");
            return None;
        }

        // Stable sort by tabindex
        focusable.sort_by(compare_tab_indices);

        let index = focusable.iter().position(|e| Some(e.0) == focus);
        let count = focusable.len();
        let next = match (index, reverse) {
            (Some(idx), false) => (idx + 1).rem_euclid(count),
            (Some(idx), true) => (idx + count - 1).rem_euclid(count),
            (None, false) => 0,
            (None, true) => count - 1,
        };
        focusable.get(next).map(|(e, _)| e).copied()
    }

    fn gather_focusable(&self, out: &mut Vec<(Entity, TabIndex)>, parent: Entity) {
        if let Ok((entity, tabindex, children)) = self.tabindex.get(parent) {
            if let Some(tabindex) = tabindex {
                if tabindex.0 >= 0 {
                    out.push((entity, *tabindex));
                }
            }
            if let Some(children) = children {
                for child in children.iter() {
                    // Don't recurse into tab groups
                    if self.tabgroup.get(*child).is_err() {
                        self.gather_focusable(out, *child);
                    }
                }
            }
        } else if let Ok((_, tabgroup, children)) = self.tabgroup.get(parent) {
            if !tabgroup.trapped {
                for child in children.iter() {
                    self.gather_focusable(out, *child);
                }
            }
        }
    }
}

fn compare_tab_groups(a: &(Entity, TabGroup), b: &(Entity, TabGroup)) -> std::cmp::Ordering {
    a.1.order.cmp(&b.1.order)
}

// Stable sort which compares by tab index
fn compare_tab_indices(a: &(Entity, TabIndex), b: &(Entity, TabIndex)) -> std::cmp::Ordering {
    a.1 .0.cmp(&b.1 .0)
}
