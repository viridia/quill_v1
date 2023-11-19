use bevy::{prelude::*, utils::HashSet};
use bevy_mod_picking::focus::{HoverMap, PreviousHoverMap};

use crate::{
    style::{ComputedStyle, UpdateComputedStyle},
    ElementClasses, ElementContext, ElementStyles, SelectorMatcher, TrackedLocals,
    TrackedResources, ViewHandle,
};

pub struct QuillPlugin;

impl Plugin for QuillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (render_views, update_styles).chain());
    }
}

// Updating views needs to be split in 3 phases for borrowing issues
// Phase 1: Identify which ViewRoot Entity needs to re-render
// Phase 2: Use Option::take() to remove the ViewRoot::handle from the World
// Phase 3: Use the taken handle and call AnyViewState::build() on it.
//          Since the handle isn't part of the World we can freely pass a mutable reference to the World.
fn render_views(world: &mut World) {
    // phase 1
    let mut v = HashSet::new();
    let mut q = world.query::<(Entity, &TrackedResources)>();
    for (e, tracked_resources) in q.iter(world) {
        if tracked_resources.data.iter().any(|x| x.is_changed(world)) {
            v.insert(e);
        }
    }

    let mut q = world.query::<(Entity, &mut TrackedLocals)>();
    for (e, mut tracked_locals) in q.iter_mut(world) {
        if TrackedLocals::is_changed(&tracked_locals) {
            v.insert(e);
            tracked_locals.reset_changed();
        }
    }

    // force build every view that just got spawned
    let mut qf = world.query_filtered::<Entity, Added<ViewHandle>>();
    for e in qf.iter(world) {
        v.insert(e);
    }

    // phase 2
    let mut v2 = vec![];
    for e in v {
        if let Some(mut view_root) = world.get_mut::<ViewHandle>(e) {
            // take the view handle out of the world
            v2.push((e, view_root.inner.take()));
        }
    }

    // phase 3
    for (e, handle) in v2 {
        let Some(mut handle) = handle else {
            continue;
        };
        let mut ec = ElementContext { world, entity: e };
        handle.build(&mut ec, e);

        if let Some(mut view_root) = world.get_mut::<ViewHandle>(e) {
            // Now that we are done with the handle we can put it back in the world
            view_root.inner = Some(handle);
        }
    }
}

fn update_styles(
    mut commands: Commands,
    query: Query<(
        Entity,
        Ref<'static, ElementStyles>,
        Ref<'static, ElementClasses>,
        Option<&'static Parent>,
    )>,
    hover_map: Res<HoverMap>,
    hover_map_prev: Res<PreviousHoverMap>,
) {
    let matcher = SelectorMatcher::new(&query, &hover_map.0);
    let matcher_prev = SelectorMatcher::new(&query, &hover_map_prev.0);
    for (entity, styles, _, _) in query.iter() {
        let mut changed = styles.is_changed();
        if !changed && styles.selector_depth > 0 {
            // Search ancestors to see if any have changed.
            // We want to know if either the class list or the hover state has changed.
            let mut e = entity;
            for _ in 0..styles.selector_depth {
                match query.get(e) {
                    Ok((_, _, a_classes, a_parent)) => {
                        if styles.uses_hover
                            && matcher.is_hovering(&e) != matcher_prev.is_hovering(&e)
                        {
                            changed = true;
                        }
                        if a_classes.is_changed() {
                            changed = true;
                            break;
                        } else if a_parent.is_none() {
                            break;
                        } else {
                            e = a_parent.unwrap().get();
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
