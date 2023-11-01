use bevy::prelude::*;

use super::view_root::ViewRootResource;

pub struct QuillPlugin;

impl Plugin for QuillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_views);
    }
}

pub fn render_views(world: &mut World) {
    // TODO: figure out how to put the ViewRoot in a component rather than a resource.
    // for mut root in world.query::<&mut ViewRoot>().iter_mut(world) {
    //     // roots.push(root.handle.clone())
    //     root.build(world);
    // }
    world.resource_scope(|world, mut res: Mut<ViewRootResource>| {
        res.0.build(world);
    });
}
