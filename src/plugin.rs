use bevy::{prelude::*, utils::HashSet};

use crate::{
    style::{ComputedStyle, UpdateComputedStyle},
    view::ElementStyles,
    view::TrackedResources,
    ElementClasses, ElementContext, ViewHandle,
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
    let mut q = world.query::<(Entity, &TrackedResources)>();
    let mut v = vec![];
    for (e, tracked) in q.iter(world) {
        if tracked.data.iter().any(|x| x.is_changed(world)) {
            v.push(e);
        }
    }

    // force build every view that just got spawned
    let mut qf = world.query_filtered::<Entity, Added<ViewHandle>>();
    for e in qf.iter(world) {
        v.push(e);
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
        Ref<ElementStyles>,
        Ref<ElementClasses>,
        Option<&Parent>,
    )>,
) {
    for (entity, styles, _, _) in query.iter() {
        let mut changed = styles.is_changed();
        if !changed && styles.ancestor_depth > 0 {
            // Search ancestors to see if any have changed.
            let mut e = entity;
            for _ in 0..styles.ancestor_depth {
                match query.get(e) {
                    Ok((_, _, a_classes, a_parent)) => {
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

            // Build list of class components
            let mut classes: Vec<HashSet<String>> = Vec::with_capacity(styles.ancestor_depth);
            let mut e = entity;
            for _ in 0..styles.ancestor_depth {
                match query.get(e) {
                    Ok((_, _, a_classes, a_parent)) => {
                        classes.push(a_classes.0.to_owned());
                        if a_parent.is_none() {
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

            let mut computed = ComputedStyle::new();
            for ss in styles.styles.iter() {
                ss.apply_to(&mut computed, classes.as_slice());
            }
            commands.add(UpdateComputedStyle { entity, computed });
        }
    }
}
