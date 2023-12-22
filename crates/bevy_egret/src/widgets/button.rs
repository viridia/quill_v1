use bevy::prelude::*;
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::prelude::*;

use crate::Clicked;

const CLS_PRESSED: &str = "pressed";
const CLS_DISABLED: &str = "disabled";

#[derive(Clone, PartialEq, Default)]
pub struct ButtonProps<'a, V: View + Clone, S: StyleTuple = (), C: ClassNames<'a> = ()> {
    pub id: &'static str,
    pub children: V,
    pub style: S,
    pub class_names: C,
    pub disabled: bool,
    pub marker: std::marker::PhantomData<&'a ()>,
}

pub fn button<'a, V: View + Clone, S: StyleTuple, C: ClassNames<'a>>(
    mut cx: Cx<ButtonProps<'a, V, S, C>>,
) -> impl View {
    let is_pressed = cx.create_atom_init::<bool>(|| false);
    // Needs to be a local variable so that it can be captured in the event handler.
    let id = cx.props.id;
    let disabled = cx.props.disabled;
    Element::new()
        .class_names((
            cx.props.class_names.clone(),
            CLS_PRESSED.if_true(cx.read_atom(is_pressed)),
            CLS_DISABLED.if_true(disabled),
        ))
        .insert((
            On::<Pointer<Click>>::run(
                move |ev: Listener<Pointer<Click>>, mut writer: EventWriter<Clicked>| {
                    if !disabled {
                        writer.send(Clicked {
                            target: ev.target,
                            id,
                        });
                    }
                },
            ),
            On::<Pointer<DragStart>>::run(move |mut atoms: AtomStore| {
                if !disabled {
                    atoms.set(is_pressed, true);
                }
            }),
            On::<Pointer<DragEnd>>::run(move |mut atoms: AtomStore| {
                if !disabled {
                    atoms.set(is_pressed, false);
                }
            }),
            On::<Pointer<PointerCancel>>::run(move |mut atoms: AtomStore| {
                if !disabled {
                    atoms.set(is_pressed, false);
                }
            }),
        ))
        .styled(cx.props.style.clone())
        .children(cx.props.children.clone())
}
