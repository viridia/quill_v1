use bevy::{prelude::*, ui};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::{prelude::*, ScrollArea, ScrollBar, ScrollBarThumb, ScrollContent, ScrollWheel};
use static_init::dynamic;

// Style definitions for scrollview widget.

// The combined scroll view with scrolling region and scrollbars.
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
        .gap(2)
});

/// The scrolling region which defines the clipping bounds.
#[dynamic]
static STYLE_SCROLL_REGION: StyleHandle = StyleHandle::build(|ss| {
    ss.grid_column(ui::GridPlacement::start_span(1, 1))
        .grid_row(ui::GridPlacement::start_span(1, 1))
        .overflow(ui::OverflowAxis::Clip)
});

/// The scrolling content which is clipped.
#[dynamic]
static STYLE_SCROLL_CONTENT: StyleHandle = StyleHandle::build(|ss| {
    ss.position(ui::PositionType::Absolute)
        .height(ui::Val::Auto)
});

#[dynamic]
static STYLE_SCROLLBAR_X: StyleHandle = StyleHandle::build(|ss| {
    ss.grid_column(ui::GridPlacement::start_span(1, 1))
        .grid_row(ui::GridPlacement::start_span(2, 1))
        .height(8)
});

#[dynamic]
static STYLE_SCROLLBAR_X_THUMB: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#334")
        .position(ui::PositionType::Absolute)
        .top(1)
        .bottom(1)
        .selector(":hover > &,.drag", |ss| ss.background_color("#556"))
});

#[dynamic]
static STYLE_SCROLLBAR_Y: StyleHandle = StyleHandle::build(|ss| {
    ss.grid_column(ui::GridPlacement::start_span(2, 1))
        .grid_row(ui::GridPlacement::start_span(1, 1))
        .width(8)
});

#[dynamic]
static STYLE_SCROLLBAR_Y_THUMB: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#334")
        .position(ui::PositionType::Absolute)
        .left(1)
        .right(1)
        .selector(":hover > &,.drag", |ss| ss.background_color("#556"))
});

const CLS_DRAG: &str = "drag";

#[derive(Clone, PartialEq, Default)]
pub struct ScrollViewProps<V: View> {
    pub children: V,
    pub style: StyleHandle,
    pub scroll_enable_x: bool,
    pub scroll_enable_y: bool,
}

#[derive(Clone, PartialEq, Default, Copy)]
enum DragMode {
    #[default]
    None,
    DragX,
    DragY,
}

#[derive(Clone, PartialEq, Default, Copy)]
struct DragState {
    mode: DragMode,
    offset: f32,
}

// A widget which displays a scrolling view of its children.
pub fn scroll_view<V: View + Clone>(mut cx: Cx<ScrollViewProps<V>>) -> impl View {
    let enable_x = cx.props.scroll_enable_x;
    let enable_y = cx.props.scroll_enable_y;
    let id_scroll_area = cx.create_entity();
    let id_scrollbar_x = cx.create_entity();
    let id_scrollbar_y = cx.create_entity();
    let drag_state = cx.create_atom_init(|| DragState::default());
    Element::new()
        .styled((STYLE_SCROLL_VIEW.clone(), cx.props.style.clone()))
        .children((
            // Scroll area
            RefElement::new(id_scroll_area)
                .once(move |mut e| {
                    e.insert((
                        ScrollArea {
                            id_scrollbar_x: if enable_x { Some(id_scrollbar_x) } else { None },
                            id_scrollbar_y: if enable_y { Some(id_scrollbar_y) } else { None },
                            ..default()
                        },
                        On::<ScrollWheel>::listener_component_mut::<ScrollArea>(
                            move |ev, scrolling| {
                                // TODO: stop prop
                                // ev.stop_propagation();
                                scrolling.scroll_by(-ev.delta.x, -ev.delta.y);
                            },
                        ),
                    ));
                })
                .styled(STYLE_SCROLL_REGION.clone())
                .children(
                    Element::new()
                        .once(move |mut e| {
                            e.insert(ScrollContent);
                        })
                        .styled(STYLE_SCROLL_CONTENT.clone())
                        .children(cx.props.children.clone()),
                ),
            // Horizontal scroll bar
            If::new(
                cx.props.scroll_enable_x,
                scrollbar.bind(ScrollbarProps {
                    id_scroll_area,
                    id_scrollbar: id_scrollbar_x,
                    drag_state,
                    vertical: false,
                }),
                (),
            ),
            // Vertical scroll bar
            If::new(
                cx.props.scroll_enable_y,
                scrollbar.bind(ScrollbarProps {
                    id_scroll_area,
                    id_scrollbar: id_scrollbar_y,
                    drag_state,
                    vertical: true,
                }),
                (),
            ),
        ))
}

#[derive(Clone, PartialEq)]
pub struct ScrollbarProps {
    id_scroll_area: Entity,
    id_scrollbar: Entity,
    drag_state: AtomHandle<DragState>,
    vertical: bool,
}

fn scrollbar(mut cx: Cx<ScrollbarProps>) -> impl View {
    let vertical = cx.props.vertical;
    let drag_state = cx.props.drag_state;
    let id_scroll_area = cx.props.id_scroll_area;
    let id_thumb = cx.create_entity();
    let mode = if vertical {
        DragMode::DragY
    } else {
        DragMode::DragX
    };
    RefElement::new(cx.props.id_scrollbar)
        .once(move |mut e| {
            e.insert(
                (
                    ScrollBar {
                        id_scroll_area,
                        vertical,
                        min_thumb_size: 10.,
                    },
                    // Click outside of thumb
                    On::<Pointer<DragStart>>::run(
                        move |mut ev: ListenerMut<Pointer<DragStart>>,
                              mut query: Query<&mut ScrollArea>,
                              query_thumb: Query<(
                            &Node,
                            &mut ScrollBarThumb,
                            &GlobalTransform,
                        )>| {
                            ev.stop_propagation();
                            if let Ok(mut scroll_area) = query.get_mut(id_scroll_area) {
                                if let Ok((thumb, _, transform)) = query_thumb.get(id_thumb) {
                                    // Get thumb rectangle
                                    let rect = thumb.logical_rect(transform);
                                    handle_track_click(
                                        &mut scroll_area,
                                        vertical,
                                        ev.pointer_location.position,
                                        rect,
                                    );
                                }
                            };
                        },
                    ),
                ),
            );
        })
        .styled(if vertical {
            STYLE_SCROLLBAR_Y.clone()
        } else {
            STYLE_SCROLLBAR_X.clone()
        })
        .children(
            RefElement::new(id_thumb)
                .class_names(CLS_DRAG.if_true(cx.read_atom(drag_state).mode == mode))
                .styled(if vertical {
                    STYLE_SCROLLBAR_Y_THUMB.clone()
                } else {
                    STYLE_SCROLLBAR_X_THUMB.clone()
                })
                .once(move |mut e| {
                    e.insert((
                        ScrollBarThumb,
                        // Click/Drag on thumb
                        On::<Pointer<DragStart>>::run(
                            move |mut ev: ListenerMut<Pointer<DragStart>>,
                                  mut atoms: AtomStore,
                                  query: Query<&mut ScrollArea>| {
                                ev.stop_propagation();
                                if let Ok(scroll_area) = query.get(id_scroll_area) {
                                    handle_thumb_drag_start(
                                        &scroll_area,
                                        vertical,
                                        &mut atoms,
                                        drag_state,
                                    );
                                };
                            },
                        ),
                        On::<Pointer<Drag>>::run(
                            move |mut ev: ListenerMut<Pointer<Drag>>,
                                  atoms: AtomStore,
                                  mut query: Query<&mut ScrollArea>| {
                                ev.stop_propagation();
                                if let Ok(mut scroll_area) = query.get_mut(id_scroll_area) {
                                    if let Some(ds) = atoms.try_get(drag_state) {
                                        handle_thumb_drag(&mut scroll_area, &ds, ev.distance);
                                    }
                                }
                            },
                        ),
                        On::<Pointer<DragEnd>>::run(
                            move |mut ev: ListenerMut<Pointer<DragEnd>>, mut atoms: AtomStore| {
                                ev.stop_propagation();
                                handle_thumb_drag_end(&mut atoms, drag_state);
                            },
                        ),
                        On::<Pointer<PointerCancel>>::run(
                            move |mut ev: ListenerMut<Pointer<DragEnd>>, mut atoms: AtomStore| {
                                ev.stop_propagation();
                                handle_thumb_drag_end(&mut atoms, drag_state);
                            },
                        ),
                    ));
                }),
        )
}

fn handle_thumb_drag_start(
    scroll_area: &ScrollArea,
    vertical: bool,
    atoms: &mut AtomStore,
    drag_state: AtomHandle<DragState>,
) {
    if vertical {
        atoms.set(
            drag_state,
            DragState {
                mode: DragMode::DragY,
                offset: scroll_area.scroll_top,
            },
        );
    } else {
        atoms.set(
            drag_state,
            DragState {
                mode: DragMode::DragX,
                offset: scroll_area.scroll_left,
            },
        );
    }
}

fn handle_thumb_drag(scroll_area: &mut ScrollArea, ds: &DragState, distance: Vec2) {
    if ds.mode == DragMode::DragY {
        let left = scroll_area.scroll_left;
        let top = if scroll_area.visible_size.y > 0. {
            ds.offset + distance.y * scroll_area.content_size.x / scroll_area.visible_size.y
        } else {
            0.
        };
        scroll_area.scroll_to(left, top);
    } else if ds.mode == DragMode::DragX {
        let top = scroll_area.scroll_top;
        let left = if scroll_area.visible_size.x > 0. {
            ds.offset + distance.x * scroll_area.content_size.y / scroll_area.visible_size.x
        } else {
            0.
        };
        scroll_area.scroll_to(left, top);
    };
}

fn handle_thumb_drag_end(atoms: &mut AtomStore, drag_state: AtomHandle<DragState>) {
    atoms.set(
        drag_state,
        DragState {
            mode: DragMode::None,
            offset: 0.,
        },
    );
}

fn handle_track_click(scroll_area: &mut ScrollArea, vertical: bool, position: Vec2, rect: Rect) {
    if vertical {
        let page_size = scroll_area.visible_size.y;
        if position.y >= rect.max.y {
            scroll_area.scroll_by(0., page_size);
        } else if position.y < rect.min.y {
            scroll_area.scroll_by(0., -page_size);
        }
    } else {
        let page_size = scroll_area.visible_size.x;
        if position.x >= rect.max.x {
            scroll_area.scroll_by(page_size, 0.);
        } else if position.x < rect.min.x {
            scroll_area.scroll_by(-page_size, 0.);
        }
    }
}
