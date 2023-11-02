use std::{marker::PhantomData, mem::swap};

use bevy::{
    prelude::*,
    text::{Text, TextStyle},
};

use crate::ViewStateComp;

use super::node_span::NodeSpan;

pub struct ElementContext<'w> {
    pub(crate) world: &'w mut World,
}

pub trait AnyResource: Send + Sync {
    fn is_changed(&self, world: &World) -> bool;
}

#[derive(PartialEq, Eq)]
pub struct AnyRes<T> {
    pub pdata: PhantomData<T>,
}

impl<T> AnyRes<T> {
    fn new() -> Self {
        Self { pdata: PhantomData }
    }
}

impl<T> AnyResource for AnyRes<T>
where
    T: Resource,
{
    fn is_changed(&self, world: &World) -> bool {
        world.is_resource_changed::<T>()
    }
}

/// Tracks resources used by each ViewState
/// the key is the ViewState id
#[derive(Component, Default)]
pub struct TrackedResources {
    pub data: Vec<Box<dyn AnyResource>>,
}

// TODO: Move this to it's own file once it's stable.
pub struct Cx<'w, 'p, Props = ()> {
    pub props: &'p Props,
    pub sys: &'p mut ElementContext<'w>,
    pub entity: Entity,
}

impl<'w, 'p, Props> Cx<'w, 'p, Props> {
    pub fn use_resource<T: Resource>(&mut self) -> &T {
        let mut tracked = self
            .sys
            .world
            .get_mut::<TrackedResources>(self.entity)
            .expect("TrackedResources not found for this entity");
        tracked.data.push(Box::new(AnyRes::<T>::new()));
        self.sys.world.resource::<T>()
    }

    pub fn use_resource_mut<T: Resource>(&mut self) -> Mut<T> {
        let mut tracked = self
            .sys
            .world
            .get_mut::<TrackedResources>(self.entity)
            .expect("TrackedResources not found for this entity");
        tracked.data.push(Box::new(AnyRes::<T>::new()));
        self.sys.world.resource_mut::<T>()
    }
}

// pub struct ClassList {
//     classes: HashSet<String>,
// }

pub trait View: Send + Sync {
    type State: Send + Sync + Default;

    fn build(&self, ecx: &mut ElementContext, state: &mut Self::State, prev: &NodeSpan)
        -> NodeSpan;
}

/// View which renders nothing
impl View for () {
    type State = ();

    fn build(
        &self,
        _ecx: &mut ElementContext,
        _state: &mut Self::State,
        _prev: &NodeSpan,
    ) -> NodeSpan {
        NodeSpan::Empty
    }
}

/// View which renders a String
impl View for String {
    type State = ();

    fn build(
        &self,
        ecx: &mut ElementContext,
        _state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        if let NodeSpan::Node(text_entity) = prev {
            if let Some(mut old_text) = ecx.world.entity_mut(*text_entity).get_mut::<Text>() {
                // TODO: compare text for equality.
                old_text.sections.clear();
                old_text.sections.push(TextSection {
                    value: self.to_owned(),
                    style: TextStyle { ..default() },
                });
                return NodeSpan::Node(*text_entity);
            }
        }

        prev.despawn_recursive(ecx.world);
        let new_entity = ecx
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

        return NodeSpan::Node(new_entity);
    }
}

/// View which renders a string slice.
impl View for &'static str {
    type State = ();

    fn build(
        &self,
        ecx: &mut ElementContext,
        _state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        if let NodeSpan::Node(text_entity) = prev {
            if let Some(mut old_text) = ecx.world.entity_mut(*text_entity).get_mut::<Text>() {
                // TODO: compare text for equality.
                old_text.sections.clear();
                old_text.sections.push(TextSection {
                    value: self.to_string(),
                    style: TextStyle { ..default() },
                });
                return NodeSpan::Node(*text_entity);
            }
        }

        prev.despawn_recursive(ecx.world);
        let new_entity = ecx
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

        return NodeSpan::Node(new_entity);
    }
}

/// View which renders a bare presenter with no arguments
impl<A: View + 'static> View for fn(cx: Cx) -> A {
    type State = Option<Entity>;

    fn build(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        let mut child_state: A::State = Default::default();
        let entity: Entity = match state {
            Some(entity) => *entity,
            None => {
                let entity = ecx.world.spawn(TrackedResources::default()).id();
                *state = Some(entity);
                entity
            }
        };
        let cx = Cx {
            sys: ecx,
            props: &(),
            entity,
        };
        self(cx).build(ecx, &mut child_state, prev)
    }
}

/// Binds a presenter to properties and implements a view
pub struct Bind<V: View, Props: Send + Sync + Clone> {
    presenter: fn(cx: Cx<Props>) -> V,
    props: Props,
}

impl<V: View, Props: Send + Sync + Clone> Bind<V, Props> {
    pub fn new(presenter: fn(cx: Cx<Props>) -> V, props: Props) -> Self {
        Self { presenter, props }
    }
}

impl<V: View + 'static, Props: Send + Sync + 'static + Clone> View for Bind<V, Props> {
    type State = Option<Entity>;

    fn build(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        let entity = match state {
            Some(entity) => *entity,
            None => {
                let entity = ecx
                    .world
                    .spawn((
                        TrackedResources::default(),
                        ViewStateComp::new(self.presenter, self.props.clone()),
                    ))
                    .id();
                *state = Some(entity);
                entity
            }
        };

        // get the handle from the current view state
        let mut entt = ecx.world.entity_mut(entity);
        let Some(mut view_state) = entt.get_mut::<ViewStateComp>() else {
            return NodeSpan::Empty;
        };
        let mut handle = view_state
            .handle
            .take()
            .expect("ViewState::handle should be present at this point");

        // build the view
        handle.build(ecx, entity);
        let nodes = handle.nodes(prev);

        // put back the handle
        let mut entt = ecx.world.entity_mut(entity);
        let Some(mut view_state) = entt.get_mut::<ViewStateComp>() else {
            return NodeSpan::Empty;
        };
        view_state.handle = Some(handle);

        nodes
    }
}

pub struct Sequence<A: ViewTuple> {
    items: A,
}

impl<A: ViewTuple> Sequence<A> {
    pub fn new(items: A) -> Self {
        Self { items }
    }
}

impl<A: ViewTuple> View for Sequence<A> {
    type State = A::State;

    fn build(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        let count_spans = self.items.len();
        let mut child_spans: Vec<NodeSpan> = vec![NodeSpan::Empty; count_spans];

        // Get a copy of child spans from Component
        if let NodeSpan::Node(entity) = prev {
            if let Some(cmp) = ecx.world.entity_mut(*entity).get_mut::<SequenceComponent>() {
                if cmp.children.len() == self.items.len() {
                    child_spans = cmp.children.clone();
                }
            }
        }

        // Rebuild span array, replacing ones that changed.
        self.items.build_spans(ecx, state, &mut child_spans);
        let mut count_children: usize = 0;
        for node in child_spans.iter() {
            count_children += node.count()
        }
        let mut flat: Vec<Entity> = Vec::with_capacity(count_children);
        for node in child_spans.iter() {
            node.flatten(&mut flat);
        }

        if let NodeSpan::Node(entity) = prev {
            let mut em = ecx.world.entity_mut(*entity);
            if let Some(mut cmp) = em.get_mut::<SequenceComponent>() {
                if cmp.children != child_spans {
                    swap(&mut cmp.children, &mut child_spans);
                    // TODO: Need to replace child entities
                    // em.push_children(&flat);
                }
                return NodeSpan::Node(*entity);
            }
        }

        // Remove previous entity
        prev.despawn_recursive(ecx.world);

        let new_entity = ecx
            .world
            .spawn((
                SequenceComponent {
                    children: child_spans,
                },
                NodeBundle {
                    // focus_policy: FocusPolicy::Pass,
                    visibility: Visibility::Visible,
                    ..default()
                },
            ))
            .push_children(&flat)
            .id();

        NodeSpan::Node(new_entity)
    }
}

/// Component for a sequence, tracks the list of children by span.
#[derive(Component)]
pub struct SequenceComponent {
    pub(crate) children: Vec<NodeSpan>,
}

// If

pub struct If<Pos: View, Neg: View> {
    test: bool,
    pos: Pos,
    neg: Neg,
}

impl<Pos: View, Neg: View> If<Pos, Neg> {
    pub fn new(test: bool, pos: Pos, neg: Neg) -> Self {
        Self { test, pos, neg }
    }
}

impl<Pos: View, Neg: View> View for If<Pos, Neg> {
    // TODO: Make this a union instead.
    type State = (Pos::State, Neg::State);

    fn build(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        if self.test {
            self.pos.build(ecx, &mut state.0, prev)
        } else {
            self.neg.build(ecx, &mut state.1, prev)
        }
    }
}

// ViewTuple

// TODO: Move this to it's own file once it's stable.
// TODO: Turn this into a macro once it's stable.
pub trait ViewTuple: Send + Sync {
    type State: Send + Sync + Default;

    fn len(&self) -> usize;

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]);
}

impl<A: View> ViewTuple for A {
    type State = A::State;

    fn len(&self) -> usize {
        1
    }

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]) {
        out[0] = self.build(cx, state, &out[0])
    }
}

impl<A: View> ViewTuple for (A,) {
    type State = (A::State,);

    fn len(&self) -> usize {
        1
    }

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]) {
        out[0] = self.0.build(cx, &mut state.0, &out[0])
    }
}

impl<A0: View, A1: View> ViewTuple for (A0, A1) {
    type State = (A0::State, A1::State);

    fn len(&self) -> usize {
        2
    }

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]) {
        out[0] = self.0.build(cx, &mut state.0, &out[0]);
        out[1] = self.1.build(cx, &mut state.1, &out[1]);
    }
}

impl<A0: View, A1: View, A2: View> ViewTuple for (A0, A1, A2) {
    type State = (A0::State, A1::State, A2::State);

    fn len(&self) -> usize {
        3
    }

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]) {
        out[0] = self.0.build(cx, &mut state.0, &out[0]);
        out[1] = self.1.build(cx, &mut state.1, &out[1]);
        out[2] = self.2.build(cx, &mut state.2, &out[2]);
    }
}
