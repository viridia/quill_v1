use std::any::Any;

use bevy::prelude::*;

use crate::{BuildContext, PresenterFn, View, ViewHandle};

use crate::node_span::NodeSpan;

use super::presenter_state::PresenterStateChanged;

struct BindState<Marker: 'static, F: PresenterFn<Marker>> {
    presenter: F,
    props: F::Props,
}

impl<Marker: 'static, F: PresenterFn<Marker>> BindState<Marker, F> {
    fn new(presenter: F, props: F::Props) -> Self {
        Self {
            presenter,
            props: props,
        }
    }
}

trait AnyBindState: Send {
    fn create_handle(&self) -> ViewHandle;
    fn update_handle_props(&self, handle: &mut ViewHandle) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn eq(&self, other: &dyn AnyBindState) -> bool;
    fn clone(&self) -> Box<dyn AnyBindState>;
}

impl<Marker: 'static, F: PresenterFn<Marker>> AnyBindState for BindState<Marker, F> {
    fn create_handle(&self) -> ViewHandle {
        // if self.props.is_none() {
        //     panic!("BindState::create_handle called twice");
        // }
        ViewHandle::new(self.presenter, self.props.clone())
    }

    fn update_handle_props(&self, handle: &mut ViewHandle) -> bool {
        handle.update_props(&self.props)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq(&self, other: &dyn AnyBindState) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            Some(other) => {
                &self.presenter as *const _ == &other.presenter as *const _
                    && self.props == other.props
            }
            None => false,
        }
    }

    fn clone(&self) -> Box<dyn AnyBindState> {
        Box::new(Self {
            presenter: self.presenter.clone(),
            props: self.props.clone(),
        })
    }
}

/// Binds a presenter to properties and implements a view.
/// Implementation note: It is important that this type be completely type-erased, otherwise
/// recursive presenter invocations (like tree views) will not compile.
#[doc(hidden)]
pub struct Bind {
    binding: Box<dyn AnyBindState>,
}

impl Bind {
    pub fn new<Marker, F: PresenterFn<Marker>>(presenter: F, props: F::Props) -> Self {
        Self {
            binding: Box::new(BindState::new(presenter, props)),
        }
    }
}

impl View for Bind {
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

    // Spawn a new presenter entity.
    fn build(&self, parent_ecx: &mut BuildContext) -> Self::State {
        let entity = parent_ecx
            .world
            .spawn((self.binding.create_handle(), Name::new("presenter")))
            .insert(PresenterStateChanged)
            .set_parent(parent_ecx.entity)
            .id();
        // Not calling inner build here: will be done asynchronously.
        entity
    }

    fn update(&self, vc: &mut BuildContext, state: &mut Self::State) {
        // get the handle from the current view state
        let mut entt = vc.entity_mut(*state);
        let Some(mut handle) = entt.get_mut::<ViewHandle>() else {
            return;
        };
        // Update child view properties. This transfers the props from the 'new' presenter
        // that is a member of the Bind, to the 'old' presenter state which is stored in the
        // view handle. The old state is the one that will persist.
        if self.binding.update_handle_props(&mut handle) {
            entt.insert(PresenterStateChanged);
        }
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        let mut entt = world.entity_mut(*state);
        let Some(handle) = entt.get_mut::<ViewHandle>() else {
            panic!("Bind::raze called without ViewHandle");
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

impl Clone for Bind {
    fn clone(&self) -> Self {
        Self {
            binding: self.binding.clone(),
        }
    }
}

impl PartialEq for Bind {
    fn eq(&self, other: &Self) -> bool {
        if &self as *const _ == &other as *const _ {
            return true;
        }
        self.binding.eq(&*other.binding)
    }
}
