//! Example that shows how to add custom ECS components to a Quill View.

use std::f32::consts::PI;
use std::sync::Arc;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::Viewport,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    ui,
};
use bevy_mod_picking::{
    events::PointerCancel,
    picking_core::{CorePlugin, InteractionPlugin},
    prelude::*,
};
use bevy_quill::prelude::*;
use lazy_static::lazy_static;

fn main() {
    App::new()
        .init_resource::<ViewportInset>()
        .init_resource::<PanelWidth>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
        .add_plugins(EventListenerPlugin::<Clicked>::default())
        .add_plugins(QuillPlugin)
        .add_systems(Startup, (setup, setup_view_root))
        .add_event::<Clicked>()
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                rotate,
                update_viewport_inset,
                update_camera_viewport,
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
    static ref STYLE_VSPLITTER: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#181818").unwrap()))
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Center)
        .display(ui::Display::Flex)
        .width(9)
        .selector(".drag", |ss| ss
            .background_color(Some(Color::hex("#080808").unwrap())))));
    static ref STYLE_VSPLITTER_INNER: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#282828").unwrap()))
        .display(ui::Display::Flex)
        .width(5)
        .height(ui::Val::Percent(30.))
        .pointer_events(PointerEvents::None)
        .selector(":hover > &", |ss| ss
            .background_color(Some(Color::hex("#383838").unwrap())))
        .selector(".drag > &", |ss| ss
            .background_color(Some(Color::hex("#484848").unwrap())))));
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
        .selector(":hover", |ss| ss
            .border_color(Some(Color::hex("#444").unwrap()))
            .background_color(Some(Color::hex("#2F2F2F").unwrap())))
        .selector(":hover.pressed", |ss| ss
            .background_color(Some(Color::hex("#484848").unwrap())))));
    static ref STYLE_VIEWPORT: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss.flex_grow(1.)));
}

const DEFAULT_FOV: f32 = 0.69; // 40 degrees
const X_EXTENT: f32 = 14.5;
const CLS_DRAG: &str = "drag";
const CLS_PRESSED: &str = "pressed";

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

/// Marker which identifies the primary camera.
#[derive(Component)]
pub struct PrimaryCamera;

/// Used to create margins around the viewport so that side panels don't overwrite the 3d scene.
#[derive(Default, Resource, PartialEq)]
pub struct ViewportInset {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

/// A marker component for that identifies which element contains the 3d view. The
/// `update_viewport_inset` system measures the on-screen position of the UiNode that this
/// component is attached to, and updates the screen position of the 3D view to match it.
#[derive(Component, Clone)]
pub struct ViewportInsetElement;

#[derive(Resource)]
pub struct PanelWidth(pub i32);

impl Default for PanelWidth {
    fn default() -> Self {
        Self(160)
    }
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn(ViewHandle::new(ui_main, ()));
}

fn ui_main(mut cx: Cx) -> impl View {
    let width = cx.use_resource::<PanelWidth>();
    Element::new().styled(STYLE_MAIN.clone()).children((
        Element::new()
            .styled((
                STYLE_ASIDE.clone(),
                Arc::new(StyleSet::build(|b| b.width(width.0))),
            ))
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
            ))
            .once(|entity, world| {
                let mut e = world.entity_mut(entity);
                e.insert((On::<Clicked>::run(|ev: Res<ListenerInput<Clicked>>| {
                    println!("Clicked {}", ev.id);
                }),));
            }),
        v_splitter,
        Element::new()
            .styled(STYLE_VIEWPORT.clone())
            .insert(ViewportInsetElement {}),
    ))
}

fn v_splitter(_cx: Cx) -> impl View {
    Element::new()
        .children(Element::new().styled(STYLE_VSPLITTER_INNER.clone()))
        .once(|entity, world| {
            let mut e = world.entity_mut(entity);
            e.insert((
                On::<Pointer<DragStart>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    classes.add_class(CLS_DRAG)
                }),
                On::<Pointer<DragEnd>>::listener_component_mut::<ElementClasses>(|_, classes| {
                    classes.remove_class(CLS_DRAG)
                }),
                On::<Pointer<Drag>>::run(
                    |ev: Listener<Pointer<Drag>>, mut res: ResMut<PanelWidth>| {
                        res.0 += ev.delta.x as i32;
                    },
                ),
                On::<Pointer<PointerCancel>>::listener_component_mut::<ElementClasses>(
                    |_, classes| {
                        println!("Cancel");
                        classes.remove_class(CLS_DRAG)
                    },
                ),
            ));
        })
        .styled(STYLE_VSPLITTER.clone())
}

#[derive(Clone, PartialEq)]
struct ButtonProps<V: View> {
    id: &'static str,
    children: V,
}

#[derive(Clone, Event, EntityEvent)]
struct Clicked {
    #[target] // Marks the field of the event that specifies the target entity
    target: Entity,
    id: &'static str,
}

fn button<V: View + Clone>(cx: Cx<ButtonProps<V>>) -> impl View {
    // Needs to be a local variable so that it can be captured in the event handler.
    let id = cx.props.id;
    Element::new()
        .children(cx.props.children.clone())
        .once(move |entity, world| {
            let mut e = world.entity_mut(entity);
            e.insert((
                On::<Pointer<Click>>::run(
                    move |events: Res<ListenerInput<Pointer<Click>>>,
                          mut ev: EventWriter<Clicked>| {
                        ev.send(Clicked {
                            target: events.target,
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
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        meshes.add(shape::Cube::default().into()),
        meshes.add(shape::Box::default().into()),
        meshes.add(shape::Capsule::default().into()),
        meshes.add(shape::Torus::default().into()),
        meshes.add(shape::Cylinder::default().into()),
        meshes.add(shape::Icosphere::default().try_into().unwrap()),
        meshes.add(shape::UVSphere::default().into()),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    0.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

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

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6., 12.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        PrimaryCamera,
        UiCameraConfig { show_ui: false },
    ));
}

pub fn update_viewport_inset(
    windows: Query<&Window>,
    query: Query<(&Node, &GlobalTransform), With<ViewportInsetElement>>,
    mut viewport_inset: ResMut<ViewportInset>,
) {
    let mut inset = ViewportInset::default();
    match query.get_single() {
        Ok((node, transform)) => {
            let position = transform.translation();
            let ui_position = position.truncate();
            let extents = node.size() / 2.0;
            let min = ui_position - extents;
            let max = ui_position + extents;

            let window = windows.single();
            let ww = window.resolution.physical_width() as f32;
            let wh = window.resolution.physical_height() as f32;
            let sf = window.resolution.scale_factor() as f32;

            inset.left = min.x;
            inset.top = min.y;
            inset.right = ww / sf - max.x;
            inset.bottom = wh / sf - max.y;
        }
        Err(_) => {
            if query.iter().count() > 1 {
                error!("Multiple ViewportInsetControllers!");
            }
        }
    }

    if inset != *viewport_inset {
        *viewport_inset.as_mut() = inset;
    }
}

/// Update the camera viewport and fov properties based on the window size and the viewport
/// margins.
pub fn update_camera_viewport(
    viewport_inset: Res<ViewportInset>,
    windows: Query<&Window>,
    mut camera_query: Query<(&mut Camera, &mut Projection), With<PrimaryCamera>>,
) {
    let window = windows.single();
    let ww = window.resolution.physical_width() as f32;
    let wh = window.resolution.physical_height() as f32;
    let sf = window.resolution.scale_factor() as f32;
    let left = viewport_inset.left * sf;
    let right = viewport_inset.right * sf;
    let top = viewport_inset.top * sf;
    let bottom = viewport_inset.bottom * sf;
    let vw = (ww - left - right).max(1.);
    let vh = (wh - top - bottom).max(1.);

    let (mut camera, mut projection) = camera_query.single_mut();
    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(left as u32, top as u32),
        physical_size: UVec2::new(vw as u32, vh as u32),
        ..default()
    });

    if let Projection::Perspective(ref mut perspective) = *projection {
        let aspect = vw / vh;
        perspective.aspect_ratio = aspect;
        perspective.fov = f32::min(DEFAULT_FOV, DEFAULT_FOV * 2. / aspect);
        perspective.near = 0.5;
        perspective.far = 100.;
    }
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
