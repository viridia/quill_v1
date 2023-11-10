//! Example of nested presenter functions.

use bevy::prelude::*;
use quill::{Bind, Cx, Element, If, QuillPlugin, TrackedResources, View, ViewHandle};

fn main() {
    App::new()
        .init_resource::<Counter>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(QuillPlugin)
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (bevy::window::close_on_esc, update_counter))
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

fn setup_view_root(mut commands: Commands) {
    commands.spawn((
        TrackedResources::default(),
        ViewHandle::new(root_presenter, ()),
    ));
}

fn root_presenter(mut _cx: Cx) -> impl View {
    Element::new(("Root Presenter: ", Bind::new(nested, "Fred")))
}

fn nested(mut cx: Cx<&str>) -> impl View {
    let name = *cx.props;
    let counter = cx.use_resource::<Counter>();
    Element::new((
        "Nested Presenter: ",
        format!("{}: {}", name, counter.count),
        If::new(
            counter.count & 1 == 0,
            Bind::new(even, ()),
            Bind::new(odd, ()),
        ),
    ))
}

fn even(mut _cx: Cx) -> impl View {
    " [even]"
}

fn odd(mut _cx: Cx) -> impl View {
    " [odd]"
}

#[derive(Resource, Default)]
pub struct Counter {
    pub count: u32,
    pub foo: usize,
}

fn update_counter(mut counter: ResMut<Counter>, key: Res<Input<KeyCode>>) {
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
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}
