use std::sync::Arc;

use bevy::{prelude::*, ui};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use lazy_static::lazy_static;
use quill::{Cx, Element, ElementClasses, PointerEvents, StyleSet, View};

pub struct SplitterPlugin;

impl Plugin for SplitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EventListenerPlugin::<SplitterDragged>::default())
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

#[derive(Clone, PartialEq)]
pub struct SplitterProps {
    pub value: f32,
    pub id: &'static str,
}

#[derive(Clone, Event, EntityEvent)]
pub struct SplitterDragged {
    #[target] // Marks the field of the event that specifies the target entity
    pub target: Entity,
    pub id: &'static str,
    pub value: f32,
}

// Vertical splitter bar which can be dragged
pub fn v_splitter(mut cx: Cx<SplitterProps>) -> impl View {
    let drag_offset = cx.use_local::<f32>(|| 0.);

    // Needs to be a local variable so that it can be captured in the event handler.
    let id = cx.props.id;
    let current_offset = cx.props.value;
    Element::new(Element::new(()).styled(STYLE_VSPLITTER_INNER.clone()))
        .with(move |entity, world| {
            let mut e = world.entity_mut(entity);
            let mut drag_offset_1 = drag_offset.clone();
            let drag_offset_2 = drag_offset_1.clone();
            e.insert((
                On::<Pointer<DragStart>>::run(
                    move |ev: Res<ListenerInput<Pointer<DragStart>>>,
                          mut query: Query<&mut ElementClasses>| {
                        // println!("Start drag offset: {}", current_offset);
                        // Save initial value to use as drag offset.
                        drag_offset_1.set(current_offset);
                        if let Ok(mut classes) = query.get_mut(ev.target) {
                            classes.add_class(CLS_DRAG)
                        }
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
                            value: ev.distance.x + drag_offset_2.get(),
                        });
                    },
                ),
                On::<Pointer<PointerCancel>>::listener_component_mut::<ElementClasses>(
                    |_, classes| {
                        println!("Splitter Cancel");
                        classes.remove_class(CLS_DRAG)
                    },
                ),
            ));
        })
        .styled(STYLE_VSPLITTER.clone())
}
