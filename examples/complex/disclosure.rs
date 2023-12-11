use std::f32::consts::PI;

use bevy::{asset::AssetPath, prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_quill::prelude::*;
use static_init::dynamic;

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
        .selector(":hover", |ss| ss.rotation(PI / 2.))
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
    pub id: &'static str,
}

#[derive(Clone, Event, EntityEvent)]
pub struct Clicked {
    #[target]
    pub target: Entity,
    pub id: &'static str,
}

pub fn disclosure_triangle(cx: Cx<DisclosureTriangleProps>) -> impl View {
    // Needs to be a local variable so that it can be captured in the event handler.
    let id = cx.props.id;
    Element::new()
        .with_memo(
            move |mut e| {
                e.insert((On::<Pointer<Click>>::run(
                    move |ev: Listener<Pointer<Click>>, mut writer: EventWriter<Clicked>| {
                        writer.send(Clicked {
                            target: ev.target,
                            id,
                        });
                    },
                ),));
            },
            (),
        )
        .styled(STYLE_DISCLOSURE_TRIANGLE.clone())
        .children(Element::new().styled(STYLE_ICON.clone()))
}
