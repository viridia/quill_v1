use bevy::prelude::*;

use super::{
    view::{Cx, ElementContext, TrackedResources},
    NodeSpan, View,
};

#[derive(Resource)]
pub struct ViewRootResource(pub ViewRoot);

pub struct ViewRoot {
    pub handle: Box<dyn AnyViewState>,
}

impl ViewRoot {
    /// Construct a new ViewRoot from a presenter and props.
    pub fn new<V: View + 'static, Props: Send + Sync + 'static>(
        presenter: fn(cx: Cx<Props>) -> V,
        props: Props,
    ) -> Self {
        Self {
            handle: Box::new(ViewState::new(presenter, props)),
        }
    }

    /// Return the count of top-level UiNodes
    pub fn count(&self) -> usize {
        self.handle.count()
    }

    /// Rebuild the UiNodes.
    pub fn build(&mut self, world: &mut World) {
        let mut ec = ElementContext { world };
        self.handle.build(&mut ec);
    }
}

// pub struct ViewHandle {
//     pub(crate) state: Box<dyn AnyViewHandle>,
// }

pub struct ViewState<V: View, Props: Send + Sync> {
    presenter: fn(cx: Cx<Props>) -> V,
    nodes: NodeSpan,
    props: Props,
    needs_rebuild: bool,
    entity: Option<Entity>,
}

impl<V: View, Props: Send + Sync> ViewState<V, Props> {
    pub fn new(presenter: fn(cx: Cx<Props>) -> V, props: Props) -> Self {
        Self {
            presenter,
            nodes: NodeSpan::Empty,
            props,
            needs_rebuild: true,
            // TODO generate an id based on something
            entity: None,
        }
    }
}

pub trait AnyViewState: Send + Sync {
    fn count(&self) -> usize;
    fn build<'w>(&mut self, cx: &'w mut ElementContext<'w>);
}

impl<V: View, Props: Send + Sync> AnyViewState for ViewState<V, Props> {
    fn count(&self) -> usize {
        self.nodes.count()
    }

    fn build<'w>(&mut self, ecx: &'w mut ElementContext<'w>) {
        // initialize entity
        let entity = match self.entity {
            Some(id) => id,
            None => {
                let entity = ecx.world.spawn(TrackedResources::default()).id();
                self.entity = Some(entity);
                entity
            }
        };

        let Some(tracked_resources) = ecx.world.get::<TrackedResources>(entity) else {
            println!("TrackedResources not found for ViewState");
            return;
        };
        // Check if any resource used by this ViewState has changed
        self.needs_rebuild = self.needs_rebuild
            || tracked_resources
                .data
                .iter()
                .any(|x| x.is_changed(ecx.world));

        if self.needs_rebuild {
            // println!("rebuild");

            // reset the tracked resources
            ecx.world
                .get_mut::<TrackedResources>(entity)
                .unwrap()
                .data
                .clear();

            self.needs_rebuild = false;
            let cx = Cx::<Props> {
                sys: ecx,
                props: &self.props,
                entity,
            };
            let v = (self.presenter)(cx);
            self.nodes = v.build(ecx, &self.nodes);
        }
    }
}
