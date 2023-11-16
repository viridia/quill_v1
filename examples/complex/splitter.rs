use std::sync::Arc;

use bevy::{prelude::*, ui};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use lazy_static::lazy_static;
use quill::{Cx, Element, ElementClasses, PointerEvents, StyleSet, View};

// Style definitions for the splitter widget.
lazy_static! {
    // The splitter widget
    static ref STYLE_VSPLITTER: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#181818").unwrap()))
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Center)
        .display(ui::Display::Flex)
        .pointer_events(PointerEvents::SelfOnly)
        .width(9)
        .selector(".drag", |ss| ss
            .background_color(Some(Color::hex("#080808").unwrap())))));
    // The decorative handle inside the splitter.
    static ref STYLE_VSPLITTER_INNER: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#282828").unwrap()))
        .display(ui::Display::Flex)
        .width(5)
        .height(ui::Val::Percent(30.))
        .selector(".hover > &", |ss| ss
            .background_color(Some(Color::hex("#383838").unwrap())))
        .selector(".drag > &", |ss| ss
            .background_color(Some(Color::hex("#484848").unwrap())))));
}

const CLS_HOVER: &str = "hover";
const CLS_DRAG: &str = "drag";

#[derive(Clone)]
pub struct SplitterProps {
    pub id: &'static str,
}

#[derive(Clone, Event, EntityEvent)]
pub struct SplitterDragged {
    #[target] // Marks the field of the event that specifies the target entity
    pub target: Entity,
    pub id: &'static str,
    pub delta: f32,
}

// Vertical splitter bar which can be dragged
pub fn v_splitter(cx: Cx<SplitterProps>) -> impl View {
    // Needs to be a local variable so that it can be captured in the event handler.
    let id = cx.props.id;
    Element::new(Element::new(()).styled(STYLE_VSPLITTER_INNER.clone()))
        .once(move |entity, world| {
            let mut e = world.entity_mut(entity);
            e.insert((
                On::<Pointer<Over>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    // println!("Over");
                    classes.add_class(CLS_HOVER)
                }),
                On::<Pointer<Out>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    // println!("Out");
                    classes.remove_class(CLS_HOVER)
                }),
                On::<Pointer<DragStart>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    // Add 'drag' class while dragging.
                    classes.add_class(CLS_DRAG)
                }),
                On::<Pointer<DragEnd>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    classes.remove_class(CLS_DRAG)
                }),
                On::<Pointer<Drag>>::run(
                    move |events: Res<ListenerInput<Pointer<Drag>>>,
                          mut ev: EventWriter<SplitterDragged>| {
                        ev.send(SplitterDragged {
                            target: events.target,
                            id,
                            delta: events.delta.x,
                        });
                    },
                ),
                // On::<Pointer<Drag>>::run(
                //     |ev: Listener<Pointer<Drag>>, mut res: ResMut<PanelWidth>| {
                //         res.0 += ev.delta.x as i32;
                //     },
                // ),
                On::<Pointer<PointerCancel>>::listener_component_mut::<ElementClasses>(
                    |_, classes| {
                        println!("Cancel");
                        classes.remove_class(CLS_HOVER);
                        classes.remove_class(CLS_DRAG)
                    },
                ),
            ));
        })
        .styled(STYLE_VSPLITTER.clone())
}
