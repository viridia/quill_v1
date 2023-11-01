use bevy::prelude::*;

use crate::{
    view::{self, TrackedResources},
    view_root, ElementContext, ViewRoot, ViewStateComp,
};

use super::view_root::ViewRootResource;

pub struct QuillPlugin;

impl Plugin for QuillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (render_views, update_views));
    }
}

pub fn render_views(world: &mut World) {
    // TODO: figure out how to put the ViewRoot in a component rather than a resource.
    // for mut root in world.query::<&mut ViewRoot>().iter_mut(world) {
    //     // roots.push(root.handle.clone())
    //     root.build(world);
    // }

    // world.resource_scope(|world, mut res: Mut<ViewRootResource>| {
    //     res.0.build(world);
    // });
}

fn update_views(world: &mut World) {
    let mut q = world.query::<(Entity, &TrackedResources)>();
    let mut v = vec![];
    for (e, tracked) in q.iter(world) {
        if tracked.data.iter().any(|x| x.is_changed(world)) {
            v.push(e);
        }
    }

    // force build every view that just got spawned
    let mut qf = world.query_filtered::<Entity, Added<ViewRoot>>();
    for e in qf.iter(world) {
        v.push(e);
    }

    let mut v2 = vec![];
    for e in v {
        if let Some(mut view_root) = world.get_mut::<ViewRoot>(e) {
            // take the view handle out of the world
            v2.push((e, view_root.handle.take()));
        }
    }

    for (e, handle) in v2 {
        let Some(mut handle) = handle else {
            continue;
        };
        let mut ec = ElementContext { world };
        handle.build(&mut ec, e);

        if let Some(mut view_root) = world.get_mut::<ViewRoot>(e) {
            // Now that we are done with the handle we can put it back in the world
            view_root.handle = Some(handle);
        }
    }
}
