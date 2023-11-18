//! Example of a For view.

use bevy::prelude::*;
use quill::{Cx, Element, For, QuillPlugin, View, ViewHandle};

fn main() {
    App::new()
        .init_resource::<List>()
        .init_resource::<Random32>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(QuillPlugin)
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (bevy::window::close_on_esc, update_counter))
        .run();
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn(ViewHandle::new(root_presenter, ()));
}

const SUITS: &[&str] = &["hearts", "spades", "clubs", "diamonds"];

fn root_presenter(mut cx: Cx) -> impl View {
    let items = cx.use_resource::<List>();
    Element::new((
        "Suits: ",
        For::each(&items.items, |item| format!("[{}]", item)),
    ))
}

#[derive(Resource, Default)]
pub struct List {
    pub items: Vec<String>,
}

fn update_counter(
    mut counter: ResMut<List>,
    key: Res<Input<KeyCode>>,
    mut random: ResMut<Random32>,
) {
    if key.pressed(KeyCode::Space) {
        let i = (random.next() as usize) % SUITS.len();
        counter.items.push(SUITS[i].to_string());
        while counter.items.len() > 10 {
            counter.items.remove(0);
        }
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

#[derive(Resource)]
struct Random32 {
    state: u32,
}

impl Random32 {
    // Generate a pseudo-random number
    fn next(&mut self) -> u32 {
        // Constants for 32-bit LCG (example values, you might want to choose different ones)
        let a: u32 = 1664525; // Multiplier
        let c: u32 = 1013904223; // Increment
        let m: u32 = 2u32.pow(31); // Modulus, often set to 2^31 for a 32-bit generator

        // Simple LCG formula: X_{n+1} = (aX_n + c) mod m
        self.state = (a.wrapping_mul(self.state).wrapping_add(c)) % m;
        self.state
    }
}

impl Default for Random32 {
    fn default() -> Self {
        Self { state: 17 }
    }
}
