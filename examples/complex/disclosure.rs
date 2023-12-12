use std::f32::consts::PI;

use bevy::{asset::AssetPath, prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_quill::prelude::*;
use static_init::dynamic;

pub struct DisclosureTrianglePlugin;

impl Plugin for DisclosureTrianglePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EventListenerPlugin::<ToggleExpand>::default())
            .add_event::<ToggleExpand>();
    }
}

#[dynamic]
static STYLE_DISCLOSURE_TRIANGLE: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .height(16)
        .width(16)
        .transition(&vec![Transition {
            property: TransitionProperty::Transform,
            duration: 0.3,
            timing: timing::EASE_IN_OUT,
            ..default()
        }])
        .selector(".expanded", |ss| ss.rotation(PI / 2.))
});

#[dynamic]
static STYLE_ICON: StyleHandle = StyleHandle::build(|ss| {
    ss.height(9)
        .width(9)
        .background_image(Some(AssetPath::from("arrow-right.png")))
        .background_color("#555")
        .selector(":hover > &", |ss| ss.background_color("#888"))
});

#[derive(Clone, PartialEq)]
pub struct DisclosureTriangleProps {
    pub expanded: bool,
}

#[derive(Clone, Event, EntityEvent)]
pub struct ToggleExpand {
    #[target]
    pub target: Entity,
    pub value: bool,
}

pub fn disclosure_triangle(cx: Cx<DisclosureTriangleProps>) -> impl View {
    let expanded = cx.props.expanded;
    Element::new()
        .with_memo(
            move |mut e| {
                e.insert((On::<Pointer<Click>>::run(
                    move |ev: Listener<Pointer<Click>>, mut writer: EventWriter<ToggleExpand>| {
                        writer.send(ToggleExpand {
                            target: ev.target,
                            value: !expanded,
                        });
                    },
                ),));
            },
            expanded,
        )
        .class_names("expanded".if_true(cx.props.expanded))
        .styled(STYLE_DISCLOSURE_TRIANGLE.clone())
        .children(Element::new().styled(STYLE_ICON.clone()))
}
