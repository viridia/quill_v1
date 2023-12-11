use bevy::{asset::AssetPath, prelude::*, ui};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::prelude::*;
use static_init::dynamic;

pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EventListenerPlugin::<OnChange<f32>>::default())
            .add_event::<OnChange<f32>>();
    }
}

const THUMB_SIZE: f32 = 18.;

// Style definitions for slider widget.

// The slider
#[dynamic]
static STYLE_SLIDER: StyleHandle =
    StyleHandle::build(|ss| ss.min_width(THUMB_SIZE).height(THUMB_SIZE));

// Slider track
#[dynamic]
static STYLE_TRACK_INNER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(Some(Color::BLACK))
        .position(ui::PositionType::Absolute)
        .top(ui::Val::Percent(40.))
        .bottom(ui::Val::Percent(40.))
        .left(0)
        .right(0)
});

#[dynamic]
static STYLE_TRACK_INNER_HLITE: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#447")
        .position(ui::PositionType::Absolute)
        .top(ui::Val::Percent(40.))
        .bottom(ui::Val::Percent(40.))
        .left(0)
});

#[dynamic]
static STYLE_TRACK: StyleHandle = StyleHandle::build(|ss| {
    ss.position(ui::PositionType::Absolute)
        .top(0)
        .bottom(0)
        .left(0)
        .right(THUMB_SIZE)
});

// Slider thumb
#[dynamic]
static STYLE_THUMB: StyleHandle = StyleHandle::build(|ss| {
    ss.position(ui::PositionType::Absolute)
        .top(0.)
        .width(THUMB_SIZE)
        .height(THUMB_SIZE)
});

// Slider thumb fg
#[dynamic]
static STYLE_THUMB_FG: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#777")
        .background_image(Some(AssetPath::from("disc.png")))
        .position(ui::PositionType::Absolute)
        .top(0.)
        .left(0.)
        .width(THUMB_SIZE)
        .height(THUMB_SIZE)
        .z_index(1)
        .pointer_events(PointerEvents::None)
        .selector(":hover > &,.drag > &", |ss| ss.background_color("#aaa"))
});

// Slider thumb shadow
#[dynamic]
static STYLE_THUMB_SHADOW: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#0008")
        .background_image(Some(AssetPath::from("disc.png")))
        .position(ui::PositionType::Absolute)
        .top(3.)
        .left(3.)
        .width(THUMB_SIZE)
        .height(THUMB_SIZE)
        .z_index(-1)
        .pointer_events(PointerEvents::None)
});

const CLS_DRAG: &str = "drag";

#[derive(Clone, PartialEq)]
pub struct SliderProps {
    pub id: &'static str,
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

#[derive(Clone, Event, EntityEvent)]
pub struct OnChange<T: Clone + Send + Sync + 'static> {
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

// Horizontal slider widget
pub fn h_slider(mut cx: Cx<SliderProps>) -> impl View {
    let drag_offset = cx.create_atom_init::<f32>(|| 0.);
    let is_dragging = cx.create_atom_init::<bool>(|| false);
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
        .styled(STYLE_SLIDER.clone())
        .with(move |mut e| {
            let eid = e.id();
            e.insert((
                On::<Pointer<DragStart>>::run(
                    move |ev: Listener<Pointer<DragStart>>,
                          mut atoms: AtomStore,
                          mut query: Query<&mut ElementClasses>| {
                        // Save initial value to use as drag offset.
                        atoms.set(drag_offset, value);
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
                          query: Query<(&Node, &GlobalTransform)>,
                          atoms: AtomStore,
                          mut writer: EventWriter<OnChange<f32>>| {
                        if atoms.get(is_dragging) {
                            match query.get(eid) {
                                Ok((node, transform)) => {
                                    // Measure node width and slider value.
                                    let slider_width =
                                        node.logical_rect(transform).width() - THUMB_SIZE;
                                    let new_value = if range > 0. {
                                        atoms.get(drag_offset)
                                            + (ev.distance.x * range) / slider_width
                                    } else {
                                        min + range * 0.5
                                    };
                                    writer.send(OnChange::<f32> {
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
                On::<Pointer<PointerCancel>>::listener_component_mut::<ElementClasses>(
                    |_, classes| {
                        println!("Splitter Cancel");
                        classes.remove_class(CLS_DRAG)
                    },
                ),
            ));
        })
        .children((
            Element::new().styled(STYLE_TRACK_INNER.clone()),
            Element::new().styled((
                STYLE_TRACK_INNER_HLITE.clone(),
                StyleHandle::build(|s| s.width(ui::Val::Percent(pos * 100.))),
            )),
            Element::new().styled(STYLE_TRACK.clone()).children(
                Element::new()
                    .styled((
                        STYLE_THUMB.clone(),
                        StyleHandle::build(|s| s.left(ui::Val::Percent(pos * 100.))),
                    ))
                    .children((
                        Element::new().styled(STYLE_THUMB_FG.clone()),
                        Element::new().styled(STYLE_THUMB_SHADOW.clone()),
                    )),
            ),
        ))
}
