use std::sync::Arc;

use bevy::{prelude::*, ui};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use lazy_static::lazy_static;
use quill::{Cx, Element, ElementClasses, PointerEvents, StyleSet, View};

pub struct SplitterPlugin;

impl Plugin for SplitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EventListenerPlugin::<SplitterDragStart>::default())
            .add_plugins(EventListenerPlugin::<SplitterDragged>::default())
            .add_event::<SplitterDragStart>()
            .add_event::<SplitterDragged>();
    }
}

// Style definitions for the splitter widget.
lazy_static! {
    // The splitter widget
    static ref STYLE_VSPLITTER: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#181818").unwrap()))
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Center)
        .display(ui::Display::Flex)
        .width(9)
        .selector(".drag", |ss| ss
            .background_color(Some(Color::hex("#080808").unwrap())))));
    // The decorative handle inside the splitter.
    static ref STYLE_VSPLITTER_INNER: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#282828").unwrap()))
        .display(ui::Display::Flex)
        .width(5)
        .pointer_events(PointerEvents::None)
        .height(ui::Val::Percent(30.))
        .selector(":hover > &", |ss| ss
            .background_color(Some(Color::hex("#383838").unwrap())))
        .selector(".drag > &", |ss| ss
            .background_color(Some(Color::hex("#484848").unwrap())))));
}

const CLS_DRAG: &str = "drag";

#[derive(Clone)]
pub struct SplitterProps {
    pub id: &'static str,
}

#[derive(Clone, Event, EntityEvent)]
pub struct SplitterDragStart {
    #[target] // Marks the field of the event that specifies the target entity
    pub target: Entity,
    pub id: &'static str,
}

#[derive(Clone, Event, EntityEvent)]
pub struct SplitterDragged {
    #[target] // Marks the field of the event that specifies the target entity
    pub target: Entity,
    pub id: &'static str,
    pub distance: f32,
    // pub origin: f32,
}

// Vertical splitter bar which can be dragged
pub fn v_splitter(cx: Cx<SplitterProps>) -> impl View {
    // Needs to be a local variable so that it can be captured in the event handler.
    let id = cx.props.id;
    Element::new(Element::new(()).styled(STYLE_VSPLITTER_INNER.clone()))
        .once(move |entity, world| {
            let mut e = world.entity_mut(entity);
            e.insert((
                On::<Pointer<DragStart>>::run(
                    // TODO: Instead of sending a start event, what we need is to simply remember
                    // the current split value at drag start and add it to the output.
                    move |ev: Res<ListenerInput<Pointer<DragStart>>>,
                          mut writer: EventWriter<SplitterDragStart>,
                          mut query: Query<&mut ElementClasses>| {
                        if let Ok(mut classes) = query.get_mut(ev.target) {
                            classes.add_class(CLS_DRAG)
                        }
                        writer.send(SplitterDragStart {
                            target: ev.target,
                            id,
                        });
                    },
                ),
                On::<Pointer<DragEnd>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    classes.remove_class(CLS_DRAG)
                }),
                On::<Pointer<Drag>>::run(
                    move |ev: Res<ListenerInput<Pointer<Drag>>>,
                          mut writer: EventWriter<SplitterDragged>| {
                        writer.send(SplitterDragged {
                            target: ev.target,
                            id,
                            distance: ev.distance.x,
                        });
                    },
                ),
                On::<Pointer<PointerCancel>>::listener_component_mut::<ElementClasses>(
                    |_, classes| {
                        println!("Cancel");
                        classes.remove_class(CLS_DRAG)
                    },
                ),
            ));
        })
        .styled(STYLE_VSPLITTER.clone())
}
