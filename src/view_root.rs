use bevy::prelude::*;

use super::{
    view::{Cx, ElementContext},
    NodeSpan, View,
};

#[derive(Resource)]
pub struct ViewRootResource(pub ViewRoot);

#[derive(Component)]
pub struct ViewRoot {
    pub handle: Option<Box<dyn AnyViewState>>,
}

impl ViewRoot {
    /// Construct a new ViewRoot from a presenter and props.
    pub fn new<V: View + 'static, Props: Send + Sync + 'static + Clone>(
        presenter: fn(cx: Cx<Props>) -> V,
        props: Props,
    ) -> Self {
        Self {
            handle: Some(Box::new(ViewState::new(presenter, props))),
        }
    }

    /// Return the count of top-level UiNodes
    pub fn count(&self) -> usize {
        self.handle.as_ref().unwrap().count()
    }

    /// Rebuild the UiNodes.
    pub fn build(&mut self, world: &mut World, entity: Entity) {
        let mut ec = ElementContext { world };
        self.handle.as_mut().unwrap().build(&mut ec, entity);
    }
}

// pub struct ViewHandle {
//     pub(crate) state: Box<dyn AnyViewHandle>,
// }

#[derive(Component)]
pub struct ViewStateComp<V: View, Props> {
    pub presenter: fn(cx: Cx<Props>) -> V,
    pub nodes: NodeSpan,
    pub props: Props,
}

#[derive(Component)]
struct NeedsRebuild;

pub struct ViewState<V: View, Props: Send + Sync> {
    presenter: fn(cx: Cx<Props>) -> V,
    nodes: NodeSpan,
    props: Props,
}

impl<V: View, Props: Send + Sync> ViewState<V, Props> {
    pub fn new(presenter: fn(cx: Cx<Props>) -> V, props: Props) -> Self {
        Self {
            presenter,
            nodes: NodeSpan::Empty,
            props,
        }
    }
}

pub trait AnyViewState: Send + Sync {
    fn count(&self) -> usize;
    fn build<'w>(&mut self, cx: &'w mut ElementContext<'w>, entity: Entity);
}

impl<V: View, Props: Send + Sync + Clone> AnyViewState for ViewState<V, Props> {
    fn count(&self) -> usize {
        self.nodes.count()
    }

    fn build<'w>(&mut self, ecx: &'w mut ElementContext<'w>, entity: Entity) {
        let cx = Cx::<Props> {
            sys: ecx,
            props: &self.props,
            entity,
        };
        let v = (self.presenter)(cx);
        self.nodes = v.build(ecx, &self.nodes);
    }
}
