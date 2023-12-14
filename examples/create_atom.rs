//! Example of nested presenter functions.

use bevy::prelude::*;
use bevy_mod_picking::{
    backends::bevy_ui::BevyUiBackend,
    events::Click,
    input::InputPlugin,
    picking_core::{CorePlugin, InteractionPlugin},
    prelude::*,
};
use bevy_quill::prelude::*;

fn main() {
    App::new()
        .init_resource::<Counter>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
        .add_plugins(QuillPlugin)
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn(ViewHandle::new(root_presenter, ()));
}

fn root_presenter(mut _cx: Cx) -> impl View {
    Element::new().children(("Root Presenter: ", nested.bind("Fred")))
}

fn nested(mut cx: Cx<&str>) -> impl View {
    let name = *cx.props;
    let counter = cx.create_atom_init::<i32>(|| 0);
    Element::new()
        .children((
            "Nested Presenter: ",
            format!("{}: {}", name, cx.read_atom(counter)),
            If::new(cx.read_atom(counter) & 1 == 0, even, odd),
        ))
        .insert(On::<Pointer<Click>>::run(
            move |_ev: Listener<Pointer<Click>>, mut atoms: AtomStore| {
                atoms.update(counter, |n| n + 1)
            },
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
