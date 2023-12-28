use crate::Changed;
use bevy::prelude::*;
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::prelude::*;
use static_init::dynamic;

const THUMB_SIZE: f32 = 18.;

// Style definitions for slider widget.

// The slider
#[dynamic]
static STYLE_SLIDER: StyleHandle =
    StyleHandle::build(|ss| ss.min_width(THUMB_SIZE).height(THUMB_SIZE));

const CLS_DRAG: &str = "drag";

pub struct SliderChildProps {
    pub percent: f32,
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

#[derive(Clone, PartialEq)]
pub struct SliderProps<V: View, F: Fn(SliderChildProps) -> V + Copy + Send + 'static> {
    pub id: &'static str,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub children: F,
}

#[derive(Clone, PartialEq, Default, Copy)]
struct DragState {
    dragging: bool,
    offset: f32,
}

// Horizontal slider widget
pub fn h_slider<V: View, F: Fn(SliderChildProps) -> V + Copy + Send + 'static>(
    mut cx: Cx<SliderProps<V, F>>,
) -> impl View {
    let drag_state = cx.create_atom_init::<DragState>(|| DragState::default());
    // Pain point: Need to capture all props for closures.
    let id = cx.props.id;
    let min = cx.props.min;
    let max = cx.props.max;
    let value = cx.props.value;
    let range = cx.props.max - cx.props.min;
    let pos = if range > 0. {
        (cx.props.value - cx.props.min) / range
    } else {
        0.
    }
    .clamp(0., 1.);

    Element::new()
        .class_names(CLS_DRAG.if_true(cx.read_atom(drag_state).dragging))
        .styled(STYLE_SLIDER.clone())
        .insert((
            On::<Pointer<DragStart>>::run(move |mut atoms: AtomStore| {
                // Save initial value to use as drag offset.
                atoms.set(
                    drag_state,
                    DragState {
                        dragging: true,
                        offset: value,
                    },
                );
            }),
            On::<Pointer<DragEnd>>::run(move |mut atoms: AtomStore| {
                atoms.set(
                    drag_state,
                    DragState {
                        dragging: false,
                        offset: value,
                    },
                );
            }),
            On::<Pointer<Drag>>::run(
                move |ev: Listener<Pointer<Drag>>,
                      query: Query<(&Node, &GlobalTransform)>,
                      atoms: AtomStore,
                      mut writer: EventWriter<Changed<f32>>| {
                    let ds = atoms.get(drag_state);
                    if ds.dragging {
                        match query.get(ev.listener()) {
                            Ok((node, transform)) => {
                                // Measure node width and slider value.
                                let slider_width =
                                    node.logical_rect(transform).width() - THUMB_SIZE;
                                let new_value = if range > 0. {
                                    ds.offset + (ev.distance.x * range) / slider_width
                                } else {
                                    min + range * 0.5
                                };
                                writer.send(Changed::<f32> {
                                    target: ev.target,
                                    id,
                                    value: new_value.clamp(min, max),
                                    finish: false,
                                });
                            }
                            _ => return,
                        }
                    }
                },
            ),
            On::<Pointer<PointerCancel>>::run(move |mut atoms: AtomStore| {
                println!("Slider Cancel");
                atoms.set(
                    drag_state,
                    DragState {
                        dragging: false,
                        offset: value,
                    },
                );
            }),
        ))
        .children((cx.props.children)(SliderChildProps {
            percent: pos,
            min,
            max,
            value,
        }))
}
