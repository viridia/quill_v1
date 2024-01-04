use std::sync::Arc;

use crate::ValueChanged;
use bevy::prelude::*;
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::prelude::*;

/// Props which are passed to the children via the closure.
pub struct SliderChildProps {
    /// Slider position as a percentage of the slider width, excluding thumb size.
    pub percent: f32,
    /// Minimum slider value.
    pub min: f32,
    /// Maximum slider value.
    pub max: f32,
    /// Current slider value.
    pub value: f32,
    /// True if the slider is being dragged.
    pub is_dragging: bool,
}

/// Properties for slider widget.
pub struct SliderProps<V: View, F: Fn(SliderChildProps) -> V, S: StyleTuple> {
    /// Unique ID for the slider.
    pub id: &'static str,

    /// Minimum slider value.
    pub min: f32,

    /// Maximum slider value.
    pub max: f32,

    /// Current slider value.
    pub value: f32,

    /// Size of thumb in pizels (along slider axis)
    pub thumb_size: f32,

    /// Closure which renders the slider elements.
    pub children: Arc<F>,

    /// Style handle for slider root element.
    pub style: S,
}

impl<V: View, F: Fn(SliderChildProps) -> V, S: StyleTuple> PartialEq for SliderProps<V, F, S> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.min == other.min
            && self.max == other.max
            && self.value == other.value
            && self.children.as_ref() as *const _ == other.children.as_ref() as *const _
    }
}

#[derive(Clone, PartialEq, Default, Copy)]
struct DragState {
    dragging: bool,
    offset: f32,
}

// Horizontal slider widget
pub fn h_slider<V: View, F: Fn(SliderChildProps) -> V, S: StyleTuple>(
    mut cx: Cx<SliderProps<V, F, S>>,
) -> impl View {
    let drag_state = cx.create_atom_init::<DragState>(|| DragState::default());
    // Pain point: Need to capture all props for closures.
    let id = cx.props.id;
    let thumb_size = cx.props.thumb_size;
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
        .styled(cx.props.style.clone())
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
                      mut writer: EventWriter<ValueChanged<f32>>| {
                    let ds = atoms.get(drag_state);
                    if ds.dragging {
                        match query.get(ev.listener()) {
                            Ok((node, transform)) => {
                                // Measure node width and slider value.
                                let slider_width =
                                    node.logical_rect(transform).width() - thumb_size;
                                let new_value = if range > 0. {
                                    ds.offset + (ev.distance.x * range) / slider_width
                                } else {
                                    min + range * 0.5
                                };
                                writer.send(ValueChanged::<f32> {
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
            percent: pos * 100.,
            min,
            max,
            value,
            is_dragging: cx.read_atom(drag_state).dragging,
        }))
}
