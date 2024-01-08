use bevy::prelude::*;
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::prelude::*;

use crate::SplitterEvent;

pub struct SplitterPlugin;

const CLS_DRAG: &str = "drag";

#[derive(Clone, PartialEq, Default)]
pub struct SplitterProps<V: View + Clone, S: StyleTuple = ()> {
    pub value: f32,
    pub id: &'static str,
    pub children: V,
    pub style: S,
}

#[derive(Clone, PartialEq, Default, Copy)]
struct DragState {
    dragging: bool,
    offset: f32,
}

// Vertical splitter bar which can be dragged
pub fn v_splitter<V: View + Clone, S: StyleTuple>(mut cx: Cx<SplitterProps<V, S>>) -> impl View {
    let drag_state = cx.create_atom_init::<DragState>(DragState::default);
    let id = cx.props.id;
    let current_offset = cx.props.value;
    Element::new()
        .named("v_splitter")
        .class_names(CLS_DRAG.if_true(cx.read_atom(drag_state).dragging))
        .styled(cx.props.style.clone())
        .insert((
            On::<Pointer<DragStart>>::run(move |mut atoms: AtomStore| {
                // Save initial value to use as drag offset.
                atoms.set(
                    drag_state,
                    DragState {
                        dragging: true,
                        offset: current_offset,
                    },
                );
            }),
            On::<Pointer<DragEnd>>::run(move |mut atoms: AtomStore| {
                atoms.set(
                    drag_state,
                    DragState {
                        dragging: false,
                        offset: current_offset,
                    },
                );
            }),
            On::<Pointer<Drag>>::run(
                move |ev: Listener<Pointer<Drag>>,
                      mut writer: EventWriter<SplitterEvent>,
                      atoms: AtomStore| {
                    let ds = atoms.get(drag_state);
                    if ds.dragging {
                        writer.send(SplitterEvent {
                            target: ev.target,
                            id,
                            value: ev.distance.x + ds.offset,
                        });
                    }
                },
            ),
            On::<Pointer<PointerCancel>>::run(move |mut atoms: AtomStore| {
                println!("Splitter Cancel");
                atoms.set(
                    drag_state,
                    DragState {
                        dragging: true,
                        offset: current_offset,
                    },
                );
            }),
        ))
        .children(cx.props.children.clone())
}
