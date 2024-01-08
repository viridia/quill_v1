use bevy::{prelude::*, utils::HashSet};
use bevy_mod_picking::prelude::EventListenerPlugin;

use crate::{
    animate_bg_colors, animate_border_colors, animate_layout, animate_transforms,
    handle_scroll_events,
    presenter_state::{PresenterGraphChanged, PresenterStateChanged},
    tracked_resources::TrackedResources,
    tracking::TrackedComponents,
    update::update_styles,
    update_scroll_positions, BuildContext, ScrollWheel, ViewHandle,
};

/// Plugin which initializes the Quill library.
pub struct QuillPlugin;

impl Plugin for QuillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (render_views, update_styles).chain(),
                animate_transforms,
                animate_bg_colors,
                animate_border_colors,
                animate_layout,
                update_scroll_positions,
                handle_scroll_events,
            ),
        )
        .add_plugins(EventListenerPlugin::<ScrollWheel>::default())
        .add_event::<ScrollWheel>();
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
        if !v.contains(&e)
            && tracked_components.data.iter().any(|(e, cid)| {
                world
                    .get_entity(*e)
                    .map(|ent| {
                        ent.get_change_ticks_by_id(*cid)
                            .map(|ct| ct.is_changed(last_run, this_run))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
        {
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

        // This means that either a presenter was just added, or its props got modified by a parent.
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
                let mut entt = world.entity_mut(e);
                // Clear tracking lists for presenters to be re-rendered.
                if let Some(mut tracked_resources) = entt.get_mut::<TrackedResources>() {
                    tracked_resources.data.clear();
                }
                if let Some(mut tracked_components) = entt.get_mut::<TrackedComponents>() {
                    tracked_components.data.clear();
                }

                // Clone the ViewHandle so we can call build() on it.
                let Some(view_handle) = entt.get_mut::<ViewHandle>() else {
                    continue;
                };
                let inner = view_handle.inner.clone();
                let mut ec = BuildContext::new(world, e);
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
        if changed_entities.is_empty() {
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
            let mut vc = BuildContext::new(world, e);
            inner.lock().unwrap().attach(&mut vc, e);
        }
    }
}
