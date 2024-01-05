use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_quill::prelude::*;

use crate::{
    hooks::{EnterExitApi, EnterExitState},
    MenuAction, MenuEvent,
};

const CLS_OPEN: &str = "open";

#[derive(Clone, PartialEq, Default)]
pub struct MenuButtonProps<
    'a,
    V: View + Clone,
    VI: View + Clone,
    S: StyleTuple = (),
    C: ClassNames<'a> = (),
> {
    pub children: V,
    pub popup: VI,
    pub style: S,
    pub class_names: C,
    pub disabled: bool,
    pub marker: std::marker::PhantomData<&'a ()>,
}

#[derive(PartialEq, Default)]
pub struct MenuPopupProps<'a, V: View + Clone, S: StyleTuple = (), C: ClassNames<'a> = ()> {
    pub children: V,
    pub style: S,
    pub class_names: C,
    pub marker: std::marker::PhantomData<&'a ()>,
}

#[derive(PartialEq, Default)]
pub struct MenuItemProps<V: View + Clone, S: StyleTuple = ()> {
    pub id: &'static str,
    pub style: S,
    pub label: V,
    pub checked: bool,
    pub disabled: bool,
    // icon
}

pub fn menu_button<'a, V: View + Clone, VI: View + Clone, S: StyleTuple, C: ClassNames<'a>>(
    mut cx: Cx<MenuButtonProps<'a, V, VI, S, C>>,
) -> impl View {
    let id_anchor = cx.create_entity();
    let is_open = cx.create_atom_init::<bool>(|| false);
    let state = cx.use_enter_exit(cx.read_atom(is_open), 0.3);
    RefElement::new(id_anchor)
        .named("menu-button")
        .class_names((
            cx.props.class_names.clone(),
            CLS_OPEN.if_true(cx.read_atom(is_open)),
        ))
        .insert((
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
                Portal::new().children((Element::new()
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
                    .children(cx.props.popup.clone()),)),
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
        .class_names((
            cx.props.class_names.clone(),
            CLS_OPEN.if_true(cx.read_atom(is_open)),
        ))
        .styled(cx.props.style.clone())
        .children(cx.props.children.clone())
}

pub fn menu_item<'a, V: View + Clone, S: StyleTuple>(mut cx: Cx<MenuItemProps<V, S>>) -> impl View {
    let is_selected = cx.create_atom_init::<bool>(|| false);
    // Needs to be a local variable so that it can be captured in the event handler.
    // let id = cx.props.id;
    Element::new()
        .named("menu-item")
        // .class_names((
        //     cx.props.class_names.clone(),
        //     CLS_PRESSED.if_true(cx.read_atom(is_selected)),
        // ))
        // .insert((On::<Pointer<Click>>::run(
        //     move |ev: Listener<Pointer<Click>>, mut writer: EventWriter<Clicked>| {
        //         writer.send(Clicked {
        //             target: ev.target,
        //             id,
        //         });
        //     },
        // ),))
        .styled(cx.props.style.clone())
        .children(cx.props.label.clone())
}
