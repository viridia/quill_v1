use bevy::{prelude::*, ui};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::{prelude::*, scroll_system, ScrollContent, Scrolling};
use static_init::dynamic;

pub struct ScrollViewPlugin;

impl Plugin for ScrollViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scroll_system);
        // app.add_plugins(EventListenerPlugin::<OnChange<f32>>::default())
        //     .add_event::<OnChange<f32>>();
    }
}

// Style definitions for slider widget.

// The slider
#[dynamic]
static STYLE_SCROLL_VIEW: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Grid)
        .grid_template_columns(vec![
            ui::RepeatedGridTrack::flex(1, 1.),
            ui::RepeatedGridTrack::auto(1),
        ])
        .grid_template_rows(vec![
            ui::RepeatedGridTrack::flex(1, 1.),
            ui::RepeatedGridTrack::auto(1),
        ])
        .gap(3)
});

// Slider track
#[dynamic]
static STYLE_SCROLL_CONTENT: StyleHandle = StyleHandle::build(|ss| {
    ss.border(1)
        .grid_column(ui::GridPlacement::start_span(1, 1))
        .grid_row(ui::GridPlacement::start_span(1, 1))
        .overflow(ui::OverflowAxis::Clip)
});

#[dynamic]
static STYLE_SCROLLBAR_X: StyleHandle = StyleHandle::build(|ss| {
    ss.grid_column(ui::GridPlacement::start_span(1, 1))
        .grid_row(ui::GridPlacement::start_span(2, 1))
        .min_height(6)
});

#[dynamic]
static STYLE_SCROLLBAR_X_THUMB: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#334")
        .position(ui::PositionType::Absolute)
        .left(ui::Val::Percent(40.))
        .width(ui::Val::Percent(40.))
        .top(0)
        .bottom(0)
        .selector(":hover > &", |ss| ss.background_color("#556"))
});

#[dynamic]
static STYLE_SCROLLBAR_Y: StyleHandle = StyleHandle::build(|ss| {
    ss.grid_column(ui::GridPlacement::start_span(2, 1))
        .grid_row(ui::GridPlacement::start_span(1, 1))
        .min_width(6)
});

#[dynamic]
static STYLE_SCROLLBAR_Y_THUMB: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#334")
        .position(ui::PositionType::Absolute)
        .top(ui::Val::Percent(40.))
        .bottom(ui::Val::Percent(40.))
        .left(0)
        .right(0)
        .selector(":hover > &", |ss| ss.background_color("#556"))
});

#[derive(Clone, PartialEq, Default)]
pub struct ScrollViewProps<V: View> {
    pub scroll_enable_x: bool,
    pub scroll_enable_y: bool,
    pub children: V,
    pub style: StyleHandle,
}

// Horizontal slider widget
pub fn scroll_view<V: View + Clone>(cx: Cx<ScrollViewProps<V>>) -> impl View {
    let enable_x = cx.props.scroll_enable_x;
    let enable_y = cx.props.scroll_enable_y;
    Element::new()
        .styled((STYLE_SCROLL_VIEW.clone(), cx.props.style.clone()))
        .once(move |mut e| {
            e.insert(Scrolling {
                enable_x,
                enable_y,
                ..default()
            });
        })
        .with(move |mut e| {
            let eid = e.id();
            e.insert((
                On::<Pointer<DragStart>>::listener_component_mut::<Scrolling>(
                    move |ev, scrolling| {
                        println!("Drag");
                        // Save initial value to use as drag offset.
                    },
                ),
                //     On::<Pointer<DragStart>>::run(
                //         move |ev: Listener<Pointer<DragStart>>,
                //               mut query: Query<&mut ElementClasses>| {
                //             // Save initial value to use as drag offset.
                //             drag_offset_1.set(value);
                //             is_dragging_1.set(true);
                //             if let Ok(mut classes) = query.get_mut(ev.target) {
                //                 classes.add_class(CLS_DRAG)
                //             }
                //         },
                //     ),
                //     On::<Pointer<DragEnd>>::listener_component_mut::<ElementClasses>(
                //         move |_, classes| {
                //             is_dragging_2.set(false);
                //             classes.remove_class(CLS_DRAG)
                //         },
                //     ),
                //     On::<Pointer<Drag>>::run(
                //         move |ev: Listener<Pointer<Drag>>,
                //               query: Query<(&Node, &GlobalTransform)>,
                //               mut writer: EventWriter<OnChange<f32>>| {
                //             if is_dragging_3.get() {
                //                 match query.get(eid) {
                //                     Ok((node, transform)) => {
                //                         // Measure node width and slider value.
                //                         let slider_width =
                //                             node.logical_rect(transform).width() - THUMB_SIZE;
                //                         let new_value = if range > 0. {
                //                             drag_offset_2.get() + (ev.distance.x * range) / slider_width
                //                         } else {
                //                             min + range * 0.5
                //                         };
                //                         writer.send(OnChange::<f32> {
                //                             target: ev.target,
                //                             id,
                //                             value: new_value.clamp(min, max),
                //                             finish: false,
                //                         });
                //                     }
                //                     _ => return,
                //                 }
                //             }
                //         },
                //     ),
                //     On::<Pointer<PointerCancel>>::listener_component_mut::<ElementClasses>(
                //         |_, classes| {
                //             println!("Splitter Cancel");
                //             classes.remove_class(CLS_DRAG)
                //         },
                //     ),
            ));
        })
        .children((
            Element::new()
                .once(move |mut e| {
                    e.insert(ScrollContent);
                })
                .styled(STYLE_SCROLL_CONTENT.clone())
                .children(cx.props.children.clone()),
            If::new(
                cx.props.scroll_enable_x,
                Element::new()
                    .styled(STYLE_SCROLLBAR_Y.clone())
                    .children(Element::new().styled(STYLE_SCROLLBAR_Y_THUMB.clone())),
                (),
            ),
            If::new(
                cx.props.scroll_enable_y,
                Element::new()
                    .styled(STYLE_SCROLLBAR_X.clone())
                    .children(Element::new().styled(STYLE_SCROLLBAR_X_THUMB.clone())),
                (),
            ),
        ))
}
