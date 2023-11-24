use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, ui};
use bevy_mod_picking::{
    picking_core::{CorePlugin, InteractionPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
        .add_plugins(EventListenerPlugin::<Clicked>::default())
        .add_event::<Clicked>()
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (bevy::window::close_on_esc, show_events))
        .run();
}

fn setup_view_root(mut commands: Commands) {
    let main = commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: ui::PositionType::Absolute,
                    left: ui::Val::Px(10.),
                    top: ui::Val::Px(10.),
                    bottom: ui::Val::Px(10.),
                    right: ui::Val::Px(10.),
                    border: ui::UiRect::all(ui::Val::Px(1.)),
                    padding: ui::UiRect::all(ui::Val::Px(8.)),
                    display: ui::Display::Flex,
                    flex_direction: ui::FlexDirection::Column,
                    row_gap: ui::Val::Px(8.),
                    ..default()
                },
                border_color: BorderColor(Color::hex("#888").unwrap()),
                ..default()
            },
            On::<Clicked>::run(|ev: Res<ListenerInput<Clicked>>| {
                println!(
                    "<-- Received Clicked Button id='{}' target={:?}",
                    ev.id, ev.target
                );
            }),
        ))
        .id();

    let button1 = create_button(&mut commands, "One");
    let button2 = create_button(&mut commands, "Two");
    let button3 = create_button(&mut commands, "Three");
    commands
        .entity(main)
        .push_children(&[button1, button2, button3]);
}

fn create_button(commands: &mut Commands, id: &'static str) -> Entity {
    let button = commands
        .spawn((
            NodeBundle {
                style: Style {
                    border: ui::UiRect::all(ui::Val::Px(1.)),
                    display: ui::Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    min_height: ui::Val::Px(32.),
                    padding: ui::UiRect::horizontal(ui::Val::Px(8.)),
                    ..default()
                },
                background_color: BackgroundColor(Color::hex("#282828").unwrap()),
                border_color: BorderColor(Color::hex("#888").unwrap()),
                ..default()
            },
            On::<Pointer<Click>>::run(
                move |ev: Res<ListenerInput<Pointer<Click>>>, mut writer: EventWriter<Clicked>| {
                    println!("--> Sending Clicked id='{}' target={:?}", id, ev.target);
                    writer.send(Clicked {
                        target: ev.target,
                        id,
                    });
                },
            ),
        ))
        .id();
    let label = commands
        .spawn(TextBundle {
            text: Text::from_section(id, TextStyle { ..default() }),
            ..default()
        })
        .id();
    commands.entity(button).add_child(label);
    button
}

#[derive(Clone, Event, EntityEvent)]
pub struct Clicked {
    #[target] // Marks the field of the event that specifies the target entity
    pub target: Entity,
    pub id: &'static str,
}

fn show_events(mut clicked: EventReader<Clicked>) {
    for ev in clicked.read() {
        println!(
            "? Reading global clicked: id='{}' target={:?}",
            ev.id, ev.target
        );
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // HUD goes on top of 3D
                order: 1,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        UiCameraConfig { show_ui: true },
    ));
}
