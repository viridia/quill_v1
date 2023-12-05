use bevy::{prelude::*, utils::HashSet};
use bevy_mod_picking::focus::{HoverMap, PreviousHoverMap};

use crate::{
    animate_bg_colors, animate_border_colors, animate_transforms,
    presenter_state::{PresenterGraphChanged, PresenterStateChanged},
    style::{ComputedStyle, UpdateComputedStyle},
    tracked_resources::TrackedResources,
    tracking::TrackedComponents,
    ElementClasses, ElementStyles, SelectorMatcher, TrackedLocals, ViewContext, ViewHandle,
};

/// Plugin which initializes the Quill library.
pub struct QuillPlugin;

impl Plugin for QuillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                render_views,
                update_styles,
                animate_transforms,
                animate_bg_colors,
                animate_border_colors,
            )
                .chain(),
        );
    }
}

const MAX_DIVERGENCE_CT: usize = 30;

// Updating views needs to be split in 3 phases for borrowing issues
// Phase 1: Identify which ViewRoot Entity needs to re-render
// Phase 2: Use Option::take() to remove the ViewRoot::handle from the World. Use the taken handle
//          and call AnyViewState::build() on it. Since the handle isn't part of the World we can
//          freely pass a mutable reference to the World.
fn render_views(world: &mut World) {
    let mut divergence_ct: usize = 0;
    let mut prev_change_ct: usize = 0;
    let last_run = world.last_change_tick();
    let this_run = world.change_tick();

    let mut v = HashSet::new();

    // Scan changed resources
    let mut q = world.query::<(Entity, &mut TrackedResources)>();
    for (e, tracked_resources) in q.iter(world) {
        if tracked_resources.data.iter().any(|x| x.is_changed(world)) {
            v.insert(e);
        }
    }

    // Scan changed components
    let mut q = world.query::<(Entity, &mut TrackedComponents)>();
    for (e, tracked_components) in q.iter(world) {
        if tracked_components.data.iter().any(|(e, cid)| {
            world
                .get_entity(*e)
                .map(|ent| {
                    ent.get_change_ticks_by_id(*cid)
                        .map(|ct| ct.is_changed(last_run, this_run))
                        .unwrap_or(false)
                })
                .unwrap_or(false)
        }) {
            v.insert(e);
        }
    }

    // force build every view that just got spawned
    let mut qf = world.query_filtered::<Entity, Added<ViewHandle>>();
    for e in qf.iter(world) {
        v.insert(e);
    }

    loop {
        // This is inside a loop because rendering may trigger further changes.

        // Scan changed locals
        let mut q = world.query::<(Entity, &mut TrackedLocals)>();
        for (e, tracked_locals) in q.iter_mut(world) {
            if TrackedLocals::cas(&tracked_locals) {
                v.insert(e);
            }
        }

        // force build every view that just got spawned
        let mut qf =
            world.query_filtered::<Entity, (With<ViewHandle>, With<PresenterStateChanged>)>();
        for e in qf.iter_mut(world) {
            v.insert(e);
        }

        for e in v.iter() {
            world.entity_mut(*e).remove::<PresenterStateChanged>();
        }

        // Most of the time changes will converge, that is, the number of changed presenters
        // decreases each time through the loop. A "divergence" is when that fails to happen.
        // We tolerate a maximum number of divergences before giving up.
        let change_ct = v.len();
        if change_ct >= prev_change_ct {
            divergence_ct += 1;
            if divergence_ct > MAX_DIVERGENCE_CT {
                panic!("Reactions failed to converge, num changes: {}", change_ct);
            }
        }
        prev_change_ct = change_ct;

        // phase 2
        if change_ct > 0 {
            for e in v.drain() {
                // Clear tracking lists for presenters to be re-rendered.
                if let Some(mut tracked_resources) = world.get_mut::<TrackedResources>(e) {
                    tracked_resources.data.clear();
                }
                if let Some(mut tracked_components) = world.get_mut::<TrackedComponents>(e) {
                    tracked_components.data.clear();
                }

                let Some(view_handle) = world.get_mut::<ViewHandle>(e) else {
                    continue;
                };
                let inner = view_handle.inner.clone();
                let mut ec = ViewContext::new(world, e);
                inner.lock().unwrap().build(&mut ec, e);
            }
        } else {
            break;
        }
    }

    // phase 3
    loop {
        let mut qf = world.query_filtered::<Entity, With<PresenterGraphChanged>>();
        let changed_entities: Vec<Entity> = qf.iter(world).collect();
        if changed_entities.len() == 0 {
            break;
        }
        // println!("Entities changed: {}", changed_entities.len());
        for e in changed_entities {
            // println!("PresenterGraphChanged {:?}", e);
            let mut ent = world.entity_mut(e);
            ent.remove::<PresenterGraphChanged>();
            let Some(view_handle) = world.get_mut::<ViewHandle>(e) else {
                continue;
            };
            let inner = view_handle.inner.clone();
            let mut vc = ViewContext::new(world, e);
            inner.lock().unwrap().attach(&mut vc, e);
        }
    }
}

fn update_styles(
    mut commands: Commands,
    query: Query<(
        Entity,
        Ref<'static, ElementStyles>,
        Ref<'static, ElementClasses>,
    )>,
    parent_query: Query<&'static Parent, Or<(With<ElementStyles>, With<Text>)>>,
    hover_map: Res<HoverMap>,
    hover_map_prev: Res<PreviousHoverMap>,
) {
    let matcher = SelectorMatcher::new(&query, &parent_query, &hover_map.0);
    let matcher_prev = SelectorMatcher::new(&query, &parent_query, &hover_map_prev.0);
    for (entity, styles, _) in query.iter() {
        let mut changed = styles.is_changed();
        if !changed && styles.selector_depth > 0 {
            // Search ancestors to see if any have changed.
            // We want to know if either the class list or the hover state has changed.
            let mut e = entity;
            for _ in 0..styles.selector_depth {
                match query.get(e) {
                    Ok((_, _, a_classes)) => {
                        if styles.uses_hover
                            && matcher.is_hovering(&e) != matcher_prev.is_hovering(&e)
                        {
                            changed = true;
                        }
                        if a_classes.is_changed() {
                            changed = true;
                            break;
                        } else {
                            match parent_query.get(e) {
                                Ok(parent) => e = **parent,
                                _ => break,
                            }
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
        if changed {
            // Compute computed style.
            let mut computed = ComputedStyle::new();
            for ss in styles.styles.iter() {
                ss.apply_to(&mut computed, &matcher, &entity);
            }
            commands.add(UpdateComputedStyle { entity, computed });
        }
    }
}
