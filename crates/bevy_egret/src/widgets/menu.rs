use bevy::{
    a11y::{
        accesskit::{HasPopup, NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};
use bevy_mod_picking::prelude::*;
use bevy_quill::prelude::*;
use bevy_tabindex::TabIndex;

use crate::{
    hooks::{EnterExitApi, EnterExitState},
    Clicked, MenuAction, MenuEvent,
};

const CLS_OPEN: &str = "open";

pub const MENU_ANCHOR: ScopedValueKey<Entity> = ScopedValueKey::new("menu-anchor");

#[derive(Clone, PartialEq)]
pub struct MenuButtonProps<
    'a,
    V: View + Clone,
    VP: View + Clone,
    S: StyleTuple = (),
    C: ClassNames<'a> = (),
> {
    pub anchor: Entity,
    pub children: V,
    pub popup: VP,
    pub style: S,
    pub class_names: C,
    pub disabled: bool,
    pub marker: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Default)]
pub struct MenuPopupProps<'a, V: View + Clone, S: StyleTuple = (), C: ClassNames<'a> = ()> {
    pub children: V,
    pub style: S,
    pub class_names: C,
    pub marker: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Default)]
pub struct MenuItemProps<V: View + Clone, S: StyleTuple = ()> {
    pub id: &'static str,
    pub style: S,
    pub label: V,
    pub checked: bool,
    pub disabled: bool,
    // icon
}

pub fn menu_button<'a, V: View + Clone, VP: View + Clone, S: StyleTuple, C: ClassNames<'a>>(
    mut cx: Cx<MenuButtonProps<'a, V, VP, S, C>>,
) -> impl View {
    let id_anchor = cx.props.anchor;
    let is_open = cx.create_atom_init::<bool>(|| false);
    let state = cx.use_enter_exit(cx.read_atom(is_open), 0.3);
    cx.define_scoped_value(MENU_ANCHOR, id_anchor);
    RefElement::new(cx.props.anchor)
        .named("menu-button")
        .class_names((
            cx.props.class_names.clone(),
            CLS_OPEN.if_true(cx.read_atom(is_open)),
        ))
        .insert((
            TabIndex(0),
            AccessibilityNode::from({
                let mut builder = NodeBuilder::new(Role::Button);
                builder.set_has_popup(HasPopup::Menu);
                builder.set_expanded(cx.read_atom(is_open));
                builder
            }),
            On::<Pointer<Click>>::run(
                move |ev: Listener<Pointer<Click>>,
                      mut writer: EventWriter<MenuEvent>,
                      atoms: AtomStore| {
                    let open = atoms.get(is_open);
                    writer.send(MenuEvent {
                        target: ev.target,
                        action: if open {
                            MenuAction::Close
                        } else {
                            MenuAction::Open
                        },
                    });
                },
            ),
            On::<MenuEvent>::run(move |ev: Listener<MenuEvent>, mut atoms: AtomStore| {
                match ev.action {
                    MenuAction::Open => {
                        atoms.set(is_open, true);
                    }
                    MenuAction::Close => {
                        atoms.set(is_open, false);
                    }
                    _ => {}
                }
            }),
        ))
        .styled(cx.props.style.clone())
        .children((
            cx.props.children.clone(),
            If::new(
                state != EnterExitState::Exited,
                Portal::new().children(
                    Element::new()
                        .class_names(state.as_class_name())
                        .insert((
                            On::<Pointer<Down>>::run(move |mut writer: EventWriter<MenuEvent>| {
                                writer.send(MenuEvent {
                                    action: MenuAction::Close,
                                    target: id_anchor,
                                });
                            }),
                            Style {
                                left: Val::Px(0.),
                                right: Val::Px(0.),
                                top: Val::Px(0.),
                                bottom: Val::Px(0.),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            ZIndex::Global(100),
                        ))
                        .children(cx.props.popup.clone()),
                ),
                (),
            ),
        ))
}

pub fn menu_popup<'a, V: View + Clone, S: StyleTuple, C: ClassNames<'a>>(
    mut cx: Cx<MenuPopupProps<'a, V, S, C>>,
) -> impl View {
    let is_open = cx.create_atom_init::<bool>(|| false);
    // Needs to be a local variable so that it can be captured in the event handler.
    // let id = cx.props.id;
    Element::new()
        .named("menu-popup")
        .insert((On::<Pointer<Down>>::run(
            move |mut ev: ListenerMut<Pointer<Down>>| {
                ev.stop_propagation();
            },
        ),))
        .class_names((
            cx.props.class_names.clone(),
            CLS_OPEN.if_true(cx.read_atom(is_open)),
        ))
        .styled(cx.props.style.clone())
        .children(cx.props.children.clone())
}

pub fn menu_item<V: View + Clone, S: StyleTuple>(mut cx: Cx<MenuItemProps<V, S>>) -> impl View {
    let _is_selected = cx.create_atom_init::<bool>(|| false);
    // Needs to be a local variable so that it can be captured in the event handler.
    let id = cx.props.id;
    let anchor = cx.get_scoped_value(MENU_ANCHOR).unwrap();
    Element::new()
        .named("menu-item")
        // .class_names((
        //     cx.props.class_names.clone(),
        //     CLS_PRESSED.if_true(cx.read_atom(is_selected)),
        // ))
        .insert((On::<Pointer<Click>>::run(
            move |mut writer: EventWriter<Clicked>, mut writer2: EventWriter<MenuEvent>| {
                writer.send(Clicked { target: anchor, id });
                writer2.send(MenuEvent {
                    action: MenuAction::Close,
                    target: anchor,
                });
            },
        ),))
        .styled(cx.props.style.clone())
        .children(cx.props.label.clone())
}
