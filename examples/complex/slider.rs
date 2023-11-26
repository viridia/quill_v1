use bevy::{asset::AssetPath, prelude::*, ui};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::prelude::*;
use lazy_static::lazy_static;

pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EventListenerPlugin::<OnChange<f32>>::default())
            .add_event::<OnChange<f32>>();
    }
}

const THUMB_SIZE: f32 = 18.;

// Style definitions for slider widget.
lazy_static! {
    // The slider
    static ref STYLE_SLIDER: StyleHandle = StyleHandle::build(|ss| ss
        .min_width(THUMB_SIZE)
        .height(THUMB_SIZE));
    // Slider track
    static ref STYLE_TRACK_INNER: StyleHandle = StyleHandle::build(|ss| ss
        .background_color(Some(Color::BLACK))
        .position(ui::PositionType::Absolute)
        .top(ui::Val::Percent(40.))
        .bottom(ui::Val::Percent(40.))
        .left(0)
        .right(0)
    );
    static ref STYLE_TRACK_INNER_HLITE: StyleHandle = StyleHandle::build(|ss| ss
        .background_color("#447")
        .position(ui::PositionType::Absolute)
        .top(ui::Val::Percent(40.))
        .bottom(ui::Val::Percent(40.))
        .left(0)
    );
    static ref STYLE_TRACK: StyleHandle = StyleHandle::build(|ss| ss
        .position(ui::PositionType::Absolute)
        .top(0)
        .bottom(0)
        .left(0)
        .right(THUMB_SIZE)
    );
    // Slider thumb
    static ref STYLE_THUMB: StyleHandle = StyleHandle::build(|ss| ss
        .position(ui::PositionType::Absolute)
        .top(0.)
        .width(THUMB_SIZE)
        .height(THUMB_SIZE)
    );
    // Slider thumb fg
    static ref STYLE_THUMB_FG: StyleHandle = StyleHandle::build(|ss| ss
        .background_color("#777")
        .background_image(Some(AssetPath::from("disc.png")))
        .position(ui::PositionType::Absolute)
        .top(0.)
        .left(0.)
        .width(THUMB_SIZE)
        .height(THUMB_SIZE)
        .z_index(1)
        .pointer_events(PointerEvents::None)
        .selector(":hover > &", |ss| ss
            .background_color("#aaa"))
    );
    // Slider thumb shadow
    static ref STYLE_THUMB_SHADOW: StyleHandle = StyleHandle::build(|ss| ss
        .background_color("#0008")
        .background_image(Some(AssetPath::from("disc.png")))
        .position(ui::PositionType::Absolute)
        .top(3.)
        .left(3.)
        .width(THUMB_SIZE)
        .height(THUMB_SIZE)
        .z_index(-1)
        .pointer_events(PointerEvents::None)
    );
}

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
pub fn h_slider(cx: Cx<SliderProps>) -> impl View {
    let id = cx.props.id;
    let pos = if cx.props.max > cx.props.min {
        cx.props.value / (cx.props.max - cx.props.min)
    } else {
        0.
    }
    .clamp(0., 1.);
    Element::new()
        .styled(STYLE_SLIDER.clone())
        .once(move |entity, world| {
            let mut e = world.entity_mut(entity);
            e.insert((
                On::<Pointer<DragStart>>::run(
                    move |ev: Res<ListenerInput<Pointer<DragStart>>>,
                          mut query: Query<&mut ElementClasses>| {
                        // println!("Start drag offset: {}", current_offset);
                        // Save initial value to use as drag offset.
                        // drag_offset_1.set(current_offset);
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
                          mut writer: EventWriter<OnChange<f32>>| {
                        writer.send(OnChange::<f32> {
                            target: ev.target,
                            id,
                            value: ev.distance.x + 0.,
                            finish: false,
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
