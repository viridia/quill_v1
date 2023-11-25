//! Complex example with multiple views
mod shapes;
mod splitter;
mod swatch;
mod viewport;

use bevy::{prelude::*, ui};
use bevy_mod_picking::{
    events::PointerCancel,
    picking_core::{CorePlugin, InteractionPlugin},
    prelude::*,
};
use bevy_quill::prelude::*;
use lazy_static::lazy_static;
use splitter::{v_splitter, SplitterDragged, SplitterPlugin, SplitterProps};
use swatch::{swatch, swatch_grid, SwatchGridProps, SwatchProps};
use viewport::{ViewportInset, ViewportInsetElement};

fn main() {
    App::new()
        .init_resource::<ViewportInset>()
        .init_resource::<PanelWidth>()
        .init_resource::<ClickLog>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
        .add_plugins(EventListenerPlugin::<Clicked>::default())
        .add_plugins(QuillPlugin)
        .add_plugins(SplitterPlugin)
        .add_systems(Startup, (shapes::setup, setup_view_root))
        .add_event::<Clicked>()
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                shapes::rotate,
                shapes::update_viewport_inset,
                shapes::update_camera_viewport,
                show_events,
            ),
        )
        .run();
}

lazy_static! {
    static ref STYLE_MAIN: StyleHandle = StyleHandle::build(|ss| ss
        .position(ui::PositionType::Absolute)
        .left(10.)
        .top(10.)
        .bottom(10)
        .right(10.)
        .border(1)
        .border_color(Some(Color::hex("#888").unwrap()))
        .display(ui::Display::Flex));
    static ref STYLE_ASIDE: StyleHandle = StyleHandle::build(|ss| ss
        .background_color(Some(Color::hex("#222").unwrap()))
        .display(ui::Display::Flex)
        .padding(8)
        .gap(8)
        .flex_direction(ui::FlexDirection::Column)
        .width(200));
    static ref STYLE_BUTTON: StyleHandle = StyleHandle::build(|ss| ss
        .background_color(Some(Color::hex("#282828").unwrap()))
        .border_color(Some(Color::hex("#383838").unwrap()))
        .border(1)
        .display(ui::Display::Flex)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .min_height(32)
        .padding_left(8)
        .padding_right(8)
        .selector(".pressed", |ss| ss
            .background_color(Some(Color::hex("#404040").unwrap())))
        .selector(":hover", |ss| ss
            .border_color(Some(Color::hex("#444").unwrap()))
            .background_color(Some(Color::hex("#2F2F2F").unwrap())))
        .selector(":hover.pressed", |ss| ss
            .background_color(Some(Color::hex("#484848").unwrap()))));
    static ref STYLE_VIEWPORT: StyleHandle = StyleHandle::build(|ss| ss
        .flex_grow(1.)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .justify_content(ui::JustifyContent::FlexEnd));
    static ref STYLE_LOG: StyleHandle = StyleHandle::build(|ss| ss
        .background_color(Some(Color::hex("#0008").unwrap()))
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_self(ui::AlignSelf::Stretch)
        .height(ui::Val::Percent(30.))
        .margin(8));
    static ref STYLE_LOG_INNER: StyleHandle = StyleHandle::build(|ss| ss
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .justify_content(ui::JustifyContent::FlexEnd)
        .align_self(ui::AlignSelf::Stretch)
        .flex_grow(1.)
        .flex_basis(0)
        .overflow(ui::OverflowAxis::Clip)
        .gap(3)
        .margin(8));
    static ref STYLE_LOG_ENTRY: StyleHandle = StyleHandle::build(|ss| ss
        .display(ui::Display::Flex)
        .justify_content(ui::JustifyContent::SpaceBetween)
        .align_self(ui::AlignSelf::Stretch));
    static ref COLORS: Vec<Color> = vec![
        Color::hex("#fff").unwrap(),
        Color::hex("#ffc").unwrap(),
        Color::hex("#ff8").unwrap(),
        Color::hex("#ff4").unwrap(),
        Color::hex("#ff0").unwrap(),
        Color::hex("#fcf").unwrap(),
        Color::hex("#fcc").unwrap(),
        Color::hex("#fc8").unwrap(),
        Color::hex("#fc4").unwrap(),
        Color::hex("#fc0").unwrap(),
        Color::hex("#f8f").unwrap(),
        Color::hex("#f8c").unwrap(),
        Color::hex("#f88").unwrap(),
        Color::hex("#f84").unwrap(),
        Color::hex("#f80").unwrap(),
        Color::hex("#f4f").unwrap(),
        Color::hex("#f4c").unwrap(),
        Color::hex("#f48").unwrap(),
        Color::hex("#f44").unwrap(),
        Color::hex("#f40").unwrap(),
        Color::hex("#f0f").unwrap(),
        Color::hex("#f0c").unwrap(),
        Color::hex("#f08").unwrap(),
        Color::hex("#f04").unwrap(),
        Color::hex("#f00").unwrap(),
    ];
}

const CLS_PRESSED: &str = "pressed";

#[derive(Resource)]
pub struct PanelWidth {
    value: f32,
}

impl Default for PanelWidth {
    fn default() -> Self {
        Self { value: 160. }
    }
}

#[derive(Resource, Default)]
pub struct ClickLog(Vec<String>);

fn setup_view_root(mut commands: Commands) {
    commands.spawn(ViewHandle::new(ui_main, ()));
}

fn ui_main(mut cx: Cx) -> impl View {
    let width = cx.use_resource::<PanelWidth>();
    // let log = cx.use_resource_mut::<ClickLog>();
    Element::new()
        .styled(STYLE_MAIN.clone())
        .once(|entity, world| {
            let mut e = world.entity_mut(entity);
            e.insert((On::<SplitterDragged>::run(
                move |ev: Res<ListenerInput<SplitterDragged>>,
                      mut width: ResMut<PanelWidth>,
                      query: Query<(&Node, &GlobalTransform)>| {
                    match query.get(entity) {
                        Ok((node, transform)) => {
                            // Measure node width and clamp split position.
                            let node_width = node.logical_rect(transform).width();
                            width.value = ev.value.clamp(100., node_width - 100.);
                        }
                        _ => return,
                    }
                },
            ),));
        })
        .children((
            Element::new()
                .styled((
                    STYLE_ASIDE.clone(),
                    StyleHandle::build(|b| b.width(width.value.floor())),
                ))
                .once(|entity, world| {
                    let mut e = world.entity_mut(entity);
                    println!("Adding event handlers");
                    e.insert(On::<Clicked>::run(|ev: Res<ListenerInput<Clicked>>| {
                        println!(
                            "Received Clicked Button id='{}' target={:?}",
                            ev.id, ev.target
                        );
                    }));
                })
                .children((
                    button.bind(ButtonProps {
                        id: "save",
                        children: "Save",
                    }),
                    button.bind(ButtonProps {
                        id: "load",
                        children: "Load",
                    }),
                    button.bind(ButtonProps {
                        id: "quit",
                        children: "Quit",
                    }),
                    Fragment::new((
                        swatch.bind(SwatchProps { color: Color::RED }),
                        swatch_grid.bind(SwatchGridProps {
                            colors: &COLORS,
                            row_span: 4,
                        }),
                    )),
                )),
            v_splitter.bind(SplitterProps {
                id: "",
                value: width.value,
            }),
            Element::new()
                .styled(STYLE_VIEWPORT.clone())
                .insert(ViewportInsetElement {})
                .children(event_log),
        ))
}

#[derive(Clone, PartialEq)]
pub struct ButtonProps<V: View> {
    pub id: &'static str,
    pub children: V,
}

#[derive(Clone, Event, EntityEvent)]
pub struct Clicked {
    #[target] // Marks the field of the event that specifies the target entity
    pub target: Entity,
    pub id: &'static str,
}

fn button<V: View + Clone>(cx: Cx<ButtonProps<V>>) -> impl View {
    // Needs to be a local variable so that it can be captured in the event handler.
    let id = cx.props.id;
    Element::new()
        .once(move |entity, world| {
            let mut e = world.entity_mut(entity);
            e.insert((
                On::<Pointer<Click>>::run(
                    move |ev: Res<ListenerInput<Pointer<Click>>>,
                          mut writer: EventWriter<Clicked>| {
                        println!("Sending Clicked id='{}' target={:?}", id, ev.target);
                        writer.send(Clicked {
                            target: ev.target,
                            id,
                        });
                    },
                ),
                On::<Pointer<DragStart>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    classes.add_class(CLS_PRESSED)
                }),
                On::<Pointer<DragEnd>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    classes.remove_class(CLS_PRESSED)
                }),
                On::<Pointer<PointerCancel>>::listener_component_mut::<ElementClasses>(
                    |_, classes| {
                        println!("Cancel");
                        classes.remove_class(CLS_PRESSED)
                    },
                ),
            ));
        })
        .styled(STYLE_BUTTON.clone())
        .children(cx.props.children.clone())
}

fn event_log(mut cx: Cx) -> impl View {
    let log = cx.use_resource::<ClickLog>();
    Element::new().styled(STYLE_LOG.clone()).children(
        Element::new()
            .styled(STYLE_LOG_INNER.clone())
            .children(For::each(&log.0, |item| {
                Element::new()
                    .styled(STYLE_LOG_ENTRY.clone())
                    .children((item.to_owned(), "00:00:00"))
            })),
    )
}

fn show_events(mut clicked: EventReader<Clicked>, mut log: ResMut<ClickLog>) {
    for ev in clicked.read() {
        println!(
            "Reading global clicked: id='{}' target={:?}",
            ev.id, ev.target
        );
        log.0
            .push(format!("Button Clicked: id='{}'", ev.id).to_string())
    }
}
