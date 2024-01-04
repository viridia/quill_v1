use bevy::ecs::event::Event;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct EgretEventsPlugin;

impl Plugin for EgretEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EventListenerPlugin::<Clicked>::default(),
            EventListenerPlugin::<ValueChanged<f32>>::default(),
            EventListenerPlugin::<MenuEvent>::default(),
            EventListenerPlugin::<SplitterEvent>::default(),
        ))
        .add_event::<Clicked>()
        .add_event::<ValueChanged<f32>>()
        .add_event::<MenuEvent>()
        .add_event::<SplitterEvent>();
    }
}

/// Event that is triggered when a button is clicked
#[derive(Clone, Event, EntityEvent)]
pub struct Clicked {
    #[target]
    pub target: Entity,
    pub id: &'static str,
}

/// Event emitted by a widget that contains a value; indicates that the value has changed.
#[derive(Clone, Event, EntityEvent)]
pub struct ValueChanged<T: Clone + Send + Sync + 'static> {
    #[target]
    pub target: Entity,

    /// The id of the widget emitting this change.
    pub id: &'static str,

    /// The updated value.
    pub value: T,

    /// Indicates that this is the last change of a series, for example when dragging a slider,
    /// this indicates that the dragging is complete.
    pub finish: bool,
}

/// Menu keyboard actions
#[derive(Clone, Debug)]
pub enum MenuAction {
    /// Toggle menu open
    Open,
    /// Toggle menu closed
    Close,
    /// Move selection up
    Up,
    /// Move selection down
    Down,
    /// Move selection to beginning
    Home,
    /// Move selection to end
    End,
    // / Trigger an action
    // Accept,
}

/// Sent by MenuButton to toggle menu open/closed.
#[derive(Clone, Event, EntityEvent)]
pub struct MenuEvent {
    #[target]
    pub target: Entity,
    pub action: MenuAction,
}

#[derive(Clone, Event, EntityEvent)]
pub struct SplitterEvent {
    #[target]
    pub target: Entity,
    pub id: &'static str,
    pub value: f32,
}
