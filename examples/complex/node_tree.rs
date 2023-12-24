use bevy::{asset::AssetPath, prelude::*, ui};
use bevy_grackle::hooks::{EnterExitApi, EnterExitState};
use bevy_mod_picking::prelude::*;
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::{
    collapse::{collapse, CollapseProps},
    disclosure::{disclosure_triangle, DisclosureTriangleProps, ToggleExpand},
    scrollview::{scroll_view, ScrollViewProps},
};

pub struct NodeTreePlugin;

impl Plugin for NodeTreePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RootEntityList>()
            .init_resource::<SelectedEntity>()
            .add_systems(Update, (update_root_entities, update_node_entities));
    }
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
pub struct EntityListNode {
    entity: Entity,
}

impl PartialOrd for EntityListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.entity.partial_cmp(&other.entity)
    }
}

#[derive(Resource, Default)]
pub struct RootEntityList(pub Vec<EntityListNode>);

#[derive(Resource, Default)]
pub struct SelectedEntity(pub Option<Entity>);

#[derive(Component)]
pub struct NodeInfo {
    entity: Entity,
    children: Vec<Entity>,
}

#[dynamic]
static STYLE_TREE: StyleHandle = StyleHandle::build(|ss| {
    ss.border(1)
        .border_color("#080808")
        .background_color("#171717")
        .flex_grow(1.)
        .padding(2)
});

#[dynamic]
static STYLE_TREE_INNER: StyleHandle = StyleHandle::build(|ss| {
    ss.flex_direction(ui::FlexDirection::Column)
        .height(ui::Val::Auto)
});

#[dynamic]
static STYLE_CONTENT: StyleHandle = StyleHandle::build(|ss| {
    ss.flex_direction(ui::FlexDirection::Column)
        .align_items(ui::AlignItems::Stretch)
});

#[dynamic]
static STYLE_TREE_NODE: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .align_items(ui::AlignItems::Stretch)
});

#[dynamic]
static STYLE_TREE_NODE_HEADER: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Start)
        .height(24)
        .padding(ui::UiRect::horizontal(ui::Val::Px(4.)))
        .padding_left(16)
        .selector(":hover", |ss| ss.background_color("#222"))
        .selector(".selected", |ss| ss.background_color("044"))
        .selector(".expandable", |ss| ss.padding_left(0))
        .color(Color::RED)
});

#[dynamic]
static STYLE_TREE_NODE_TITLE: StyleHandle = StyleHandle::build(|ss| {
    ss.font_size(16.)
        .font(Some(AssetPath::from("fonts/Exo_2/static/Exo2-Medium.ttf")))
});

#[dynamic]
static STYLE_TREE_NODE_NAME: StyleHandle = StyleHandle::build(|ss| {
    ss.font_size(16.)
        .font(Some(AssetPath::from("fonts/Exo_2/static/Exo2-Medium.ttf")))
        .margin_left(4)
});

#[dynamic]
static STYLE_TREE_NODE_CHILDREN: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .flex_grow(1.)
        .align_items(ui::AlignItems::Stretch)
        .margin_left(16)
});

pub fn node_tree(cx: Cx) -> impl View {
    let roots = cx.use_resource::<RootEntityList>();
    scroll_view.bind(ScrollViewProps {
        children: ViewParam::new(Element::new().styled(STYLE_TREE_INNER.clone()).children(
            For::keyed(&roots.0, |e| e.entity, |e| node_item.bind(e.clone())),
        )),
        scroll_enable_x: true,
        scroll_enable_y: true,
        style: STYLE_TREE.clone(),
        content_style: STYLE_CONTENT.clone(),
    })
}

pub fn node_item(mut cx: Cx<EntityListNode>) -> impl View {
    let expanded = cx.create_atom_init(|| false);
    cx.use_effect(
        |mut ve| {
            ve.insert(NodeInfo {
                entity: cx.props.entity,
                children: Vec::new(),
            });
        },
        cx.props.entity,
    );
    let info = cx.use_view_component::<NodeInfo>();
    let children = match info {
        Some(inf) => inf.children.clone(),
        None => Vec::new(),
    };
    let entity = cx.props.entity;
    let state = cx.use_enter_exit(cx.read_atom(expanded), 0.3);
    let selected = cx.use_resource::<SelectedEntity>();
    let name = cx.use_component_untracked::<Name>(entity);
    Element::new().styled(STYLE_TREE_NODE.clone()).children((
        Element::new()
            .styled(STYLE_TREE_NODE_HEADER.clone())
            .class_names((
                "selected".if_true(selected.0 == Some(cx.props.entity)),
                "expandable".if_true(children.len() > 0),
            ))
            .with_memo(
                move |mut e| {
                    e.insert((
                        On::<Pointer<Click>>::run(move |mut selected: ResMut<SelectedEntity>| {
                            selected.0 = Some(entity);
                        }),
                        On::<ToggleExpand>::run(
                            move |mut ev: ListenerMut<ToggleExpand>, mut atoms: AtomStore| {
                                ev.stop_propagation();
                                atoms.set(expanded, ev.value);
                            },
                        ),
                    ));
                },
                (),
            )
            .children((
                If::new(
                    children.len() > 0,
                    disclosure_triangle.bind(DisclosureTriangleProps {
                        expanded: cx.read_atom(expanded),
                    }),
                    (),
                ),
                format!("{:?}", cx.props.entity).styled(STYLE_TREE_NODE_TITLE.clone()),
                If::new(
                    name.is_some(),
                    name.map_or_else(|| "".to_string(), |n| n.to_string())
                        .styled(STYLE_TREE_NODE_NAME.clone()),
                    (),
                ),
                node_desc.bind(cx.props.clone()),
            )),
        If::new(
            state != EnterExitState::Exited,
            collapse.bind(CollapseProps {
                expanded: state == EnterExitState::Entering || state == EnterExitState::Entered,
                style: STYLE_TREE_NODE_CHILDREN.clone(),
                children: ViewParam::new(For::keyed(
                    &children,
                    |e| e.clone(),
                    |e| node_item.bind(EntityListNode { entity: e.clone() }),
                )),
            }),
            (),
        ),
    ))
}

pub fn node_desc(cx: Cx<EntityListNode>) -> impl View {
    let is_text = cx.use_component::<Text>(cx.props.entity);
    let is_mesh = cx.use_component::<Handle<Mesh>>(cx.props.entity);
    let is_camera = cx.use_component::<Camera>(cx.props.entity);
    let is_point_light = cx.use_component::<PointLight>(cx.props.entity);
    Fragment::new((
        If::new(is_mesh.is_some(), " Mesh", ()),
        If::new(is_text.is_some(), " Text", ()),
        If::new(is_camera.is_some(), " Camera", ()),
        If::new(is_point_light.is_some(), " PointLight", ()),
    ))
}

fn update_root_entities(
    query: Query<Entity, (Without<Parent>, Without<AtomCell>)>,
    mut roots: ResMut<RootEntityList>,
) {
    let mut sorted: Vec<EntityListNode> = Vec::with_capacity(query.iter().len());
    for entity in query.iter() {
        let node = EntityListNode { entity };
        sorted.push(node);
    }
    sorted.sort();

    if sorted != roots.0 {
        roots.0 = sorted;
    }
}

fn update_node_entities(
    mut query: Query<&mut NodeInfo>,
    query_children: Query<&Children, Without<AtomCell>>,
) {
    for mut node in query.iter_mut() {
        if let Ok(children) = query_children.get(node.entity) {
            let child_vec = children.to_vec();
            if node.children != child_vec {
                node.children = child_vec;
            }
        } else if node.children.len() > 0 {
            node.children.clear();
        }
    }
}
