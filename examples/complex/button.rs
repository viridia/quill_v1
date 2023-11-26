use bevy::{prelude::*, ui};
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref STYLE_BUTTON: StyleHandle = StyleHandle::build(|ss| ss
        .background_color("#282828")
        .border_color("#383838")
        .border(1)
        .display(ui::Display::Flex)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .min_height(32)
        .padding_left(8)
        .padding_right(8)
        .selector(".pressed", |ss| ss.background_color("#404040"))
        .selector(":hover", |ss| ss
            .border_color("#444")
            .background_color("#2F2F2F"))
        .selector(":hover.pressed", |ss| ss.background_color("#484848")));
}

const CLS_PRESSED: &str = "pressed";

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

pub fn button<V: View + Clone>(cx: Cx<ButtonProps<V>>) -> impl View {
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
