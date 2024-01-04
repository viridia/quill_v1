use std::cell::Cell;

use bevy::{
    prelude::*,
    text::{Text, TextStyle},
};

use crate::{presenter_state::*, ClassNames, Cx, StyleTuple, ViewHandle, ViewTuple};

use crate::node_span::NodeSpan;

use super::{
    bind::Bind, view_children::ViewChildren, view_classes::ViewClasses,
    view_insert_bundle::ViewInsertBundle, view_named::ViewNamed, view_styled::ViewStyled,
    view_with::ViewWith, view_with_memo::ViewWithMemo,
};

/// Passed to `build`, `update` and `raze` methods to give access to the world and the view entity.
pub struct BuildContext<'w> {
    pub(crate) world: &'w mut World,

    /// The entity which contains the PresenterState.
    pub(crate) entity: Entity,
}

impl<'w> BuildContext<'w> {
    pub(crate) fn new(world: &'w mut World, entity: Entity) -> Self {
        Self { world, entity }
    }

    /// Indicate that the shape of the display graph has changed.
    pub fn mark_changed_shape(&mut self) {
        self.world
            .entity_mut(self.entity)
            .insert(PresenterGraphChanged);
    }

    /// Return a modified [`ViewContext`] for a different entity.
    pub(crate) fn for_entity<'k>(&'k mut self, entity: Entity) -> BuildContext<'k>
    where
        'w: 'k,
    {
        BuildContext {
            world: &mut *self.world,
            entity,
        }
    }

    pub(crate) fn entity(&self, entity: Entity) -> EntityRef {
        self.world.entity(entity)
    }

    pub(crate) fn entity_mut(&mut self, entity: Entity) -> EntityWorldMut {
        self.world.entity_mut(entity)
    }
}

/// An object which generates one or more display nodes. Output of a presenter function
pub trait View: Send
where
    Self: Sized,
{
    /// The external state for this View.
    type State: Send;

    /// Return the span of UiNodes produced by this View.
    fn nodes(&self, vc: &BuildContext, state: &Self::State) -> NodeSpan;

    /// Construct and patch the tree of UiNodes produced by this view.
    /// This may also spawn child entities representing nested components.
    fn build(&self, vc: &mut BuildContext) -> Self::State;

    /// Update the internal state of this view, re-creating any UiNodes.
    fn update(&self, vc: &mut BuildContext, state: &mut Self::State);

    /// Attach child nodes to parents. This is typically called after generating/updating
    /// the display nodes (via build/rebuild), however it can also be called after rebuilding
    /// the display graph of nested presenters.
    fn assemble(&self, vc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.nodes(vc, state)
    }

    /// Recursively despawn any child entities that were created as a result of calling `.build()`.
    /// This calls `.raze()` for any nested views within the current view state.
    fn raze(&self, world: &mut World, state: &mut Self::State);

    /// Assign a human-readable debug name to the generated display node.
    fn named(self, name: &str) -> ViewNamed<Self> {
        ViewNamed::new(self, name)
    }

    /// Apply styles to this view.
    fn styled<S: StyleTuple>(self, styles: S) -> ViewStyled<Self> {
        ViewStyled::new(self, styles)
    }

    /// Set the class names for this View. This replaces any existing class names.
    fn class_names<'a, CN: ClassNames<'a>>(self, class_names: CN) -> ViewClasses<Self> {
        ViewClasses::new(self, class_names)
    }

    /// Inserts a default instance of the specified component or bundle to the display entity.
    /// This insertion occurs only once per output entity. The entity takes ownership of the
    /// bundle.
    ///
    /// This method will panic if you call this on a view which produces more than one output
    /// entity, since only one entity can take ownership.
    fn insert<B: Bundle>(self, component: B) -> ViewInsertBundle<Self, B> {
        ViewInsertBundle {
            inner: self,
            component: Cell::new(Some(component)),
        }
    }

    /// Sets up a callback which is called for each output UiNode generated by this `View`.
    /// Typically used to manipulate components on the entity. This is called each time the
    /// view is rebuilt.
    fn with<F: Fn(EntityWorldMut) -> () + Send>(self, callback: F) -> ViewWith<Self, F> {
        ViewWith {
            inner: self,
            callback,
        }
    }

    /// Sets up a callback which is called for each output UiNode generated by this `View`.
    /// Typically used to manipulate components on the entity. This callback is called when
    /// the view is first created, and then called again if either (a) the output entity
    /// changes, or (b) the value of the [`deps`] parameter is different than the previous
    /// call.
    fn with_memo<D: Clone + PartialEq + Send, F: Fn(EntityWorldMut) -> () + Send>(
        self,
        callback: F,
        deps: D,
    ) -> ViewWithMemo<Self, D, F> {
        ViewWithMemo {
            inner: self,
            callback,
            deps,
        }
    }

    /// Sets up a callback which is called for each output UiNode, but only when the node is first
    /// created.
    fn children<A: ViewTuple>(self, items: A) -> ViewChildren<Self, A> {
        ViewChildren { inner: self, items }
    }
}

/// View which renders nothing
impl View for () {
    type State = ();

    fn nodes(&self, _vc: &BuildContext, _state: &Self::State) -> NodeSpan {
        NodeSpan::Empty
    }

    fn build(&self, _vc: &mut BuildContext) -> Self::State {
        ()
    }

    fn update(&self, _vc: &mut BuildContext, _state: &mut Self::State) {}

    fn raze(&self, _vc: &mut World, _state: &mut Self::State) {}
}

/// View which renders a String
impl View for String {
    type State = Entity;

    fn nodes(&self, _vc: &BuildContext, state: &Self::State) -> NodeSpan {
        NodeSpan::Node(*state)
    }

    fn build(&self, vc: &mut BuildContext) -> Self::State {
        let id = vc
            .world
            .spawn((TextBundle {
                text: Text::from_section(self.clone(), TextStyle { ..default() }),
                // TextStyle {
                //     font_size: 40.0,
                //     color: Color::rgb(0.9, 0.9, 0.9),
                //     ..Default::default()
                // },
                // background_color: Color::rgb(0.65, 0.75, 0.65).into(),
                // border_color: Color::BLUE.into(),
                // focus_policy: FocusPolicy::Pass,
                ..default()
            },))
            .id();
        id
    }

    fn update(&self, vc: &mut BuildContext, state: &mut Self::State) {
        // If it's a single node and has a text component
        let nodes = self.nodes(vc, state);
        if let NodeSpan::Node(text_node) = nodes {
            if let Some(mut old_text) = vc.entity_mut(text_node).get_mut::<Text>() {
                // TODO: compare text for equality.
                old_text.sections.clear();
                old_text.sections.push(TextSection {
                    value: self.to_owned(),
                    style: TextStyle { ..default() },
                });
                return;
            }
        }

        // Despawn node and create new text node
        nodes.despawn(vc.world);
        vc.mark_changed_shape();
        *state = self.build(vc)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        let mut entt = world.entity_mut(*state);
        entt.remove_parent();
        entt.despawn();
    }
}

/// View which renders a string slice.
impl View for &str {
    type State = Entity;

    fn nodes(&self, _vc: &BuildContext, state: &Self::State) -> NodeSpan {
        NodeSpan::Node(*state)
    }

    fn build(&self, vc: &mut BuildContext) -> Self::State {
        let id = vc
            .world
            .spawn((TextBundle {
                text: Text::from_section(self.to_string(), TextStyle { ..default() }),
                // TextStyle {
                //     font_size: 40.0,
                //     color: Color::rgb(0.9, 0.9, 0.9),
                //     ..Default::default()
                // },
                // background_color: Color::rgb(0.65, 0.75, 0.65).into(),
                // border_color: Color::BLUE.into(),
                // focus_policy: FocusPolicy::Pass,
                ..default()
            },))
            .id();
        id
    }

    fn update(&self, vc: &mut BuildContext, state: &mut Self::State) {
        // If it's a single node and has a text component
        let nodes = self.nodes(vc, state);
        if let NodeSpan::Node(text_node) = nodes {
            if let Some(mut old_text) = vc.entity_mut(text_node).get_mut::<Text>() {
                // TODO: compare text for equality.
                old_text.sections.clear();
                old_text.sections.push(TextSection {
                    value: self.to_string(),
                    style: TextStyle { ..default() },
                });
                return;
            }
        }

        // Despawn node and create new text node
        nodes.despawn(vc.world);
        vc.mark_changed_shape();
        *state = self.build(vc)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        let mut entt = world.entity_mut(*state);
        entt.remove_parent();
        entt.despawn();
    }
}

/// View which renders a bare presenter with no arguments
impl<V: View + 'static, F: PresenterFn<fn(Cx<()>) -> V, Props = ()>> View for F
where
    F: Fn(Cx<()>) -> V + Send,
{
    // State holds the PresenterState entity.
    type State = Entity;

    fn nodes(&self, vc: &BuildContext, state: &Self::State) -> NodeSpan {
        // get the handle from the PresenterState for this invocation.
        let entt = vc.entity(*state);
        let Some(ref handle) = entt.get::<ViewHandle>() else {
            return NodeSpan::Empty;
        };
        handle.nodes()
    }

    fn build(&self, parent_ecx: &mut BuildContext) -> Self::State {
        let entity = parent_ecx
            .world
            .spawn((ViewHandle::new(*self, ()), Name::new("presenter")))
            .insert(PresenterStateChanged)
            .set_parent(parent_ecx.entity)
            .id();
        // Not calling build here: will be done asynchronously.
        entity
    }

    fn update(&self, _parent_ecx: &mut BuildContext, _state: &mut Self::State) {
        // Rebuild does nothing: it's up to the child to decide whether or not it wants to
        // rebuild. Since there are no props, we don't mark the child as modified.
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        let mut entt = world.entity_mut(*state);
        let Some(handle) = entt.get_mut::<ViewHandle>() else {
            return;
        };
        let inner = handle.inner.clone();
        // Raze the contents of the child ViewState.
        inner.lock().unwrap().raze(world, *state);
        // Despawn the ViewHandle.
        let mut entt = world.entity_mut(*state);
        entt.remove_parent();
        entt.despawn();
    }
}

/// A trait that allows methods to be added to presenter function references.
pub trait PresenterFn<Marker: 'static>: Sized + Send + Copy + 'static {
    /// The type of properties expected by this presenter.
    type Props: Send + PartialEq;

    /// The type of view produced by this presenter.
    type View: View;
    // type Param: PresenterParam;

    /// Used to invoke a presenter from within a presenter. This binds a set of properties
    /// to the child presenter, and constructs a new [`ViewHandle`] containing a [`PresenterState`].
    /// The result is a [`View`] which references this handle.
    fn bind(self, props: Self::Props) -> Bind;

    /// Method which calls the presenter, creating the [`View`].
    fn call(
        &mut self,
        cx: Cx<Self::Props>,
        // Dependency injection? How?
    ) -> Self::View;
}

impl<V: View, P: Send + PartialEq + 'static, F: FnMut(Cx<P>) -> V + Copy + Send + 'static>
    PresenterFn<fn(Cx<P>) -> V> for F
where
    V: 'static,
{
    type Props = P;
    type View = V;

    fn bind(self, props: Self::Props) -> Bind {
        Bind::new(self, props)
    }

    fn call(&mut self, cx: Cx<Self::Props>) -> Self::View {
        self(cx)
    }
}
