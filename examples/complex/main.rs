//! Complex example with multiple views
mod shapes;
mod splitter;
mod viewport;

use std::sync::Arc;

use bevy::{prelude::*, ui};
use bevy_mod_picking::{
    events::PointerCancel,
    picking_core::{CorePlugin, InteractionPlugin},
    prelude::*,
};
use lazy_static::lazy_static;
use quill::{
    Cx, Element, ElementClasses, PresenterFn, QuillPlugin, StyleSet, TrackedResources, View,
    ViewHandle,
};
use splitter::{v_splitter, SplitterDragged, SplitterProps};
use viewport::{ViewportInset, ViewportInsetElement};

fn main() {
    App::new()
        .init_resource::<ViewportInset>()
        .init_resource::<PanelWidth>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
        .add_plugins(EventListenerPlugin::<SplitterDragged>::default())
        .add_plugins(EventListenerPlugin::<Clicked>::default())
        .add_plugins(QuillPlugin)
        .add_systems(Startup, (shapes::setup, setup_view_root))
        .add_event::<Clicked>()
        .add_event::<SplitterDragged>()
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                shapes::rotate,
                shapes::update_viewport_inset,
                shapes::update_camera_viewport,
            ),
        )
        .run();
}

lazy_static! {
    static ref STYLE_MAIN: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .position(ui::PositionType::Absolute)
        .left(10.)
        .top(10.)
        .bottom(10)
        .right(10.)
        .border(1)
        .border_color(Some(Color::hex("#888").unwrap()))
        .display(ui::Display::Flex)));
    static ref STYLE_ASIDE: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#222").unwrap()))
        .display(ui::Display::Flex)
        .padding(8)
        .gap(8)
        .flex_direction(ui::FlexDirection::Column)
        .width(200)));
    static ref STYLE_BUTTON: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
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
        .selector(".hover", |ss| ss
            .border_color(Some(Color::hex("#444").unwrap()))
            .background_color(Some(Color::hex("#2F2F2F").unwrap())))
        .selector(".hover.pressed", |ss| ss
            .background_color(Some(Color::hex("#484848").unwrap())))));
    static ref STYLE_VIEWPORT: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss.flex_grow(1.)));
}

const CLS_HOVER: &str = "hover";
const CLS_PRESSED: &str = "pressed";

#[derive(Resource)]
pub struct PanelWidth(pub i32);

impl Default for PanelWidth {
    fn default() -> Self {
        Self(160)
    }
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn((TrackedResources::default(), ViewHandle::new(ui_main, ())));
}

fn ui_main(mut cx: Cx) -> impl View {
    let width = cx.use_resource::<PanelWidth>();
    Element::new((
        Element::new((
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
        ))
        .styled((
            STYLE_ASIDE.clone(),
            Arc::new(StyleSet::build(|b| b.width(width.0))),
        ))
        .once(|entity, world| {
            let mut e = world.entity_mut(entity);
            e.insert((
                On::<SplitterDragged>::run(|ev: Res<ListenerInput<SplitterDragged>>| {
                    println!("Dragged {}", ev.id);
                }),
                On::<Clicked>::run(|ev: Res<ListenerInput<Clicked>>| {
                    println!("Clicked button {}", ev.id);
                }),
            ));
        }),
        v_splitter.bind(SplitterProps { id: "" }),
        Element::new(())
            .styled(STYLE_VIEWPORT.clone())
            .insert(ViewportInsetElement {}),
    ))
    .styled(STYLE_MAIN.clone())
}

#[derive(Clone)]
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
    Element::new(cx.props.children.clone())
        .once(move |entity, world| {
            let mut e = world.entity_mut(entity);
            e.insert((
                On::<Pointer<Click>>::run(
                    move |ev: Res<ListenerInput<Pointer<Click>>>,
                          mut writer: EventWriter<Clicked>| {
                        writer.send(Clicked {
                            target: ev.target,
                            id,
                        });
                    },
                ),
                On::<Pointer<Over>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    classes.add_class(CLS_HOVER)
                }),
                On::<Pointer<Out>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    classes.remove_class(CLS_HOVER)
                }),
                On::<Pointer<DragStart>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    classes.add_class(CLS_PRESSED)
                }),
                On::<Pointer<DragEnd>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    classes.remove_class(CLS_PRESSED)
                }),
                On::<Pointer<PointerCancel>>::listener_component_mut::<ElementClasses>(
                    |_, classes| {
                        println!("Cancel");
                        classes.remove_class(CLS_HOVER);
                        classes.remove_class(CLS_PRESSED)
                    },
                ),
            ));
        })
        .styled(STYLE_BUTTON.clone())
}
