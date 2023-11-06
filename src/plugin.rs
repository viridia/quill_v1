use bevy::prelude::*;

use crate::{view::TrackedResources, ElementContext, ViewHandle};

pub struct QuillPlugin;

impl Plugin for QuillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_views);
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
