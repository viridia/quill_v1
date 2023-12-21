use bevy::{prelude::*, ui};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::prelude::*;
use static_init::dynamic;

pub struct SplitterPlugin;

impl Plugin for SplitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EventListenerPlugin::<SplitterDragged>::default())
            .add_event::<SplitterDragged>();
    }
}

// Style definitions for the splitter widget.

// The splitter widget
#[dynamic]
static STYLE_VSPLITTER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#181818")
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Center)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .gap(8)
        .width(9)
        .selector(".drag", |ss| ss.background_color("#080808"))
});

// The decorative handle inside the splitter.
#[dynamic]
static STYLE_VSPLITTER_INNER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#282828")
        .display(ui::Display::Flex)
        .width(3)
        .pointer_events(PointerEvents::None)
        .height(ui::Val::Percent(5.))
        .selector(":hover > &", |ss| ss.background_color("#383838"))
        .selector(".drag > &", |ss| ss.background_color("#484848"))
});

const CLS_DRAG: &str = "drag";

#[derive(Clone, PartialEq)]
pub struct SplitterProps {
    pub value: f32,
    pub id: &'static str,
}

#[derive(Clone, Event, EntityEvent)]
pub struct SplitterDragged {
    #[target]
    pub target: Entity,
    pub id: &'static str,
    pub value: f32,
}

// Vertical splitter bar which can be dragged
pub fn v_splitter(mut cx: Cx<SplitterProps>) -> impl View {
    let drag_offset = cx.create_atom_init::<f32>(|| 0.);

    // This is needed because bevy_eventlistener sometimes sends events out of order,
    // so that we get Drag before DragStart.
    let is_dragging = cx.create_atom_init::<bool>(|| false);

    // Needs to be a local variable so that it can be captured in the event handler.
    let id = cx.props.id;
    let current_offset = cx.props.value;
    Element::new()
        .named("splitter")
        .with(move |mut e| {
            e.insert((
                On::<Pointer<DragStart>>::run(
                    move |ev: Listener<Pointer<DragStart>>,
                          mut atoms: AtomStore,
                          mut query: Query<&mut ElementClasses>| {
                        // println!("Start drag offset: {}", current_offset);
                        // Save initial value to use as drag offset.
                        atoms.set(drag_offset, current_offset);
                        atoms.set(is_dragging, true);
                        if let Ok(mut classes) = query.get_mut(ev.target) {
                            classes.add_class(CLS_DRAG)
                        }
                    },
                ),
                On::<Pointer<DragEnd>>::run(
                    move |ev: Listener<Pointer<DragEnd>>,
                          mut atoms: AtomStore,
                          mut query: Query<&mut ElementClasses>| {
                        if let Ok(mut classes) = query.get_mut(ev.target) {
                            classes.remove_class(CLS_DRAG)
                        }
                        atoms.set(is_dragging, false);
                    },
                ),
                On::<Pointer<Drag>>::run(
                    move |ev: Listener<Pointer<Drag>>,
                          mut writer: EventWriter<SplitterDragged>,
                          atoms: AtomStore| {
                        if atoms.get(is_dragging) {
                            writer.send(SplitterDragged {
                                target: ev.target,
                                id,
                                value: ev.distance.x + atoms.get(drag_offset),
                            });
                        }
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
        .children((
            Element::new().styled(STYLE_VSPLITTER_INNER.clone()),
            Element::new().styled(STYLE_VSPLITTER_INNER.clone()),
        ))
}
