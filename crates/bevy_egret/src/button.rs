use bevy::prelude::*;
use bevy_mod_picking::{events::PointerCancel, prelude::*};
use bevy_quill::prelude::*;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EventListenerPlugin::<Clicked>::default())
            .add_event::<Clicked>();
    }
}

const CLS_PRESSED: &str = "pressed";

#[derive(Clone, PartialEq, Default)]
pub struct ButtonProps<'a, V: View + Clone, S: StyleTuple = (), C: ClassNames<'a> = ()> {
    pub id: &'static str,
    pub children: V,
    pub style: S,
    pub class_names: C,
    pub marker: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, Event, EntityEvent)]
pub struct Clicked {
    #[target]
    pub target: Entity,
    pub id: &'static str,
}

pub fn button<'a, V: View + Clone, S: StyleTuple, C: ClassNames<'a>>(
    mut cx: Cx<ButtonProps<'a, V, S, C>>,
) -> impl View {
    let is_dragging = cx.create_atom_init::<bool>(|| false);
    // Needs to be a local variable so that it can be captured in the event handler.
    let id = cx.props.id;
    Element::new()
        .class_names((
            cx.props.class_names.clone(),
            CLS_PRESSED.if_true(cx.read_atom(is_dragging)),
        ))
        .insert((
            On::<Pointer<Click>>::run(
                move |ev: Listener<Pointer<Click>>, mut writer: EventWriter<Clicked>| {
                    writer.send(Clicked {
                        target: ev.target,
                        id,
                    });
                },
            ),
            On::<Pointer<DragStart>>::run(move |mut atoms: AtomStore| {
                atoms.set(is_dragging, true);
            }),
            On::<Pointer<DragEnd>>::run(move |mut atoms: AtomStore| {
                atoms.set(is_dragging, false);
            }),
            On::<Pointer<PointerCancel>>::run(move |mut atoms: AtomStore| {
                println!("Cancel");
                atoms.set(is_dragging, false);
            }),
        ))
        .styled(cx.props.style.clone())
        .children(cx.props.children.clone())
}
