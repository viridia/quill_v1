//! Example of styling.

use bevy::{prelude::*, ui};
use bevy_mod_picking::{
    backends::bevy_ui::BevyUiBackend,
    input::InputPlugin,
    picking_core::{CorePlugin, InteractionPlugin},
};
use bevy_quill::prelude::*;
use static_init::dynamic;

fn main() {
    App::new()
        .init_resource::<Counter>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
        .add_plugins(QuillPlugin)
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (bevy::window::close_on_esc, update_counter))
        .run();
}

#[dynamic]
static STYLE_MAIN: StyleHandle = StyleHandle::build(|ss| {
    ss.position(ui::PositionType::Absolute)
        .left(10.)
        .top(10.)
        .bottom(20.)
        .right(10.)
        .border(1)
        .border_color("#888")
        .display(ui::Display::Flex)
});

#[dynamic]
static STYLE_ASIDE: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#222")
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .width(200)
});

#[dynamic]
static STYLE_VSPLITTER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#181818")
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Center)
        .display(ui::Display::Flex)
        .width(7)
});

#[dynamic]
static STYLE_VSPLITTER_INNER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#282828")
        .display(ui::Display::Flex)
        .width(3)
        .height(ui::Val::Percent(30.))
});

#[dynamic]
static STYLE_EVEN: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(Some(Color::RED))
        .padding(UiRect::all(Val::Px(2.)))
});

#[dynamic]
static STYLE_ODD: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(Some(Color::GREEN))
        .padding(UiRect::all(Val::Px(2.)))
});

fn setup_view_root(mut commands: Commands) {
    commands.spawn(ViewHandle::new(ui_main, ()));
}

fn ui_main(cx: Cx) -> impl View {
    let counter = cx.use_resource::<Counter>();
    Element::new().styled(STYLE_MAIN.clone()).children((
        Element::new().children(()).styled(STYLE_ASIDE.clone()),
        v_splitter,
        If::new(counter.count & 1 == 0, even, odd),
    ))
}

fn v_splitter(mut _cx: Cx) -> impl View {
    Element::new()
        .styled(STYLE_VSPLITTER.clone())
        .children(Element::new().styled(STYLE_VSPLITTER_INNER.clone()))
}

fn even(mut _cx: Cx) -> impl View {
    Element::new().children("even").styled(STYLE_EVEN.clone())
}

fn odd(mut _cx: Cx) -> impl View {
    Element::new().children("odd").styled(STYLE_ODD.clone())
}

#[derive(Resource, Default)]
pub struct Counter {
    pub count: u32,
    pub foo: usize,
}

fn update_counter(mut counter: ResMut<Counter>, key: Res<ButtonInput<KeyCode>>) {
    if key.pressed(KeyCode::Space) {
        counter.count += 1;
    }
}

// Setup 3d shapes
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9_000_000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}
